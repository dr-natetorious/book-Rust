@echo off
setlocal EnableExtensions

set "SCRIPT_DIR=%~dp0"
for %%I in ("%SCRIPT_DIR%..") do set "ROOT_DIR=%%~fI"
set "IMAGE_NAME=book-rust-toolchain"
set "HOST_OUT=%ROOT_DIR%\.out"
set "CONTAINER_OUT=/book/out"
set "CONTAINER_NAME=book-rust-build-%RANDOM%-%RANDOM%"

call :resolve_docker
if errorlevel 1 exit /b 1

if /I "%DOCKER_MODE%"=="windows" (
  call :check_daemon_windows
  if errorlevel 1 exit /b 1

  call :ensure_image_windows
  if errorlevel 1 exit /b 1

  call :prepare_host_out
  if errorlevel 1 exit /b 1

  call :build_and_copy_windows
  exit /b %ERRORLEVEL%
)

call :resolve_wsl_root
if errorlevel 1 exit /b 1

call :resolve_wsl_out
if errorlevel 1 exit /b 1

call :check_daemon_wsl
if errorlevel 1 exit /b 1

call :ensure_image_wsl
if errorlevel 1 exit /b 1

call :prepare_host_out
if errorlevel 1 exit /b 1

call :build_and_copy_wsl
exit /b %ERRORLEVEL%

:resolve_docker
docker --version >nul 2>&1
if not errorlevel 1 (
  set "DOCKER_MODE=windows"
  exit /b 0
)

wsl.exe sh -lc "command -v docker >/dev/null 2>&1"
if not errorlevel 1 (
  set "DOCKER_MODE=wsl"
  exit /b 0
)

echo Docker was not found on the Windows PATH or inside WSL2.>&2
echo Install Docker Desktop or a WSL2 Docker engine, then run this script again.>&2
exit /b 1

:resolve_wsl_root
set "WSL_ROOT="
for /f "usebackq delims=" %%I in (`wsl.exe wslpath -a "%ROOT_DIR%"`) do set "WSL_ROOT=%%I"
if defined WSL_ROOT exit /b 0

echo Failed to convert %ROOT_DIR% into a WSL path.>&2
exit /b 1

:resolve_wsl_out
set "WSL_OUT="
for /f "usebackq delims=" %%I in (`wsl.exe wslpath -a "%HOST_OUT%"`) do set "WSL_OUT=%%I"
if defined WSL_OUT exit /b 0

echo Failed to convert %HOST_OUT% into a WSL path.>&2
exit /b 1

:prepare_host_out
if exist "%HOST_OUT%" rmdir /s /q "%HOST_OUT%"
mkdir "%HOST_OUT%" >nul 2>&1
if exist "%HOST_OUT%" exit /b 0

echo Failed to create output directory %HOST_OUT%.>&2
exit /b 1

:check_daemon_windows
docker info >nul 2>&1
if not errorlevel 1 exit /b 0

echo Docker is on the Windows PATH, but the daemon is not reachable.>&2
echo Start Docker Desktop or fix the local Docker engine, then run this script again.>&2
exit /b 1

:check_daemon_wsl
wsl.exe sh -lc "docker info >/dev/null 2>&1"
if not errorlevel 1 exit /b 0

echo Docker is installed inside WSL2, but the current WSL user cannot access the daemon.>&2
echo Fix WSL Docker access, for example by enabling Docker Desktop WSL integration or adding your WSL user to the docker group, then rerun this script.>&2
exit /b 1

:ensure_image_windows
docker image inspect "%IMAGE_NAME%" >nul 2>&1
if not errorlevel 1 exit /b 0

echo Building Docker image %IMAGE_NAME%...
docker build -t "%IMAGE_NAME%" "%ROOT_DIR%"
exit /b %ERRORLEVEL%

:ensure_image_wsl
wsl.exe sh -lc "docker image inspect %IMAGE_NAME% >/dev/null 2>&1"
if not errorlevel 1 exit /b 0

echo Building Docker image %IMAGE_NAME% inside WSL2...
wsl.exe sh -lc "docker build -t %IMAGE_NAME% '%WSL_ROOT%'"
exit /b %ERRORLEVEL%

:build_and_copy_windows
echo Building book outputs with Docker on Windows...
docker create --name "%CONTAINER_NAME%" -e BUILD_DIR=%CONTAINER_OUT% -v "%ROOT_DIR%:/workspace" "%IMAGE_NAME%" build-book >nul
if errorlevel 1 exit /b 1

docker start -a "%CONTAINER_NAME%"
if errorlevel 1 goto :cleanup_windows_fail

docker cp "%CONTAINER_NAME%:%CONTAINER_OUT%/." "%HOST_OUT%"
if errorlevel 1 goto :cleanup_windows_fail

docker rm "%CONTAINER_NAME%" >nul 2>&1
echo Book artifacts copied to %HOST_OUT%
exit /b 0

:cleanup_windows_fail
docker rm "%CONTAINER_NAME%" >nul 2>&1
exit /b 1

:build_and_copy_wsl
echo Building book outputs with Docker inside WSL2...
wsl.exe sh -lc "docker create --name %CONTAINER_NAME% -e BUILD_DIR=%CONTAINER_OUT% -v '%WSL_ROOT%:/workspace' %IMAGE_NAME% build-book >/dev/null"
if errorlevel 1 exit /b 1

wsl.exe sh -lc "docker start -a %CONTAINER_NAME%"
if errorlevel 1 goto :cleanup_wsl_fail

wsl.exe sh -lc "docker cp %CONTAINER_NAME%:%CONTAINER_OUT%/. '%WSL_OUT%'"
if errorlevel 1 goto :cleanup_wsl_fail

wsl.exe sh -lc "docker rm %CONTAINER_NAME% >/dev/null 2>&1"
echo Book artifacts copied to %HOST_OUT%
exit /b 0

:cleanup_wsl_fail
wsl.exe sh -lc "docker rm %CONTAINER_NAME% >/dev/null 2>&1"
exit /b 1