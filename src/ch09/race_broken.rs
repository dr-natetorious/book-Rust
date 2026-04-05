// This file is intentionally broken for chapter exercises.

use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

// tag::race_broken[]
pub fn write_without_mutex() {
    let state = Arc::new(HashMap::<u64, u32>::new());
    let handle = Arc::clone(&state);

    thread::spawn(move || {
        // Intentionally wrong: Arc<T> does not permit mutable access to T.
        handle.insert(42, 1);
    });
}
// end::race_broken[]
