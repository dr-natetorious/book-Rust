use std::fmt;

use crate::ch03::order_book::OpenOrder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiskState {
    pub current_position: i64,
    pub position_limit: i64,
    pub notional_limit: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RiskViolation {
    PositionLimit {
        attempted: i64,
        limit: i64,
    },
    NotionalLimit {
        attempted: u64,
        limit: u64,
    },
}

impl fmt::Display for RiskViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RiskViolation::PositionLimit { attempted, limit } => {
                write!(f, "position {attempted} exceeds limit {limit}")
            }
            RiskViolation::NotionalLimit { attempted, limit } => {
                write!(f, "notional {attempted} exceeds limit {limit}")
            }
        }
    }
}

impl std::error::Error for RiskViolation {}

// tag::risk_check_trait[]
pub trait RiskCheck {
    fn name(&self) -> &'static str;
    fn evaluate(&self, order: &OpenOrder, state: &RiskState) -> Result<(), RiskViolation>;
}
// end::risk_check_trait[]

#[derive(Debug, Clone, Copy, Default)]
pub struct PositionLimitCheck;

impl RiskCheck for PositionLimitCheck {
    fn name(&self) -> &'static str {
        "position-limit"
    }

    fn evaluate(&self, order: &OpenOrder, state: &RiskState) -> Result<(), RiskViolation> {
        let attempted = state.current_position + order.remaining_shares as i64;
        if attempted > state.position_limit {
            return Err(RiskViolation::PositionLimit {
                attempted,
                limit: state.position_limit,
            });
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NotionalLimitCheck;

impl RiskCheck for NotionalLimitCheck {
    fn name(&self) -> &'static str {
        "notional-limit"
    }

    fn evaluate(&self, order: &OpenOrder, state: &RiskState) -> Result<(), RiskViolation> {
        let attempted = u64::from(order.remaining_shares) * u64::from(order.price);
        if attempted > state.notional_limit {
            return Err(RiskViolation::NotionalLimit {
                attempted,
                limit: state.notional_limit,
            });
        }

        Ok(())
    }
}

// tag::generic_risk_engine[]
pub struct GenericRiskEngine<C> {
    check: C,
}

impl<C: RiskCheck> GenericRiskEngine<C> {
    pub fn new(check: C) -> Self {
        Self { check }
    }

    pub fn approve(&self, order: &OpenOrder, state: &RiskState) -> Result<(), RiskViolation> {
        self.check.evaluate(order, state)
    }
}
// end::generic_risk_engine[]

// tag::dyn_risk_engine[]
pub struct DynRiskEngine {
    checks: Vec<Box<dyn RiskCheck>>,
}

impl DynRiskEngine {
    pub fn new(checks: Vec<Box<dyn RiskCheck>>) -> Self {
        Self { checks }
    }

    pub fn approve(&self, order: &OpenOrder, state: &RiskState) -> Result<(), RiskViolation> {
        for check in &self.checks {
            check.evaluate(order, state)?;
        }

        Ok(())
    }
}
// end::dyn_risk_engine[]