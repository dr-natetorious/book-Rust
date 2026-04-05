use book_rust::ch01::parser::{parse_records, ParseError, ADD_ORDER_TYPE};

fn load_fixture_bytes() -> Vec<u8> {
    let hex = include_str!("../fixtures/ch01_add_order.hex").trim();
    assert_eq!(hex.len() % 2, 0, "fixture hex length must be even");

    let mut out = Vec::with_capacity(hex.len() / 2);
    let bytes = hex.as_bytes();

    let mut i = 0;
    while i < bytes.len() {
        let hi = bytes[i] as char;
        let lo = bytes[i + 1] as char;
        let b = u8::from_str_radix(&format!("{hi}{lo}"), 16).expect("valid fixture hex");
        out.push(b);
        i += 2;
    }

    out
}

#[test]
fn parses_single_add_order_from_fixture() {
    let fixture = load_fixture_bytes();
    assert_eq!(fixture[0], ADD_ORDER_TYPE);

    let parsed = parse_records(&fixture).expect("fixture should parse");
    assert_eq!(parsed.len(), 1);

    let msg = &parsed[0];
    assert_eq!(msg.stock_locate, 1);
    assert_eq!(msg.order_ref, 42);
    assert_eq!(msg.side, b'B');
    assert_eq!(msg.shares, 1000);
    assert_eq!(&msg.stock, b"GOOG    ");
    assert_eq!(msg.price, 40_000);
}

#[test]
fn rejects_unknown_message_type() {
    let bad = vec![b'Z', 0];
    let err = parse_records(&bad).expect_err("unknown message type should fail");
    assert!(matches!(err, ParseError::UnsupportedMessageType(b'Z')));
}

#[test]
fn rejects_truncated_payload() {
    let bad = vec![ADD_ORDER_TYPE, 27, 0, 1];
    let err = parse_records(&bad).expect_err("truncated payload should fail");
    assert!(matches!(
        err,
        ParseError::TruncatedRecord {
            expected: 27,
            actual: 2
        }
    ));
}
