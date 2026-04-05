use std::env;
use std::fs::{remove_file, File};
use std::io::Write;

use book_rust::ch04::stream::{FileFrameError, FileFrameIter, FrameIter, ParsedMessageIter};

fn load_fixture_bytes() -> Vec<u8> {
    let hex = include_str!("../fixtures/ch03_order_flow.hex").trim();
    assert_eq!(hex.len() % 2, 0, "fixture hex length must be even");

    let mut out = Vec::with_capacity(hex.len() / 2);
    let bytes = hex.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        let hi = bytes[index] as char;
        let lo = bytes[index + 1] as char;
        out.push(u8::from_str_radix(&format!("{hi}{lo}"), 16).expect("valid fixture hex"));
        index += 2;
    }

    out
}

#[test]
fn frame_iter_yields_one_frame_at_a_time() {
    let fixture = load_fixture_bytes();
    let mut iter = FrameIter::new(&fixture);

    let first = iter.next().expect("one frame").expect("valid frame");
    assert_eq!(first.offset, 0);
    assert_eq!(first.msg_type, b'A');
    assert_eq!(first.payload.len(), 27);

    let second = iter.next().expect("second frame").expect("valid frame");
    assert_eq!(second.offset, 29);
    assert_eq!(second.msg_type, b'A');
}

#[test]
fn parsed_message_iter_is_lazy_and_complete() {
    let fixture = load_fixture_bytes();
    let parsed = ParsedMessageIter::new(&fixture)
        .collect::<Result<Vec<_>, _>>()
        .expect("fixture should parse lazily");

    assert_eq!(parsed.len(), 5);
}

#[test]
fn file_frame_iter_reads_fixture_without_loading_everything_first() {
    let fixture = load_fixture_bytes();
    let mut path = env::temp_dir();
    path.push(format!("book-rust-ch04-{}.bin", std::process::id()));

    let mut file = File::create(&path).expect("temp file should open");
    file.write_all(&fixture).expect("fixture should write");
    drop(file);

    let reader = File::open(&path).expect("temp file should reopen");
    let count = FileFrameIter::new(reader)
        .collect::<Result<Vec<_>, _>>()
        .expect("file iterator should read frames")
        .len();

    remove_file(&path).ok();
    assert_eq!(count, 5);
}

#[test]
fn file_frame_iter_reports_truncated_header() {
    let bytes = [b'A'];
    let err = FileFrameIter::new(&bytes[..])
        .next()
        .expect("one result")
        .expect_err("truncated header should fail");

    assert!(matches!(
        err,
        FileFrameError::TruncatedHeader {
            offset: 0,
            actual: 1
        }
    ));
}