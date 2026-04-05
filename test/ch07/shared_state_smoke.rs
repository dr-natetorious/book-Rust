use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

use book_rust::ch07::shared::{
    apply_fill_local, apply_fill_shared, borrowed_view, LiveRiskState, LocalRiskHandle,
    SharedRiskHandle,
};

#[test]
fn borrowed_view_points_at_existing_state() {
    let symbol = *b"GOOG    ";
    let mut positions = HashMap::new();
    positions.insert(symbol, 1_200_i64);
    let state = LiveRiskState {
        positions,
        position_limit: 5_000,
    };

    let view = borrowed_view(&state, &symbol);
    let original = state.positions.get(&symbol).expect("position should exist");
    assert!(std::ptr::eq(view.current_position.expect("view should point"), original));
}

#[test]
fn refcell_reports_runtime_borrow_conflict() {
    let handle: LocalRiskHandle = Rc::new(RefCell::new(LiveRiskState::new(5_000)));
    let _borrow = handle.borrow_mut();

    let err = apply_fill_local(&handle, *b"GOOG    ", 10)
        .expect_err("borrow conflict should surface at runtime");
    let _ = err;
}

#[test]
fn arc_mutex_shares_updates_across_threads() {
    let handle: SharedRiskHandle = Arc::new(Mutex::new(LiveRiskState::new(5_000)));
    let left = Arc::clone(&handle);
    let right = Arc::clone(&handle);

    let t1 = thread::spawn(move || apply_fill_shared(&left, *b"GOOG    ", 400));
    let t2 = thread::spawn(move || apply_fill_shared(&right, *b"GOOG    ", 600));

    t1.join().expect("thread should join").expect("update should succeed");
    t2.join().expect("thread should join").expect("update should succeed");

    let locked = handle.lock().expect("state should lock");
    assert_eq!(locked.positions.get(b"GOOG    ").copied(), Some(1_000));
}