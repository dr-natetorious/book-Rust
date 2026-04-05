// This file is intentionally broken for chapter exercises.

use std::sync::{Arc, Mutex};

// tag::mutex_across_await_broken[]
pub async fn spawn_with_guard(state: Arc<Mutex<u64>>) {
    let guard = state.lock().expect("mutex should lock");

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        println!("{}", *guard);
    });
}
// end::mutex_across_await_broken[]
