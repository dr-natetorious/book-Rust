use book_rust::ch11::libz_wrapper::{mock_uncompress, safe_decompress, LibzApi, LibzError};

#[test]
fn safe_wrapper_decompresses_expected_payload() {
    let api = LibzApi::new(mock_uncompress);
    let compressed = vec![5, b'h', b'e', b'l', b'l', b'o'];

    let decoded = safe_decompress(api, &compressed, 16).expect("decode should succeed");
    assert_eq!(decoded, b"hello");
}

#[test]
fn safe_wrapper_reports_corrupt_input() {
    let api = LibzApi::new(mock_uncompress);
    let corrupt = vec![6, b'h', b'e', b'l', b'l', b'o'];

    let err = safe_decompress(api, &corrupt, 16).expect_err("corrupt payload should fail");
    assert_eq!(err, LibzError::CorruptInput);
}
