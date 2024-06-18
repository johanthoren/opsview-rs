use crate::prelude::*;
use serde::{Deserialize, Serialize};

// Example SNMPInterfaces JSON:
//
// "snmpinterfaces" : [
//    {
//       "active" : "0",
//       "discards_critical" : "15",
//       "discards_warning" : null,
//       "duplicatename" : "0",
//       "errors_critical" : "10",
//       "errors_warning" : null,
//       "indexid" : "0",
//       "interfacename" : "",
//       "throughput_critical" : "50%",
//       "throughput_warning" : null
//    },
//    {
//       "active" : "0",
//       "discards_critical" : "",
//       "discards_warning" : "",
//       "duplicatename" : "0",
//       "errors_critical" : "",
//       "errors_warning" : "",
//       "indexid" : "0",
//       "interfacename" : "eth0",
//       "throughput_critical" : "",
//       "throughput_warning" : ""
//    },
// ],

/// SNMPInterface represents a single SNMP interface configuration.
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SNMPInterface {
    /// Enables or disables the SNMP interface.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub active: Option<bool>,

    /// The critical threshold for interface discards.
    pub discards_critical: Option<String>,

    /// The warning threshold for interface discards.
    pub discards_warning: Option<String>,

    /// TODO: Verify the meaning of this field.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub duplicatename: Option<bool>,

    /// The critical threshold for interface errors.
    pub errors_critical: Option<String>,

    /// The warning threshold for interface errors.
    pub errors_warning: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]

    /// The index ID of the interface.
    pub indexid: Option<u64>,

    /// The name of the interface.
    pub interfacename: Option<String>,

    /// The critical threshold for interface throughput.
    pub throughput_critical: Option<String>,

    /// The warning threshold for interface throughput.
    pub throughput_warning: Option<String>,
}

/// SNMPInterfaces is a list of SNMPInterface objects.
///
/// This type is used to represent the `snmpinterfaces` field in the Opsview JSON configuration.
///
/// The first SNMPInterface object in the list represents the default settings for all interfaces
/// and must have an `indexid` of 0 and an empty `interfacename`. This row is required when
/// submitting the list of interfaces to the Opsview API server.
pub type SNMPInterfaces = Vec<SNMPInterface>;

/// SNMPInterfacesExt provides additional methods for the SNMPInterfaces type.
pub trait SNMPInterfacesExt {
    /// Validates the SNMPInterfaces object.
    fn validate(&self) -> Result<(), OpsviewConfigError>;
}

impl SNMPInterfacesExt for SNMPInterfaces {
    fn validate(&self) -> Result<(), OpsviewConfigError> {
        if !self.is_empty() && self[0].interfacename != Some("".to_string()) {
            return Err(OpsviewConfigError::SNMPInterfacesIndex0InvalidName);
        }

        // TODO: Add additional validation logic.

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_name_for_index_0() {
        let mut i: SNMPInterface = Default::default();
        i.interfacename = Some("eth0".to_string());
        i.indexid = Some(0);
        let i = i;

        let mut interfaces = SNMPInterfaces::new();
        interfaces.push(i);
        let interfaces = interfaces;

        let result = interfaces.validate();

        match result {
            Err(OpsviewConfigError::SNMPInterfacesIndex0InvalidName) => (),
            _ => panic!("Expected SNMPInterfacesIndex0InvalidName error"),
        }
    }
}
