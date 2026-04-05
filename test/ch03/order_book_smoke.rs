use book_rust::ch03::order_book::{
    BookError, BookEvent, ClosedReason, OrderBook, OrderState, OpenOrder, Side,
};

fn load_fixture_bytes() -> Vec<u8> {
    let hex = include_str!("../fixtures/ch03_order_flow.hex").trim();
    assert_eq!(hex.len() % 2, 0, "fixture hex length must be even");

    let mut out = Vec::with_capacity(hex.len() / 2);
    let bytes = hex.as_bytes();

    let mut index = 0;
    while index < bytes.len() {
        let hi = bytes[index] as char;
        let lo = bytes[index + 1] as char;
        let value = u8::from_str_radix(&format!("{hi}{lo}"), 16).expect("valid fixture hex");
        out.push(value);
        index += 2;
    }

    out
}

#[test]
fn replays_fixture_without_contradictory_state() {
    let fixture = load_fixture_bytes();
    let mut book = OrderBook::new();

    book.replay(&fixture).expect("fixture replay should succeed");

    assert_eq!(book.open_order_count(), 0);
    assert!(matches!(
        book.state(42),
        Some(OrderState::Closed(closed)) if closed.reason == ClosedReason::Cancelled
    ));
    assert!(matches!(
        book.state(77),
        Some(OrderState::Closed(closed)) if closed.reason == ClosedReason::Cancelled
    ));
}

#[test]
fn rejects_reduction_larger_than_remaining_shares() {
    let mut book = OrderBook::new();
    book.apply(BookEvent::Add(OpenOrder {
        order_ref: 99,
        stock: *b"AAPL    ",
        side: Side::Buy,
        remaining_shares: 500,
        price: 18_000,
    }))
    .expect("add should succeed");

    let err = book
        .apply(BookEvent::Execute {
            order_ref: 99,
            executed_shares: 600,
        })
        .expect_err("oversized execute should fail");

    assert!(matches!(
        err,
        BookError::ReductionExceedsRemaining {
            order_ref: 99,
            remaining: 500,
            requested: 600,
        }
    ));
}