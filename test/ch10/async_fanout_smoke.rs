use book_rust::ch10::async_fanout::{run_async_fanout, AsyncFanoutError, FanoutFrame};

fn frames(count: usize) -> Vec<FanoutFrame> {
    (0..count)
        .map(|seq| FanoutFrame {
            sequence: seq as u64,
            payload: vec![0xAA; 16],
        })
        .collect()
}

#[tokio::test(flavor = "current_thread")]
async fn async_fanout_reports_backpressure() {
    let report = run_async_fanout(frames(12), 2, 1, false)
        .await
        .expect("fanout should complete");

    assert_eq!(report.sent_frames, 12);
    assert!(report.backpressure_events > 0);
}

#[tokio::test(flavor = "current_thread")]
async fn async_fanout_reports_closed_consumer() {
    let err = run_async_fanout(frames(3), 1, 0, true)
        .await
        .expect_err("closed consumer should fail");

    assert_eq!(err, AsyncFanoutError::ConsumerClosed);
}
