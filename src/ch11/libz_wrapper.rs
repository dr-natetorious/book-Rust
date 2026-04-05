use std::fmt;

pub const Z_OK: i32 = 0;
pub const Z_DATA_ERROR: i32 = -3;
pub const Z_BUF_ERROR: i32 = -5;

pub type CUncompress = unsafe extern "C" fn(*mut u8, *mut usize, *const u8, usize) -> i32;

#[derive(Clone, Copy)]
pub struct LibzApi {
    uncompress: CUncompress,
}

impl LibzApi {
    pub fn new(uncompress: CUncompress) -> Self {
        Self { uncompress }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LibzError {
    BufferTooSmall,
    CorruptInput,
    Unknown(i32),
}

impl fmt::Display for LibzError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LibzError::BufferTooSmall => write!(f, "destination buffer is too small"),
            LibzError::CorruptInput => write!(f, "compressed input is corrupt"),
            LibzError::Unknown(code) => write!(f, "libz returned unknown status {code}"),
        }
    }
}

impl std::error::Error for LibzError {}

// tag::safe_decompress[]
pub fn safe_decompress(api: LibzApi, input: &[u8], output_len: usize) -> Result<Vec<u8>, LibzError> {
    let mut output = vec![0_u8; output_len];
    let mut actual_len = output.len();

    let status = unsafe {
        (api.uncompress)(
            output.as_mut_ptr(),
            &mut actual_len,
            input.as_ptr(),
            input.len(),
        )
    };

    match status {
        Z_OK => {
            output.truncate(actual_len);
            Ok(output)
        }
        Z_BUF_ERROR => Err(LibzError::BufferTooSmall),
        Z_DATA_ERROR => Err(LibzError::CorruptInput),
        code => Err(LibzError::Unknown(code)),
    }
}
// end::safe_decompress[]

// tag::ffi_contract[]
/// Mock contract for chapter tests:
/// input format is [payload_len, payload...].
/// Returns Z_DATA_ERROR if payload_len does not match provided bytes.
pub unsafe extern "C" fn mock_uncompress(
    dest: *mut u8,
    dest_len: *mut usize,
    src: *const u8,
    src_len: usize,
) -> i32 {
    if src_len == 0 {
        return Z_DATA_ERROR;
    }

    let source = unsafe { std::slice::from_raw_parts(src, src_len) };
    let expected = source[0] as usize;
    let payload = &source[1..];

    if payload.len() != expected {
        return Z_DATA_ERROR;
    }

    let available = unsafe { *dest_len };
    if available < payload.len() {
        return Z_BUF_ERROR;
    }

    unsafe {
        std::ptr::copy_nonoverlapping(payload.as_ptr(), dest, payload.len());
        *dest_len = payload.len();
    }

    Z_OK
}
// end::ffi_contract[]
