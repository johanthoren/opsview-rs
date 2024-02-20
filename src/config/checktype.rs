use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// Represents a type of SNMP check in Opsview.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SNMPCheckType {
    /// [SNMP polling](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/service-checks-and-host/snmp-polling/index.html#Heading-what-is-snmp-polling) check.
    Polling,
    /// [SNMP trap](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/service-checks-and-host/snmp-traps/index.html#overview) check.
    Trap,
}

/// Represents [a type of Check](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/service-checks-and-host/intro-to-service-checks/index.html) in Opsview.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckType {
    /// [Active plugin check](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/service-checks-and-host/intro-to-service-checks/index.html#active-check)
    Active,
    /// [Passive check](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/service-checks-and-host/intro-to-service-checks/index.html#passive-check)
    Passive,
    /// [SNMP check](`SNMPCheckType`)
    SNMP(SNMPCheckType),
}

/// Enables the creation of a JSON representation of an [`SNMPCheckType`] instance.
impl Serialize for SNMPCheckType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (name, ref_) = match self {
            SNMPCheckType::Polling => ("SNMP Polling", "/rest/config/checktype/3"),
            SNMPCheckType::Trap => ("SNMP Trap", "/rest/config/checktype/4"),
        };
        let mut state = serializer.serialize_struct("SNMPCheckType", 2)?;
        state.serialize_field("name", name)?;
        state.serialize_field("ref", ref_)?;
        state.end()
    }
}

/// Enables the creation of a JSON representation of a [`CheckType`] instance.
impl Serialize for CheckType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CheckType::Active => {
                let mut state = serializer.serialize_struct("CheckType", 2)?;
                state.serialize_field("name", "Active Plugin")?;
                state.serialize_field("ref", "/rest/config/checktype/1")?;
                state.end()
            }
            CheckType::Passive => {
                let mut state = serializer.serialize_struct("CheckType", 2)?;
                state.serialize_field("name", "Passive")?;
                state.serialize_field("ref", "/rest/config/checktype/2")?;
                state.end()
            }
            CheckType::SNMP(snmp_check) => snmp_check.serialize(serializer),
        }
    }
}

/// Enables the creation a [`CheckType`] instance from a JSON representation.
impl<'de> Deserialize<'de> for CheckType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CheckTypeVisitor;

        impl<'de> Visitor<'de> for CheckTypeVisitor {
            type Value = CheckType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a JSON object for a check type")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CheckType, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut ref_ = None;

                while let Some(key) = map.next_key::<String>()? {
                    if key == "ref" {
                        ref_ = Some(map.next_value()?);
                    } else {
                        let _: serde_json::Value = map.next_value()?;
                    }
                }

                let ref_: String = ref_.ok_or_else(|| de::Error::missing_field("ref"))?;

                match ref_.as_str() {
                    "/rest/config/checktype/1" => Ok(CheckType::Active),
                    "/rest/config/checktype/2" => Ok(CheckType::Passive),
                    "/rest/config/checktype/3" => Ok(CheckType::SNMP(SNMPCheckType::Polling)),
                    "/rest/config/checktype/4" => Ok(CheckType::SNMP(SNMPCheckType::Trap)),
                    _ => Err(de::Error::custom(format!(
                        "unexpected check type ref: '{}'",
                        ref_
                    ))),
                }
            }
        }

        const FIELDS: &[&str] = &["ref"];
        deserializer.deserialize_struct("CheckType", FIELDS, CheckTypeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_snmp_polling_check_type() {
        let snmp_check = SNMPCheckType::Polling;
        let json = serde_json::to_string(&snmp_check).unwrap();
        assert_eq!(
            json,
            r#"{"name":"SNMP Polling","ref":"/rest/config/checktype/3"}"#
        );
    }

    #[test]
    fn test_serialize_snmp_trap_check_type() {
        let snmp_check = SNMPCheckType::Trap;
        let json = serde_json::to_string(&snmp_check).unwrap();
        assert_eq!(
            json,
            r#"{"name":"SNMP Trap","ref":"/rest/config/checktype/4"}"#
        );
    }

    #[test]
    fn test_serialize_active_plugin_check_type() {
        let check = CheckType::Active;
        let json = serde_json::to_string(&check).unwrap();
        assert_eq!(
            json,
            r#"{"name":"Active Plugin","ref":"/rest/config/checktype/1"}"#
        );
    }

    #[test]
    fn test_serialize_passive_check_type() {
        let check = CheckType::Passive;
        let json = serde_json::to_string(&check).unwrap();
        assert_eq!(
            json,
            r#"{"name":"Passive","ref":"/rest/config/checktype/2"}"#
        );
    }

    #[test]
    fn test_deserialize_active_check_type() {
        let json = r#"{"ref":"/rest/config/checktype/1","name":"Active Plugin"}"#;
        let check: CheckType = serde_json::from_str(json).unwrap();
        assert_eq!(check, CheckType::Active);
    }

    #[test]
    fn test_deserialize_passive_check_typ() {
        let json = r#"{"ref":"/rest/config/checktype/2", "name":"Passive"}"#;
        let check: CheckType = serde_json::from_str(json).unwrap();
        assert_eq!(check, CheckType::Passive);
    }

    #[test]
    fn test_deserialize_snmp_polling_check_type() {
        let json = r#"{"ref":"/rest/config/checktype/3", "name":"SNMP Polling"}"#;
        let check: CheckType = serde_json::from_str(json).unwrap();
        assert_eq!(check, CheckType::SNMP(SNMPCheckType::Polling));
    }

    #[test]
    fn test_deserialize_snmp_trap_check_type() {
        let json = r#"{"ref":"/rest/config/checktype/4", "name":"SNMP Trap"}"#;
        let check: CheckType = serde_json::from_str(json).unwrap();
        assert_eq!(check, CheckType::SNMP(SNMPCheckType::Trap));
    }
}
