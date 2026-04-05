use book_rust::ch03::order_book::{OpenOrder, Side};
use book_rust::ch06::risk::{
    DynRiskEngine, GenericRiskEngine, NotionalLimitCheck, PositionLimitCheck, RiskState,
    RiskViolation,
};

fn sample_order() -> OpenOrder {
    OpenOrder {
        order_ref: 42,
        stock: *b"GOOG    ",
        side: Side::Buy,
        remaining_shares: 1_000,
        price: 120,
    }
}

#[test]
fn generic_engine_accepts_within_limits() {
    let engine = GenericRiskEngine::new(PositionLimitCheck);
    let state = RiskState {
        current_position: 2_000,
        position_limit: 5_000,
        notional_limit: 250_000,
    };

    engine
        .approve(&sample_order(), &state)
        .expect("position limit should pass");
}

#[test]
fn generic_and_dyn_engines_agree_on_rejection() {
    let order = sample_order();
    let state = RiskState {
        current_position: 4_500,
        position_limit: 5_000,
        notional_limit: 100_000,
    };

    let generic = GenericRiskEngine::new(PositionLimitCheck);
    let dyn_engine = DynRiskEngine::new(vec![
        Box::new(PositionLimitCheck),
        Box::new(NotionalLimitCheck),
    ]);

    assert!(matches!(
        generic.approve(&order, &state),
        Err(RiskViolation::PositionLimit { .. })
    ));
    assert!(matches!(
        dyn_engine.approve(&order, &state),
        Err(RiskViolation::PositionLimit { .. })
    ));
}