use crate::prelude::*;
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

/// Represents the accounting of handled and unhandled counts for either a host or service.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct HandledCount {
    /// The number of handled counts.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub handled: Option<u64>,
    /// The number of unhandled counts.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub unhandled: Option<u64>,
}

impl HandledCount {
    /// Returns the total count of handled and unhandled counts.
    pub fn total(&self) -> u64 {
        self.handled.unwrap_or(0) + self.unhandled.unwrap_or(0)
    }
}
