use book_rust::ch08::hot_path::{
    baseline_profile, choose_concurrency_model, optimized_profile, WorkloadModel,
};

fn sample_stream() -> Vec<u8> {
    let mut stream = Vec::new();

    let add_payload = {
        let mut payload = Vec::new();
        payload.extend_from_slice(&1_u16.to_be_bytes());
        payload.extend_from_slice(&77_u64.to_be_bytes());
        payload.push(b'B');
        payload.extend_from_slice(&100_u32.to_be_bytes());
        payload.extend_from_slice(b"AAPL    ");
        payload.extend_from_slice(&1234_u32.to_be_bytes());
        payload
    };

    stream.push(b'A');
    stream.push(add_payload.len() as u8);
    stream.extend_from_slice(&add_payload);

    stream
}

#[test]
fn optimized_profile_reduces_modeled_allocations() {
    let stream = sample_stream();
    let baseline = baseline_profile(&stream).expect("baseline profile should succeed");
    let optimized = optimized_profile(&stream).expect("optimized profile should succeed");

    assert_eq!(baseline.parsed_messages, optimized.parsed_messages);
    assert!(baseline.modeled_allocations > optimized.modeled_allocations);
}

#[test]
fn profile_fails_on_truncated_record() {
    let err = baseline_profile(&[b'A', 27, 0x00]).expect_err("record should be truncated");
    let message = err.to_string();
    assert!(message.contains("truncated"));
}

#[test]
fn chooses_async_for_many_consumers() {
    let model = choose_concurrency_model(3, true);
    assert_eq!(model, WorkloadModel::Async);
}
