use book_rust::ch09::pipeline::{run_threaded_pipeline, PipelineError};

fn add(order_ref: u64, shares: u32) -> Vec<u8> {
    let mut payload = Vec::new();
    payload.extend_from_slice(&1_u16.to_be_bytes());
    payload.extend_from_slice(&order_ref.to_be_bytes());
    payload.push(b'B');
    payload.extend_from_slice(&shares.to_be_bytes());
    payload.extend_from_slice(b"MSFT    ");
    payload.extend_from_slice(&120_u32.to_be_bytes());

    let mut frame = vec![b'A', payload.len() as u8];
    frame.extend_from_slice(&payload);
    frame
}

fn execute(order_ref: u64, executed_shares: u32) -> Vec<u8> {
    let mut payload = Vec::new();
    payload.extend_from_slice(&1_u16.to_be_bytes());
    payload.extend_from_slice(&order_ref.to_be_bytes());
    payload.extend_from_slice(&executed_shares.to_be_bytes());

    let mut frame = vec![b'E', payload.len() as u8];
    frame.extend_from_slice(&payload);
    frame
}

#[test]
fn threaded_pipeline_applies_events() {
    let mut stream = Vec::new();
    stream.extend_from_slice(&add(10, 100));
    stream.extend_from_slice(&execute(10, 100));

    let report = run_threaded_pipeline(&stream).expect("pipeline should succeed");
    assert_eq!(report.applied_events, 2);
    assert_eq!(report.open_orders, 0);
}

#[test]
fn threaded_pipeline_surfaces_book_error() {
    let stream = execute(999, 5);

    let err = run_threaded_pipeline(&stream).expect_err("unknown order should fail");
    assert!(matches!(err, PipelineError::Book { .. }));
}
