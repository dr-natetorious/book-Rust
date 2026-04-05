use std::fmt;
use std::sync::mpsc;
use std::thread;

use crate::ch03::order_book::{BookError, BookEvent, OrderBook};
use crate::ch04::stream::FrameIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PipelineReport {
    pub applied_events: usize,
    pub open_orders: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineError {
    Parse { offset: usize, message: String },
    Book { offset: usize, source: BookError },
    ChannelClosed,
}

impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineError::Parse { offset, message } => {
                write!(f, "parse error at offset {offset}: {message}")
            }
            PipelineError::Book { offset, source } => {
                write!(f, "book error at offset {offset}: {source}")
            }
            PipelineError::ChannelClosed => write!(f, "channel closed before all events were processed"),
        }
    }
}

impl std::error::Error for PipelineError {}

// tag::parser_thread[]
fn spawn_parser(
    stream: Vec<u8>,
    sender: mpsc::Sender<(usize, BookEvent)>,
) -> thread::JoinHandle<Result<(), PipelineError>> {
    thread::spawn(move || {
        for frame_result in FrameIter::new(&stream) {
            let frame = frame_result.map_err(|err| PipelineError::Parse {
                offset: err.offset,
                message: err.source.to_string(),
            })?;

            let message = frame.parse().map_err(|source| PipelineError::Parse {
                offset: frame.offset,
                message: source.to_string(),
            })?;

            let event = BookEvent::try_from(message).map_err(|source| PipelineError::Book {
                offset: frame.offset,
                source,
            })?;

            sender
                .send((frame.offset, event))
                .map_err(|_| PipelineError::ChannelClosed)?;
        }

        Ok(())
    })
}
// end::parser_thread[]

// tag::book_thread[]
fn spawn_book_worker(
    receiver: mpsc::Receiver<(usize, BookEvent)>,
) -> thread::JoinHandle<Result<PipelineReport, PipelineError>> {
    thread::spawn(move || {
        let mut book = OrderBook::new();
        let mut applied_events = 0_usize;

        for (offset, event) in receiver {
            book.apply(event)
                .map_err(|source| PipelineError::Book { offset, source })?;
            applied_events += 1;
        }

        Ok(PipelineReport {
            applied_events,
            open_orders: book.open_order_count(),
        })
    })
}
// end::book_thread[]

// tag::run_threaded_pipeline[]
pub fn run_threaded_pipeline(stream: &[u8]) -> Result<PipelineReport, PipelineError> {
    let (sender, receiver) = mpsc::channel();
    let parser = spawn_parser(stream.to_vec(), sender);
    let worker = spawn_book_worker(receiver);

    parser
        .join()
        .map_err(|_| PipelineError::ChannelClosed)??;

    worker
        .join()
        .map_err(|_| PipelineError::ChannelClosed)?
}
// end::run_threaded_pipeline[]
