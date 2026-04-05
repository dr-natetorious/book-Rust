use std::cell::{BorrowMutError, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveRiskState {
    pub positions: HashMap<[u8; 8], i64>,
    pub position_limit: i64,
}

impl LiveRiskState {
    pub fn new(position_limit: i64) -> Self {
        Self {
            positions: HashMap::new(),
            position_limit,
        }
    }
}

#[derive(Debug, Clone, Copy)]
// tag::borrowed_view[]
pub struct BorrowedPositionView<'a> {
    pub symbol: &'a [u8; 8],
    pub current_position: Option<&'a i64>,
    pub position_limit: i64,
}
// end::borrowed_view[]

pub fn borrowed_view<'a>(state: &'a LiveRiskState, symbol: &'a [u8; 8]) -> BorrowedPositionView<'a> {
    BorrowedPositionView {
        symbol,
        current_position: state.positions.get(symbol),
        position_limit: state.position_limit,
    }
}

// tag::rc_refcell_state[]
pub type LocalRiskHandle = Rc<RefCell<LiveRiskState>>;
// end::rc_refcell_state[]

// tag::apply_fill_local[]
pub fn apply_fill_local(
    handle: &LocalRiskHandle,
    symbol: [u8; 8],
    delta: i64,
) -> Result<i64, BorrowMutError> {
    let mut state = handle.try_borrow_mut()?;
    let entry = state.positions.entry(symbol).or_insert(0);
    *entry += delta;
    Ok(*entry)
}
// end::apply_fill_local[]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharedRiskError {
    Poisoned,
}

// tag::arc_mutex_state[]
pub type SharedRiskHandle = Arc<Mutex<LiveRiskState>>;
// end::arc_mutex_state[]

// tag::apply_fill_shared[]
pub fn apply_fill_shared(
    handle: &SharedRiskHandle,
    symbol: [u8; 8],
    delta: i64,
) -> Result<i64, SharedRiskError> {
    let mut state = handle.lock().map_err(|_| SharedRiskError::Poisoned)?;
    let entry = state.positions.entry(symbol).or_insert(0);
    *entry += delta;
    Ok(*entry)
}
// end::apply_fill_shared[]