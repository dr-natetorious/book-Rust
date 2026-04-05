use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use book_rust::ch02::feed::{scan_messages, FeedError, MessageView};

struct CountingAllocator;

static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

#[global_allocator]
static GLOBAL_ALLOCATOR: CountingAllocator = CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::SeqCst);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

fn load_fixture_bytes() -> Vec<u8> {
    let hex = include_str!("../fixtures/ch02_multi_message.hex").trim();
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
fn parses_borrowed_message_views_from_fixture() {
    let fixture = load_fixture_bytes();

    let mut add_count = 0;
    let mut execute_count = 0;
    let mut cancel_count = 0;
    let mut stock_ptr = std::ptr::null();

    scan_messages::<FeedError, _>(&fixture, |message| {
        match message {
            MessageView::Add(add) => {
                add_count += 1;
                stock_ptr = add.stock.as_ptr();
                assert_eq!(add.order_ref, 42);
                assert_eq!(add.stock, b"GOOG    ");
            }
            MessageView::Execute(execute) => {
                execute_count += 1;
                assert_eq!(execute.executed_shares, 400);
            }
            MessageView::Cancel(cancel) => {
                cancel_count += 1;
                assert_eq!(cancel.canceled_shares, 600);
            }
        }

        Ok(())
    })
    .expect("fixture should parse");

    assert_eq!(add_count, 1);
    assert_eq!(execute_count, 1);
    assert_eq!(cancel_count, 1);
    assert_eq!(stock_ptr, fixture[17..25].as_ptr());
}

#[test]
fn rejects_wrong_record_length() {
    let bad = vec![b'E', 13, 0, 1, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 1];
    let err = scan_messages::<FeedError, _>(&bad, |_| Ok(())).expect_err("invalid length must fail");

    assert!(matches!(
        err,
        FeedError::InvalidRecordLength {
            msg_type: b'E',
            expected: 14,
            actual: 13,
        }
    ));
}

#[test]
fn scans_fixture_without_heap_allocations() {
    let fixture = load_fixture_bytes();
    let mut messages_seen = 0_u32;

    ALLOCATIONS.store(0, Ordering::SeqCst);
    let before = ALLOCATIONS.load(Ordering::SeqCst);

    scan_messages::<FeedError, _>(&fixture, |_| {
        messages_seen += 1;
        Ok(())
    })
    .expect("fixture should scan without allocating");

    let after = ALLOCATIONS.load(Ordering::SeqCst);
    assert_eq!(messages_seen, 3);
    assert_eq!(after - before, 0, "scan_messages allocated on the hot path");
}