#![allow(missing_docs)]
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SNMPVersion {
    #[serde(rename = "1")]
    V1,
    #[serde(rename = "2c")]
    V2c,
    #[serde(rename = "3", deserialize_with = "deserialize_snmpv3")]
    V3(SNMPV3SecurityLevel),
}

/// Custom deserialization for SNMPv3 security levels.
/// Defaults to `NoAuthNoPriv`.
fn deserialize_snmpv3<'de, D>(deserializer: D) -> Result<SNMPV3SecurityLevel, D::Error>
where
    D: Deserializer<'de>,
{
    let version_str: String = Deserialize::deserialize(deserializer)?;

    match version_str.as_str() {
        "3" => Ok(SNMPV3SecurityLevel::NoAuthNoPriv),
        _ => Err(serde::de::Error::custom("Invalid SNMP version for V3")),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum SNMPV3AuthProtocol {
    MD5,
    SHA,
    #[serde(rename = "")]
    Unspecified,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum SNMPV3PrivProtocol {
    DES,
    AES,
    #[serde(rename = "")]
    Unspecified,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum SNMPV3SecurityLevel {
    #[serde(rename = "noAuthNoPriv")]
    NoAuthNoPriv,
    #[serde(rename = "authNoPriv")]
    AuthNoPriv(SNMPV3AuthProtocol),
    #[serde(rename = "authPriv")]
    AuthPriv(SNMPV3AuthProtocol, SNMPV3PrivProtocol),
}
