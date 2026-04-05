use std::collections::HashMap;
use std::fmt;

use crate::ch02::feed::{scan_messages, FeedError, MessageView};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenOrder {
    pub order_ref: u64,
    pub stock: [u8; 8],
    pub side: Side,
    pub remaining_shares: u32,
    pub price: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
// tag::naive_order_state[]
pub struct NaiveOrderState {
    pub live: bool,
    pub canceled: bool,
    pub executed: bool,
}
// end::naive_order_state[]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClosedReason {
    Cancelled,
    Executed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosedOrder {
    pub order_ref: u64,
    pub reason: ClosedReason,
}

#[derive(Debug, Clone, PartialEq, Eq)]
// tag::order_state[]
pub enum OrderState {
    Open(OpenOrder),
    Closed(ClosedOrder),
}
// end::order_state[]

#[derive(Debug, Clone, PartialEq, Eq)]
// tag::book_event[]
pub enum BookEvent {
    Add(OpenOrder),
    Cancel {
        order_ref: u64,
        canceled_shares: u32,
    },
    Execute {
        order_ref: u64,
        executed_shares: u32,
    },
}
// end::book_event[]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BookError {
    DuplicateOrder(u64),
    UnknownOrder(u64),
    OrderAlreadyClosed(u64),
    ReductionExceedsRemaining {
        order_ref: u64,
        remaining: u32,
        requested: u32,
    },
    UnsupportedSide(u8),
}

impl fmt::Display for BookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BookError::DuplicateOrder(order_ref) => {
                write!(f, "order {order_ref} already exists in the book")
            }
            BookError::UnknownOrder(order_ref) => {
                write!(f, "order {order_ref} is not in the book")
            }
            BookError::OrderAlreadyClosed(order_ref) => {
                write!(f, "order {order_ref} is already closed")
            }
            BookError::ReductionExceedsRemaining {
                order_ref,
                remaining,
                requested,
            } => write!(
                f,
                "order {order_ref} has {remaining} shares remaining but {requested} were requested"
            ),
            BookError::UnsupportedSide(side) => write!(f, "unsupported side byte: {side}"),
        }
    }
}

impl std::error::Error for BookError {}

#[derive(Debug)]
pub enum ReplayError {
    Feed(FeedError),
    Book(BookError),
}

impl fmt::Display for ReplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReplayError::Feed(err) => err.fmt(f),
            ReplayError::Book(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for ReplayError {}

impl From<FeedError> for ReplayError {
    fn from(value: FeedError) -> Self {
        ReplayError::Feed(value)
    }
}

impl From<BookError> for ReplayError {
    fn from(value: BookError) -> Self {
        ReplayError::Book(value)
    }
}

impl TryFrom<MessageView<'_>> for BookEvent {
    type Error = BookError;

    fn try_from(value: MessageView<'_>) -> Result<Self, Self::Error> {
        match value {
            MessageView::Add(add) => {
                let side = match add.side {
                    b'B' => Side::Buy,
                    b'S' => Side::Sell,
                    other => return Err(BookError::UnsupportedSide(other)),
                };

                let mut stock = [0_u8; 8];
                stock.copy_from_slice(add.stock);

                Ok(BookEvent::Add(OpenOrder {
                    order_ref: add.order_ref,
                    stock,
                    side,
                    remaining_shares: add.shares,
                    price: add.price,
                }))
            }
            MessageView::Cancel(cancel) => Ok(BookEvent::Cancel {
                order_ref: cancel.order_ref,
                canceled_shares: cancel.canceled_shares,
            }),
            MessageView::Execute(execute) => Ok(BookEvent::Execute {
                order_ref: execute.order_ref,
                executed_shares: execute.executed_shares,
            }),
        }
    }
}

#[derive(Debug, Default)]
// tag::order_book[]
pub struct OrderBook {
    orders: HashMap<u64, OrderState>,
}
// end::order_book[]

impl OrderBook {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open_order_count(&self) -> usize {
        self.orders
            .values()
            .filter(|state| matches!(state, OrderState::Open(_)))
            .count()
    }

    pub fn state(&self, order_ref: u64) -> Option<&OrderState> {
        self.orders.get(&order_ref)
    }

    // tag::apply_event[]
    pub fn apply(&mut self, event: BookEvent) -> Result<(), BookError> {
        match event {
            BookEvent::Add(order) => {
                if self.orders.contains_key(&order.order_ref) {
                    return Err(BookError::DuplicateOrder(order.order_ref));
                }

                self.orders.insert(order.order_ref, OrderState::Open(order));
                Ok(())
            }
            BookEvent::Cancel {
                order_ref,
                canceled_shares,
            } => self.reduce(order_ref, canceled_shares, ClosedReason::Cancelled),
            BookEvent::Execute {
                order_ref,
                executed_shares,
            } => self.reduce(order_ref, executed_shares, ClosedReason::Executed),
        }
    }
    // end::apply_event[]

    fn reduce(
        &mut self,
        order_ref: u64,
        requested: u32,
        reason: ClosedReason,
    ) -> Result<(), BookError> {
        let state = self
            .orders
            .get_mut(&order_ref)
            .ok_or(BookError::UnknownOrder(order_ref))?;

        match state {
            OrderState::Open(order) => {
                if requested > order.remaining_shares {
                    return Err(BookError::ReductionExceedsRemaining {
                        order_ref,
                        remaining: order.remaining_shares,
                        requested,
                    });
                }

                if requested == order.remaining_shares {
                    *state = OrderState::Closed(ClosedOrder { order_ref, reason });
                } else {
                    order.remaining_shares -= requested;
                }

                Ok(())
            }
            OrderState::Closed(_) => Err(BookError::OrderAlreadyClosed(order_ref)),
        }
    }

    // tag::replay_stream[]
    pub fn replay(&mut self, stream: &[u8]) -> Result<(), ReplayError> {
        scan_messages(stream, |message| {
            let event = BookEvent::try_from(message)?;
            self.apply(event)?;
            Ok(())
        })
    }
    // end::replay_stream[]
}