use std::fmt;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeedClient {
    symbol: [u8; 8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unconfigured;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Configured;

// tag::typestate_builder[]
pub struct FeedClientBuilder<State> {
    symbol: Option<[u8; 8]>,
    _state: PhantomData<State>,
}

impl FeedClientBuilder<Unconfigured> {
    pub fn new() -> Self {
        Self {
            symbol: None,
            _state: PhantomData,
        }
    }

    pub fn configure_symbol(mut self, symbol: [u8; 8]) -> FeedClientBuilder<Configured> {
        self.symbol = Some(symbol);
        FeedClientBuilder {
            symbol: self.symbol,
            _state: PhantomData,
        }
    }
}

impl FeedClientBuilder<Configured> {
    pub fn build(self) -> FeedClient {
        FeedClient {
            symbol: self.symbol.expect("configured builder must hold symbol"),
        }
    }
}
// end::typestate_builder[]

mod sealed {
    pub trait Sealed {}
}

// tag::sealed_trait[]
pub trait SnapshotFormat: sealed::Sealed {
    fn encode(&self, symbol: [u8; 8], best_bid: u32, best_ask: u32) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsonFormat;

impl sealed::Sealed for JsonFormat {}

impl SnapshotFormat for JsonFormat {
    fn encode(&self, symbol: [u8; 8], best_bid: u32, best_ask: u32) -> String {
        format!(
            "{{\"symbol\":\"{}\",\"best_bid\":{},\"best_ask\":{}}}",
            String::from_utf8_lossy(&symbol),
            best_bid,
            best_ask
        )
    }
}
// end::sealed_trait[]

impl FeedClient {
    pub fn snapshot<F: SnapshotFormat>(&self, format: F, best_bid: u32, best_ask: u32) -> String {
        format.encode(self.symbol, best_bid, best_ask)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionChange {
    Compatible,
    Breaking,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionError {
    InvalidVersion(String),
}

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VersionError::InvalidVersion(v) => write!(f, "invalid semver string: {v}"),
        }
    }
}

impl std::error::Error for VersionError {}

fn parse_semver(version: &str) -> Result<(u64, u64, u64), VersionError> {
    let mut parts = version.split('.');
    let major = parts
        .next()
        .ok_or_else(|| VersionError::InvalidVersion(version.to_string()))?
        .parse::<u64>()
        .map_err(|_| VersionError::InvalidVersion(version.to_string()))?;
    let minor = parts
        .next()
        .ok_or_else(|| VersionError::InvalidVersion(version.to_string()))?
        .parse::<u64>()
        .map_err(|_| VersionError::InvalidVersion(version.to_string()))?;
    let patch = parts
        .next()
        .ok_or_else(|| VersionError::InvalidVersion(version.to_string()))?
        .parse::<u64>()
        .map_err(|_| VersionError::InvalidVersion(version.to_string()))?;

    if parts.next().is_some() {
        return Err(VersionError::InvalidVersion(version.to_string()));
    }

    Ok((major, minor, patch))
}

// tag::semver_gate[]
pub fn classify_change(current: &str, next: &str) -> Result<VersionChange, VersionError> {
    let (current_major, _, _) = parse_semver(current)?;
    let (next_major, _, _) = parse_semver(next)?;

    if next_major > current_major {
        Ok(VersionChange::Breaking)
    } else {
        Ok(VersionChange::Compatible)
    }
}
// end::semver_gate[]
