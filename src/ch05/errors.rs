use std::fmt;

use crate::ch02::feed::FeedError;
use crate::ch03::order_book::{BookError, BookEvent, OrderBook};
use crate::ch04::stream::FrameIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// tag::corruption_policy[]
pub enum CorruptionPolicy {
    FailFast,
    SkipBadRecords,
}
// end::corruption_policy[]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RejectedReason {
    Feed(FeedError),
    Book(BookError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RejectedRecord {
    pub offset: usize,
    pub msg_type: u8,
    pub reason: RejectedReason,
}

#[derive(Debug, Clone, PartialEq, Eq)]
// tag::replay_report[]
pub struct ReplayReport {
    pub applied: usize,
    pub rejected: Vec<RejectedRecord>,
}
// end::replay_report[]

#[derive(Debug, Clone, PartialEq, Eq)]
// tag::processing_error[]
pub enum ProcessingError {
    Feed { offset: usize, source: FeedError },
    Book { offset: usize, source: BookError },
}
// end::processing_error[]

impl fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessingError::Feed { offset, source } => {
                write!(f, "feed error at offset {offset}: {source}")
            }
            ProcessingError::Book { offset, source } => {
                write!(f, "book error at offset {offset}: {source}")
            }
        }
    }
}

impl std::error::Error for ProcessingError {}

// tag::replay_with_policy[]
pub fn replay_with_policy(
    stream: &[u8],
    book: &mut OrderBook,
    policy: CorruptionPolicy,
) -> Result<ReplayReport, ProcessingError> {
    let mut report = ReplayReport {
        applied: 0,
        rejected: Vec::new(),
    };

    for frame_result in FrameIter::new(stream) {
        let frame = match frame_result {
            Ok(frame) => frame,
            Err(err) => {
                return Err(ProcessingError::Feed {
                    offset: err.offset,
                    source: err.source,
                })
            }
        };

        let message = match frame.parse() {
            Ok(message) => message,
            Err(source) => {
                if matches!(policy, CorruptionPolicy::SkipBadRecords) {
                    report.rejected.push(RejectedRecord {
                        offset: frame.offset,
                        msg_type: frame.msg_type,
                        reason: RejectedReason::Feed(source),
                    });
                    continue;
                }

                return Err(ProcessingError::Feed {
                    offset: frame.offset,
                    source,
                });
            }
        };

        let event = match BookEvent::try_from(message) {
            Ok(event) => event,
            Err(source) => {
                if matches!(policy, CorruptionPolicy::SkipBadRecords) {
                    report.rejected.push(RejectedRecord {
                        offset: frame.offset,
                        msg_type: frame.msg_type,
                        reason: RejectedReason::Book(source),
                    });
                    continue;
                }

                return Err(ProcessingError::Book {
                    offset: frame.offset,
                    source,
                });
            }
        };

        match book.apply(event) {
            Ok(()) => report.applied += 1,
            Err(source) => {
                if matches!(policy, CorruptionPolicy::SkipBadRecords) {
                    report.rejected.push(RejectedRecord {
                        offset: frame.offset,
                        msg_type: frame.msg_type,
                        reason: RejectedReason::Book(source),
                    });
                    continue;
                }

                return Err(ProcessingError::Book {
                    offset: frame.offset,
                    source,
                });
            }
        }
    }

    Ok(report)
}
// end::replay_with_policy[]