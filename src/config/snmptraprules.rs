use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};

/// Represents an [SNMP trap](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/service-checks-and-host/snmp-traps/index.html) rule in Opsview.
///
/// The `SNMPTrapRule` struct defines the structure for an SNMP trap rule entity as used in
/// Opsview. SNMP trap rules are used to define how SNMP traps are processed by Opsview.
///
/// # Example
/// ```rust
/// use opsview::config::SNMPTrapRule;
/// use opsview::prelude::*;
///
/// let snmp_trap_rule = SNMPTrapRule::builder()
///    .name("My SNMPTrapRule")
///    .build()
///    .unwrap();
///
/// assert_eq!(snmp_trap_rule.name, "My SNMPTrapRule".to_string());
/// ```
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct SNMPTrapRule {
    // Required fields ---------------------------------------------------------------------------//
    // TODO: Add validation of this field.
    /// The unique name of the `SNMPTrapRule`.
    pub name: String,
    // Optional fields ---------------------------------------------------------------------------//
    // TODO: Add validation of this field.
    /// The alert level of the `SNMPTrapRule`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub alertlevel: Option<u64>,

    // TODO: Add validation of this field.
    /// The code of the `SNMPTrapRule`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    // TODO: Add validation of this field.
    /// The message of the `SNMPTrapRule`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// A boolean indicating whether the `SNMPTrapRule` is processed.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub process: Option<bool>,

    // Read-only fields --------------------------------------------------------------------------//
    /// The unique identifier of the `SNMPTrapRule`.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    /// A boolean indicating whether the `SNMPTrapRule` is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

/// Enables the creation of a [`SNMPTrapRule`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for SNMPTrapRule {}

impl ConfigObject for SNMPTrapRule {
    type Builder = SNMPTrapRuleBuilder;

    /// Returns a builder for constructing a [`SNMPTrapRule`] object.
    ///
    /// # Returns
    /// A [`SNMPTrapRuleBuilder`] object.
    fn builder() -> Self::Builder {
        SNMPTrapRuleBuilder::new()
    }

    /// Returns the unique name of the [`SNMPTrapRule`] object.
    ///
    /// This name is used to identify the `SNMPTrapRule` when building the `HashMap` for an
    /// `ConfigObjectMap`.
    ///
    /// # Returns
    /// A string representing the unique name of the variable.
    fn unique_name(&self) -> String {
        match self.ref_ {
            Some(ref ref_) => ref_.to_string(),
            None => self.name.clone(),
        }
    }
}

/// Builder for creating instances of [`SNMPTrapRule`].
///
/// Provides a fluent interface for constructing a `SNMPTrapRule` object with optional fields.
#[derive(Clone, Debug, Default)]
pub struct SNMPTrapRuleBuilder {
    // Required fields ---------------------------------------------------------------------------//
    name: Option<String>,
    // Optional fields ---------------------------------------------------------------------------//
    alertlevel: Option<u64>,
    code: Option<String>,
    message: Option<String>,
    process: Option<bool>,
}

impl Builder for SNMPTrapRuleBuilder {
    type ConfigObject = SNMPTrapRule;

    /// Creates a new [`SNMPTrapRuleBuilder`] object.
    fn new() -> Self {
        SNMPTrapRuleBuilder::default()
    }

    /// Sets the name of the `SNMPTrapRule` object.
    ///
    /// # Arguments
    /// * `name` - The name of the `SNMPTrapRule` object.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Builds a new [`SNMPTrapRule`] object using the configured values.
    ///
    /// # Returns
    /// A `SNMPTrapRule` object.
    ///
    /// # Errors
    /// Returns an `Error` if the `name` field is not set.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;

        Ok(SNMPTrapRule {
            name,
            alertlevel: self.alertlevel,
            code: self.code,
            message: self.message,
            process: self.process,
            ref_: None,
            uncommitted: None,
        })
    }
}

/// Builder implementation for [`SNMPTrapRule`].
///
/// Provides a fluent interface for constructing a `SNMPTrapRule` object with optional fields.
impl SNMPTrapRuleBuilder {
    /// Sets the alert level of the `SNMPTrapRule` object.
    ///
    /// # Arguments
    /// * `alertlevel` - The alert level of the `SNMPTrapRule` object.
    pub fn alertlevel(mut self, alertlevel: u64) -> Self {
        self.alertlevel = Some(alertlevel);
        self
    }

    /// Clears the `alertlevel` field.
    pub fn clear_alertlevel(mut self) -> Self {
        self.alertlevel = None;
        self
    }

    /// Clears the `code` field.
    pub fn clear_code(mut self) -> Self {
        self.code = None;
        self
    }

    /// Clears the `message` field.
    pub fn clear_message(mut self) -> Self {
        self.message = None;
        self
    }

    /// Clears the `process` field.
    pub fn clear_process(mut self) -> Self {
        self.process = None;
        self
    }

    /// Clears the `name` field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Sets the code of the `SNMPTrapRule` object.
    ///
    /// # Arguments
    /// * `code` - The code of the `SNMPTrapRule` object.
    pub fn code(mut self, code: &str) -> Self {
        self.code = Some(code.to_string());
        self
    }

    /// Sets the message of the `SNMPTrapRule` object.
    ///
    /// # Arguments
    /// * `message` - The message of the `SNMPTrapRule` object.
    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    /// Sets the process flag of the `SNMPTrapRule` object.
    ///
    /// # Arguments
    /// * `process` - The process flag of the `SNMPTrapRule` object.
    pub fn process(mut self, process: bool) -> Self {
        self.process = Some(process);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal() {
        let snmptraprule = SNMPTrapRule::minimal("My SNMPTrapRule");

        assert_eq!(snmptraprule.unwrap().name, "My SNMPTrapRule".to_string());
    }

    #[test]
    fn test_default() {
        let snmptraprule = SNMPTrapRule::default();

        assert!(snmptraprule.name.is_empty());
    }
}
