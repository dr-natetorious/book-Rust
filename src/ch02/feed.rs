use std::fmt;

pub const ADD_ORDER_TYPE: u8 = b'A';
pub const EXECUTE_ORDER_TYPE: u8 = b'E';
pub const CANCEL_ORDER_TYPE: u8 = b'X';

pub const ADD_ORDER_PAYLOAD_LEN: usize = 27;
pub const EXECUTE_ORDER_PAYLOAD_LEN: usize = 14;
pub const CANCEL_ORDER_PAYLOAD_LEN: usize = 14;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// tag::add_order_view[]
pub struct AddOrderView<'a> {
    pub stock_locate: u16,
    pub order_ref: u64,
    pub side: u8,
    pub shares: u32,
    pub stock: &'a [u8],
    pub price: u32,
}
// end::add_order_view[]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecuteOrderView {
    pub stock_locate: u16,
    pub order_ref: u64,
    pub executed_shares: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CancelOrderView {
    pub stock_locate: u16,
    pub order_ref: u64,
    pub canceled_shares: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// tag::message_view[]
pub enum MessageView<'a> {
    Add(AddOrderView<'a>),
    Execute(ExecuteOrderView),
    Cancel(CancelOrderView),
}
// end::message_view[]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeedError {
    TruncatedRecord { expected: usize, actual: usize },
    UnsupportedMessageType(u8),
    InvalidRecordLength {
        msg_type: u8,
        expected: usize,
        actual: usize,
    },
}

impl fmt::Display for FeedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FeedError::TruncatedRecord { expected, actual } => {
                write!(f, "record is truncated: expected {expected} bytes, got {actual}")
            }
            FeedError::UnsupportedMessageType(msg_type) => {
                write!(f, "unsupported message type: {msg_type}")
            }
            FeedError::InvalidRecordLength {
                msg_type,
                expected,
                actual,
            } => write!(
                f,
                "message {msg_type} has invalid length: expected {expected} bytes, got {actual}"
            ),
        }
    }
}

impl std::error::Error for FeedError {}

fn be_u16(bytes: &[u8]) -> u16 {
    u16::from_be_bytes([bytes[0], bytes[1]])
}

fn be_u32(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn be_u64(bytes: &[u8]) -> u64 {
    u64::from_be_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ])
}

fn expect_len(msg_type: u8, payload: &[u8], expected: usize) -> Result<(), FeedError> {
    if payload.len() != expected {
        return Err(FeedError::InvalidRecordLength {
            msg_type,
            expected,
            actual: payload.len(),
        });
    }

    Ok(())
}

// tag::parse_message[]
pub fn parse_message(msg_type: u8, payload: &[u8]) -> Result<MessageView<'_>, FeedError> {
    match msg_type {
        ADD_ORDER_TYPE => {
            expect_len(msg_type, payload, ADD_ORDER_PAYLOAD_LEN)?;
            Ok(MessageView::Add(AddOrderView {
                stock_locate: be_u16(&payload[0..2]),
                order_ref: be_u64(&payload[2..10]),
                side: payload[10],
                shares: be_u32(&payload[11..15]),
                stock: &payload[15..23],
                price: be_u32(&payload[23..27]),
            }))
        }
        EXECUTE_ORDER_TYPE => {
            expect_len(msg_type, payload, EXECUTE_ORDER_PAYLOAD_LEN)?;
            Ok(MessageView::Execute(ExecuteOrderView {
                stock_locate: be_u16(&payload[0..2]),
                order_ref: be_u64(&payload[2..10]),
                executed_shares: be_u32(&payload[10..14]),
            }))
        }
        CANCEL_ORDER_TYPE => {
            expect_len(msg_type, payload, CANCEL_ORDER_PAYLOAD_LEN)?;
            Ok(MessageView::Cancel(CancelOrderView {
                stock_locate: be_u16(&payload[0..2]),
                order_ref: be_u64(&payload[2..10]),
                canceled_shares: be_u32(&payload[10..14]),
            }))
        }
        other => Err(FeedError::UnsupportedMessageType(other)),
    }
}
// end::parse_message[]

// tag::scan_messages[]
pub fn scan_messages<E, F>(stream: &[u8], mut handler: F) -> Result<(), E>
where
    E: From<FeedError>,
    F: FnMut(MessageView<'_>) -> Result<(), E>,
{
    let mut offset = 0_usize;

    while offset < stream.len() {
        if stream.len() - offset < 2 {
            return Err(E::from(FeedError::TruncatedRecord {
                expected: 2,
                actual: stream.len() - offset,
            }));
        }

        let msg_type = stream[offset];
        let payload_len = stream[offset + 1] as usize;
        offset += 2;

        if stream.len() - offset < payload_len {
            return Err(E::from(FeedError::TruncatedRecord {
                expected: payload_len,
                actual: stream.len() - offset,
            }));
        }

        let payload = &stream[offset..offset + payload_len];
        offset += payload_len;

        let message = parse_message(msg_type, payload).map_err(E::from)?;
        handler(message)?;
    }

    Ok(())
}
// end::scan_messages[]