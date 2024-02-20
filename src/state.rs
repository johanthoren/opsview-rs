use serde::{Deserialize, Serialize};

/// Represents the
/// [state](https://docs.itrsgroup.com/docs/opsview/6.9.0/getting-started/important-concepts/index.html#states)
/// of a [`Host`](crate::config::Host) in Opsview.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum HostState {
    /// The host is in an UP state.
    #[serde(rename = "0")]
    Up,
    /// The host is in a DOWN state.
    #[serde(rename = "1")]
    Down,
    /// The host is in an UNREACHABLE state.
    #[serde(rename = "2")]
    Unreachable,
}

/// Represents the
/// [state](https://docs.itrsgroup.com/docs/opsview/6.9.0/getting-started/important-concepts/index.html#states)
/// of a [`ServiceCheck`](crate::config::ServiceCheck) in Opsview.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum ServiceCheckState {
    /// The service check is an OK state..
    #[serde(rename = "0")]
    Ok,
    /// The service check is in a WARNING state.
    #[serde(rename = "1")]
    Warning,
    /// The service check is in a CRITICAL state.
    #[serde(rename = "2")]
    Critical,
    /// The service check is in an UNKNOWN state.
    #[serde(rename = "3")]
    Unknown,
}
