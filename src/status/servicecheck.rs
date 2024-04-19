#![allow(missing_docs)]
use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct ServiceCheckStatusSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computed_state: Option<ServiceCheckState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical: Option<HandledCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<HandledCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok: Option<HandledCount>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub handled: Option<u64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub total: Option<u64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub unhandled: Option<u64>,
}

impl StatusSummary for ServiceCheckStatusSummary {
    fn handled(&self) -> u64 {
        self.critical.clone().unwrap_or_default().handled()
            + self.warning.clone().unwrap_or_default().handled()
            + self.ok.clone().unwrap_or_default().handled()
    }

    fn unhandled(&self) -> u64 {
        self.critical.clone().unwrap_or_default().unhandled()
            + self.warning.clone().unwrap_or_default().unhandled()
            + self.ok.clone().unwrap_or_default().unhandled()
    }

    fn total(&self) -> u64 {
        self.critical.clone().unwrap_or_default().total()
            + self.warning.clone().unwrap_or_default().total()
            + self.ok.clone().unwrap_or_default().total()
    }
}
