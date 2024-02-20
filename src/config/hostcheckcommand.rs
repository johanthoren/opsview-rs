use super::{HostRef, Plugin};
use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Represents a [Host check command](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/hosts-groups/host-check-commands/index.html) in Opsview.
///
/// Defines a command used to perform checks on [`super::Host`]s within the Opsview monitoring system.
/// `HostCheckCommand`s are essential for defining the monitoring behavior for each host.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct HostCheckCommand {
    // Required fields ---------------------------------------------------------------------------//
    /// The unique name of the `HostCheckCommand`.
    pub name: String,

    // Semi-optional fields ----------------------------------------------------------------------//
    // TODO: Add validation of this field.
    /// Arguments for the `HostCheckCommand`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<String>,

    /// The [`Plugin`] associated with the `HostCheckCommand`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<Plugin>,

    // Read-only fields --------------------------------------------------------------------------//
    /// [`ConfigRefMap`] of [`HostRef`] objects associated with this `HostCheckCommand`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: Option<ConfigRefMap<HostRef>>,

    /// The unique identifier of the `HostCheckCommand`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,

    /// The priority of the `HostCheckCommand`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub priority: Option<u64>,

    /// A reference string unique to this `HostCheckCommand`.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    /// A boolean indicating whether the `HostCheckCommand` is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

/// Enables the creation of a [`HostCheckCommand`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for HostCheckCommand {}

impl ConfigObject for HostCheckCommand {
    type Builder = HostCheckCommandBuilder;

    /// Returns a builder for constructing a [`HostCheckCommand`] object.
    ///
    /// # Returns
    /// A [`HostCheckCommandBuilder`] object.
    fn builder() -> Self::Builder {
        HostCheckCommandBuilder::new()
    }

    /// Provides the configuration path for a [`HostCheckCommand`] object within the Opsview system.
    ///
    /// # Returns
    /// A string representing the API path where `HostCheckCommand`s are configured.
    fn config_path() -> Option<String> {
        Some("/config/hostcheckcommand".to_string())
    }

    /// Returns a minimal `HostCheckCommand` object with only the name set.
    ///
    /// # Arguments
    /// * `name` - Name of the [`HostCheckCommand`].
    ///
    /// # Returns
    /// A minimal `HostCheckCommand` object with only the name set, and the rest of the
    /// fields in their default states.
    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            name: validate_and_trim_hostcheckcommand_name(name)?,
            ..Default::default()
        })
    }

    /// Returns the unique name of the [`HostCheckCommand`] object.
    ///
    /// This name is used to identify the `HostCheckCommand` when building the `HashMap` for an
    /// [`ConfigObjectMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl Persistent for HostCheckCommand {
    /// Returns the unique identifier.
    fn id(&self) -> Option<u64> {
        self.id
    }

    /// Returns the reference string if it's not empty.
    fn ref_(&self) -> Option<String> {
        if self.ref_.as_ref().is_some_and(|x| !x.is_empty()) {
            self.ref_.clone()
        } else {
            None
        }
    }

    /// Returns the name if it's not empty.
    fn name(&self) -> Option<String> {
        if self.name.is_empty() {
            None
        } else {
            Some(self.name.clone())
        }
    }

    fn name_regex(&self) -> Option<String> {
        Some(INLINE_FREE_TEXT_REGEX_STR.to_string())
    }

    fn validated_name(&self, name: &str) -> Result<String, OpsviewConfigError> {
        validate_and_trim_hostcheckcommand_name(name)
    }

    fn set_name(&mut self, new_name: &str) -> Result<String, OpsviewConfigError> {
        self.name = self.validated_name(new_name)?;
        Ok(self.name.clone())
    }

    fn clear_readonly(&mut self) {
        self.hosts = None;
        self.id = None;
        self.priority = None;
        self.ref_ = None;
        self.uncommitted = None;
    }
}

impl PersistentMap for ConfigObjectMap<HostCheckCommand> {
    fn config_path() -> Option<String> {
        Some("/config/hostcheckcommand".to_string())
    }
}

/// Builder for creating instances of [`HostCheckCommand`].
///
/// Provides a fluent interface for constructing a `HostCheckCommand` object, allowing for
/// customizable construction and ensuring that the created object conforms to required parameters
/// and defaults.
///
/// # Example
/// ```rust
/// use opsview::config::{HostCheckCommand, HostCheckCommandBuilder, Plugin};
/// use opsview::prelude::*;
///
/// let shiny_plugin = Plugin::builder()
///   .name("My shiny plugin")
///   .build()
///   .unwrap();
///   
/// let host_check_command = HostCheckCommand::builder()
///    .name("My `HostCheckCommand`")
///    .args("--help")
///    .plugin(shiny_plugin)
///    .build()
///    .unwrap();
///
///    assert_eq!(host_check_command.name, "My `HostCheckCommand`".to_string());
#[derive(Clone, Debug, Default)]
pub struct HostCheckCommandBuilder {
    // Required fields ---------------------------------------------------------------------------//
    name: Option<String>,
    // Semi-optional fields ----------------------------------------------------------------------//
    args: Option<String>,
    plugin: Option<Plugin>,
}

impl Builder for HostCheckCommandBuilder {
    type ConfigObject = HostCheckCommand;
    /// Creates a new instance of [`HostCheckCommandBuilder`] with default values.
    ///
    /// Initializes a new builder for creating a [`HostCheckCommand`] object with all fields in their
    /// default state.
    fn new() -> Self {
        HostCheckCommandBuilder::default()
    }

    /// Sets the name field.
    ///
    /// # Arguments
    /// * `name` - Name of the `HostCheckCommand`.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Completes the construction of a [`HostCheckCommand`] object.
    ///
    /// Validates the current state of the [`HostCheckCommandBuilder`] and assembles a
    /// `HostCheckCommand` object. If a required field is missing, an error is returned.
    ///
    /// # Returns
    /// On success, returns a `Result` containing the constructed `HostCheckCommand` object.
    /// On failure, returns an error detailing the missing field.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;
        let args = require_field(&self.args, "args")?;
        let plugin = require_field(&self.plugin, "plugin")?;

        Ok(HostCheckCommand {
            name: validate_and_trim_hostcheckcommand_name(&name)?,
            args: Some(args),
            plugin: Some(plugin),
            hosts: None,
            id: None,
            priority: None,
            ref_: None,
            uncommitted: None,
        })
    }
}

impl HostCheckCommandBuilder {
    /// Sets the arguments field.
    ///
    /// # Arguments
    /// * `args` - The arguments for the [`HostCheckCommand`].
    pub fn args(mut self, args: &str) -> Self {
        self.args = Some(args.to_string());
        self
    }

    /// Clears the args field.
    pub fn clear_args(mut self) -> Self {
        self.args = None;
        self
    }

    /// Clears the name field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Clears the plugin field.
    pub fn clear_plugin(mut self) -> Self {
        self.plugin = None;
        self
    }

    /// Sets the plugin field.
    ///
    /// # Arguments
    /// * `plugin` - [`Plugin`] associated with the `HostCheckCommand`.
    pub fn plugin(mut self, plugin: Plugin) -> Self {
        self.plugin = Some(plugin);
        self
    }
}

/// A reference version of [`HostCheckCommand`] that is used when passing or retrieving a
/// [`HostCheckCommand`] object as part of another object.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct HostCheckCommandRef {
    name: String,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
}

/// Enables the creation of a [`HostCheckCommandRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for HostCheckCommandRef {}

impl ConfigRef for HostCheckCommandRef {
    type FullObject = HostCheckCommand;

    /// Returns the reference string of the [`HostCheckCommandRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`HostCheckCommandRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`HostCheckCommandRef`] object.
    ///
    /// This name is used to identify the `HostCheckCommandRef` when building the `HashMap` for a
    /// [`ConfigRefMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl From<HostCheckCommand> for HostCheckCommandRef {
    fn from(host_check_command: HostCheckCommand) -> Self {
        Self {
            name: host_check_command.name.clone(),
            ref_: host_check_command.ref_.clone(),
        }
    }
}

impl From<Arc<HostCheckCommand>> for HostCheckCommandRef {
    fn from(item: Arc<HostCheckCommand>) -> Self {
        let cmd: HostCheckCommand = Arc::try_unwrap(item).unwrap_or_else(|arc| (*arc).clone());
        HostCheckCommandRef::from(cmd)
    }
}

impl From<&ConfigObjectMap<HostCheckCommand>> for ConfigRefMap<HostCheckCommandRef> {
    fn from(check_commands: &ConfigObjectMap<HostCheckCommand>) -> Self {
        ref_map_from(check_commands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal() {
        let host_check_command = HostCheckCommand::minimal("My `HostCheckCommand`");

        assert_eq!(
            host_check_command.unwrap().name,
            "My `HostCheckCommand`".to_string()
        );
    }

    #[test]
    fn test_default() {
        let host_check_command = HostCheckCommand::default();

        assert!(host_check_command.name.is_empty());
    }

    #[test]
    fn test_is_valid_hostcheckcommand_name() {
        // Valid names
        assert!(validate_and_trim_hostcheckcommand_name("My `HostCheckCommand`").is_ok());
        assert!(validate_and_trim_hostcheckcommand_name("My-HostCheckCommand").is_ok());
        assert!(validate_and_trim_hostcheckcommand_name("My_HostCheckCommand").is_ok());
        assert!(validate_and_trim_hostcheckcommand_name("My.HostCheckCommand").is_ok());
        assert!(validate_and_trim_hostcheckcommand_name("My:HostCheckCommand").is_ok());
        assert!(validate_and_trim_hostcheckcommand_name("My,HostCheckCommand").is_ok());
        assert!(validate_and_trim_hostcheckcommand_name("My'HostCheckCommand").is_ok());
        assert!(validate_and_trim_hostcheckcommand_name("My\"HostCheckCommand").is_ok());
        assert!(validate_and_trim_hostcheckcommand_name("My/HostCheckCommand").is_ok());
        assert!(validate_and_trim_hostcheckcommand_name("123").is_ok());
        assert!(validate_and_trim_hostcheckcommand_name(&"a".repeat(128)).is_ok());

        // Invalid names
        assert!(validate_and_trim_hostcheckcommand_name("").is_err());
        assert!(validate_and_trim_hostcheckcommand_name(" ").is_err());
        assert!(validate_and_trim_hostcheckcommand_name(&"a".repeat(129)).is_err());
    }
}
