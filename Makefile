.PHONY: test test-ch1 build-html build-pdf build-epub build-all validate-includes docker-build docker-test-ch1

test:
	cargo test

test-ch1:
	cargo test --test integration_ch01

build-html:
	asciidoctor -r asciidoctor-diagram -a stylesheet=theme/web.css -D build/html chapters/book.adoc

build-pdf:
	asciidoctor-pdf -a pdf-theme=chapters/theme/pdf-theme.yml -D build/pdf chapters/book.adoc

build-epub:
	asciidoctor-epub3 -D build/epub chapters/book.adoc

build-all: build-html build-pdf build-epub

validate-includes:
	bash scripts/validate-includes.sh

docker-build:
	docker build -t book-rust-toolchain .

docker-test-ch1:
	docker run --rm -v "$(CURDIR):/workspace" book-rust-toolchain chapter 1
