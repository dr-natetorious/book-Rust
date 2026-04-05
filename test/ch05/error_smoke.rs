use book_rust::ch03::order_book::{ClosedReason, OrderBook, OrderState};
use book_rust::ch05::errors::{replay_with_policy, CorruptionPolicy, ProcessingError, RejectedReason};

fn load_fixture_bytes() -> Vec<u8> {
    let hex = include_str!("../fixtures/ch05_corrupt_stream.hex").trim();
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
fn fail_fast_surfaces_the_offset_of_corruption() {
    let fixture = load_fixture_bytes();
    let mut book = OrderBook::new();

    let err = replay_with_policy(&fixture, &mut book, CorruptionPolicy::FailFast)
        .expect_err("fail-fast policy should stop on corruption");

    assert!(matches!(
        err,
        ProcessingError::Feed {
            offset: 29,
            source: book_rust::ch02::feed::FeedError::UnsupportedMessageType(b'Z')
        }
    ));
}

#[test]
fn skip_bad_records_keeps_valid_records_moving() {
    let fixture = load_fixture_bytes();
    let mut book = OrderBook::new();

    let report = replay_with_policy(&fixture, &mut book, CorruptionPolicy::SkipBadRecords)
        .expect("skip policy should continue");

    assert_eq!(report.applied, 2);
    assert_eq!(report.rejected.len(), 1);
    assert!(matches!(
        report.rejected[0].reason,
        RejectedReason::Feed(book_rust::ch02::feed::FeedError::UnsupportedMessageType(b'Z'))
    ));
    assert!(matches!(
        book.state(42),
        Some(OrderState::Closed(closed)) if closed.reason == ClosedReason::Cancelled
    ));
}