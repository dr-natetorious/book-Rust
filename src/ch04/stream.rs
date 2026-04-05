use std::fmt;
use std::io::{self, Read};

use crate::ch02::feed::{parse_message, FeedError, MessageView};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// tag::frame_ref[]
pub struct FrameRef<'a> {
    pub offset: usize,
    pub msg_type: u8,
    pub payload: &'a [u8],
}
// end::frame_ref[]

impl<'a> FrameRef<'a> {
    pub fn parse(&self) -> Result<MessageView<'a>, FeedError> {
        parse_message(self.msg_type, self.payload)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameIterError {
    pub offset: usize,
    pub source: FeedError,
}

impl fmt::Display for FrameIterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "frame at offset {} failed: {}", self.offset, self.source)
    }
}

impl std::error::Error for FrameIterError {}

// tag::frame_iter[]
pub struct FrameIter<'a> {
    stream: &'a [u8],
    offset: usize,
    finished: bool,
}
// end::frame_iter[]

impl<'a> FrameIter<'a> {
    pub fn new(stream: &'a [u8]) -> Self {
        Self {
            stream,
            offset: 0,
            finished: false,
        }
    }
}

// tag::iterator_impl[]
impl<'a> Iterator for FrameIter<'a> {
    type Item = Result<FrameRef<'a>, FrameIterError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished || self.offset >= self.stream.len() {
            return None;
        }

        let frame_offset = self.offset;
        let remaining = &self.stream[self.offset..];

        if remaining.len() < 2 {
            self.finished = true;
            return Some(Err(FrameIterError {
                offset: frame_offset,
                source: FeedError::TruncatedRecord {
                    expected: 2,
                    actual: remaining.len(),
                },
            }));
        }

        let msg_type = remaining[0];
        let payload_len = remaining[1] as usize;
        let payload_start = self.offset + 2;
        let payload_end = payload_start + payload_len;

        if payload_end > self.stream.len() {
            self.finished = true;
            return Some(Err(FrameIterError {
                offset: frame_offset,
                source: FeedError::TruncatedRecord {
                    expected: payload_len,
                    actual: self.stream.len() - payload_start,
                },
            }));
        }

        self.offset = payload_end;
        Some(Ok(FrameRef {
            offset: frame_offset,
            msg_type,
            payload: &self.stream[payload_start..payload_end],
        }))
    }
}
// end::iterator_impl[]

pub struct ParsedMessageIter<'a> {
    frames: FrameIter<'a>,
}

impl<'a> ParsedMessageIter<'a> {
    pub fn new(stream: &'a [u8]) -> Self {
        Self {
            frames: FrameIter::new(stream),
        }
    }
}

// tag::parsed_message_iter_impl[]
impl<'a> Iterator for ParsedMessageIter<'a> {
    type Item = Result<MessageView<'a>, FrameIterError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.frames.next()? {
            Ok(frame) => Some(frame.parse().map_err(|source| FrameIterError {
                offset: frame.offset,
                source,
            })),
            Err(err) => Some(Err(err)),
        }
    }
}
// end::parsed_message_iter_impl[]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedFrame {
    pub offset: usize,
    pub msg_type: u8,
    pub payload: Vec<u8>,
}

#[derive(Debug)]
pub enum FileFrameError {
    Io { offset: usize, source: io::Error },
    TruncatedHeader { offset: usize, actual: usize },
    TruncatedPayload {
        offset: usize,
        expected: usize,
        actual: usize,
    },
}

impl fmt::Display for FileFrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileFrameError::Io { offset, source } => {
                write!(f, "I/O failure at offset {offset}: {source}")
            }
            FileFrameError::TruncatedHeader { offset, actual } => {
                write!(f, "header at offset {offset} is truncated: got {actual} byte(s)")
            }
            FileFrameError::TruncatedPayload {
                offset,
                expected,
                actual,
            } => write!(
                f,
                "payload at offset {offset} is truncated: expected {expected} bytes, got {actual}"
            ),
        }
    }
}

impl std::error::Error for FileFrameError {}

// tag::file_frame_iter[]
pub struct FileFrameIter<R> {
    reader: R,
    offset: usize,
    done: bool,
}
// end::file_frame_iter[]

impl<R: Read> FileFrameIter<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            offset: 0,
            done: false,
        }
    }
}

impl<R: Read> Iterator for FileFrameIter<R> {
    type Item = Result<OwnedFrame, FileFrameError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let frame_offset = self.offset;
        let mut first = [0_u8; 1];
        match self.reader.read(&mut first) {
            Ok(0) => {
                self.done = true;
                return None;
            }
            Ok(1) => {}
            Ok(actual) => {
                self.done = true;
                return Some(Err(FileFrameError::TruncatedHeader {
                    offset: frame_offset,
                    actual,
                }));
            }
            Err(source) => {
                self.done = true;
                return Some(Err(FileFrameError::Io {
                    offset: frame_offset,
                    source,
                }));
            }
        }

        let mut second = [0_u8; 1];
        if let Err(source) = self.reader.read_exact(&mut second) {
            self.done = true;
            return Some(match source.kind() {
                io::ErrorKind::UnexpectedEof => Err(FileFrameError::TruncatedHeader {
                    offset: frame_offset,
                    actual: 1,
                }),
                _ => Err(FileFrameError::Io {
                    offset: frame_offset + 1,
                    source,
                }),
            });
        }

        let payload_len = second[0] as usize;
        let mut payload = vec![0_u8; payload_len];
        let mut read = 0;
        while read < payload_len {
            match self.reader.read(&mut payload[read..]) {
                Ok(0) => {
                    self.done = true;
                    return Some(Err(FileFrameError::TruncatedPayload {
                        offset: frame_offset,
                        expected: payload_len,
                        actual: read,
                    }));
                }
                Ok(n) => read += n,
                Err(source) => {
                    self.done = true;
                    return Some(Err(FileFrameError::Io {
                        offset: frame_offset + 2 + read,
                        source,
                    }));
                }
            }
        }

        self.offset += 2 + payload_len;

        Some(Ok(OwnedFrame {
            offset: frame_offset,
            msg_type: first[0],
            payload,
        }))
    }
}