use book_rust::ch12::api::{
    classify_change, FeedClientBuilder, JsonFormat, VersionChange, VersionError,
};

#[test]
fn typestate_builder_produces_snapshot() {
    let client = FeedClientBuilder::new().configure_symbol(*b"AAPL    ").build();

    let snapshot = client.snapshot(JsonFormat, 125_100, 125_200);
    assert!(snapshot.contains("AAPL"));
}

#[test]
fn semver_gate_rejects_invalid_string() {
    let err = classify_change("1.2.3", "2.0")
        .expect_err("invalid semver should fail");
    assert!(matches!(err, VersionError::InvalidVersion(_)));
}

#[test]
fn semver_gate_flags_major_as_breaking() {
    let change = classify_change("1.4.9", "2.0.0").expect("valid semver should parse");
    assert_eq!(change, VersionChange::Breaking);
}
