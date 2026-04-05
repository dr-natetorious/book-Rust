// Intentionally broken example for chapter 7: Arc shares ownership, not mutability.

use std::sync::Arc;

use crate::ch07::shared::LiveRiskState;

// tag::arc_without_mutex_broken[]
pub fn increment_without_mutex(handle: Arc<LiveRiskState>, symbol: [u8; 8], delta: i64) {
    let entry = handle.positions.get(&symbol).copied().unwrap_or(0);
    handle.positions.insert(symbol, entry + delta);
}
// end::arc_without_mutex_broken[]
