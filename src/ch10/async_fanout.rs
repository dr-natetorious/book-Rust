use std::fmt;

use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FanoutFrame {
    pub sequence: u64,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FanoutReport {
    pub sent_frames: usize,
    pub backpressure_events: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsyncFanoutError {
    ConsumerClosed,
}

impl fmt::Display for AsyncFanoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsyncFanoutError::ConsumerClosed => write!(f, "consumer closed channel early"),
        }
    }
}

impl std::error::Error for AsyncFanoutError {}

// tag::fanout_channels[]
pub fn fanout_channels(buffer: usize) -> (
    [mpsc::Sender<FanoutFrame>; 3],
    [mpsc::Receiver<FanoutFrame>; 3],
) {
    let (book_tx, book_rx) = mpsc::channel(buffer);
    let (risk_tx, risk_rx) = mpsc::channel(buffer);
    let (log_tx, log_rx) = mpsc::channel(buffer);
    ([book_tx, risk_tx, log_tx], [book_rx, risk_rx, log_rx])
}
// end::fanout_channels[]

async fn drain_consumer(mut rx: mpsc::Receiver<FanoutFrame>, delay_ms: u64) -> usize {
    let mut seen = 0_usize;
    while rx.recv().await.is_some() {
        seen += 1;
        if delay_ms > 0 {
            sleep(Duration::from_millis(delay_ms)).await;
        }
    }
    seen
}

// tag::run_async_fanout[]
pub async fn run_async_fanout(
    frames: Vec<FanoutFrame>,
    buffer: usize,
    slow_consumer_delay_ms: u64,
    drop_logger_before_send: bool,
) -> Result<FanoutReport, AsyncFanoutError> {
    let (senders, receivers) = fanout_channels(buffer);
    let [book_tx, risk_tx, log_tx] = senders;
    let [book_rx, risk_rx, log_rx] = receivers;

    let book_task = tokio::spawn(drain_consumer(book_rx, 0));
    let risk_task = tokio::spawn(drain_consumer(risk_rx, slow_consumer_delay_ms));
    let log_task = tokio::spawn(drain_consumer(log_rx, 0));

    if drop_logger_before_send {
        log_task.abort();
    }

    let mut backpressure_events = 0_usize;
    let mut sent_frames = 0_usize;

    for frame in frames {
        for tx in [&book_tx, &risk_tx, &log_tx] {
            match tx.try_send(frame.clone()) {
                Ok(()) => {}
                Err(mpsc::error::TrySendError::Full(pending)) => {
                    backpressure_events += 1;
                    tx.send(pending)
                        .await
                        .map_err(|_| AsyncFanoutError::ConsumerClosed)?;
                }
                Err(mpsc::error::TrySendError::Closed(_)) => {
                    return Err(AsyncFanoutError::ConsumerClosed);
                }
            }
        }
        sent_frames += 1;
    }

    drop(book_tx);
    drop(risk_tx);
    drop(log_tx);

    let _ = book_task.await;
    let _ = risk_task.await;
    let _ = log_task.await;

    Ok(FanoutReport {
        sent_frames,
        backpressure_events,
    })
}
// end::run_async_fanout[]
