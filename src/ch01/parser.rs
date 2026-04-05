use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
// tag::add_order_message[]
pub struct AddOrderMessage {
    pub stock_locate: u16,
    pub order_ref: u64,
    pub side: u8,
    pub shares: u32,
    pub stock: [u8; 8],
    pub price: u32,
}
// end::add_order_message[]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    TruncatedRecord { expected: usize, actual: usize },
    UnsupportedMessageType(u8),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::TruncatedRecord { expected, actual } => {
                write!(f, "record is truncated: expected {expected} bytes, got {actual}")
            }
            ParseError::UnsupportedMessageType(ty) => {
                write!(f, "unsupported message type: {ty}")
            }
        }
    }
}

impl std::error::Error for ParseError {}

pub const ADD_ORDER_TYPE: u8 = b'A';
pub const ADD_ORDER_PAYLOAD_LEN: usize = 27;

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

// tag::parse_add_order[]
pub fn parse_add_order(payload: &[u8]) -> Result<AddOrderMessage, ParseError> {
    if payload.len() < ADD_ORDER_PAYLOAD_LEN {
        return Err(ParseError::TruncatedRecord {
            expected: ADD_ORDER_PAYLOAD_LEN,
            actual: payload.len(),
        });
    }

    let stock_locate = be_u16(&payload[0..2]);
    let order_ref = be_u64(&payload[2..10]);
    let side = payload[10];
    let shares = be_u32(&payload[11..15]);

    let mut stock = [0_u8; 8];
    stock.copy_from_slice(&payload[15..23]);

    let price = be_u32(&payload[23..27]);

    Ok(AddOrderMessage {
        stock_locate,
        order_ref,
        side,
        shares,
        stock,
        price,
    })
}
// end::parse_add_order[]

// tag::parse_records[]
pub fn parse_records(stream: &[u8]) -> Result<Vec<AddOrderMessage>, ParseError> {
    let mut offset = 0_usize;
    let mut out = Vec::new();

    while offset < stream.len() {
        if stream.len() - offset < 2 {
            return Err(ParseError::TruncatedRecord {
                expected: 2,
                actual: stream.len() - offset,
            });
        }

        let msg_type = stream[offset];
        let len = stream[offset + 1] as usize;
        offset += 2;

        if stream.len() - offset < len {
            return Err(ParseError::TruncatedRecord {
                expected: len,
                actual: stream.len() - offset,
            });
        }

        let payload = &stream[offset..offset + len];
        offset += len;

        match msg_type {
            ADD_ORDER_TYPE => out.push(parse_add_order(payload)?),
            other => return Err(ParseError::UnsupportedMessageType(other)),
        }
    }

    Ok(out)
}
// end::parse_records[]
