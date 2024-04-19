use crate::prelude::*;
use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;

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
#[derive(Clone, Debug, Serialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
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

impl<'de> Deserialize<'de> for ServiceCheckState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringOrIntVisitor;

        impl<'de> Visitor<'de> for StringOrIntVisitor {
            type Value = ServiceCheckState;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or an int")
            }

            fn visit_str<E>(self, value: &str) -> Result<ServiceCheckState, E>
            where
                E: de::Error,
            {
                match value.to_lowercase().as_str() {
                    "0" | "ok" => Ok(ServiceCheckState::Ok),
                    "1" | "warning" => Ok(ServiceCheckState::Warning),
                    "2" | "critical" => Ok(ServiceCheckState::Critical),
                    "3" | "unknown" => Ok(ServiceCheckState::Unknown),
                    _ => Err(E::custom(format!("unexpected value: {}", value))),
                }
            }

            fn visit_i64<E>(self, value: i64) -> Result<ServiceCheckState, E>
            where
                E: de::Error,
            {
                match value {
                    0 => Ok(ServiceCheckState::Ok),
                    1 => Ok(ServiceCheckState::Warning),
                    2 => Ok(ServiceCheckState::Critical),
                    3 => Ok(ServiceCheckState::Unknown),
                    _ => Err(E::custom(format!("unexpected value: {}", value))),
                }
            }
        }

        deserializer.deserialize_any(StringOrIntVisitor)
    }
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

/// A trait for summarizing the state of a host or service.
pub trait StatusSummary:
    Clone + Default + Serialize + for<'a> Deserialize<'a> + Eq + PartialEq
{
    /// Returns the number of handled counts.
    fn handled(&self) -> u64;
    /// Returns the number of unhandled counts.
    fn unhandled(&self) -> u64;
    /// Returns the total count of handled and unhandled counts.
    fn total(&self) -> u64;
}

impl StatusSummary for HandledCount {
    /// Returns the number of handled counts.
    fn handled(&self) -> u64 {
        self.handled.unwrap_or(0)
    }

    /// Returns the number of unhandled counts.
    fn unhandled(&self) -> u64 {
        self.unhandled.unwrap_or(0)
    }

    /// Returns the total count of handled and unhandled counts.
    fn total(&self) -> u64 {
        self.handled.unwrap_or(0) + self.unhandled.unwrap_or(0)
    }
}
