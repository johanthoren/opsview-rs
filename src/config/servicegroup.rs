use super::{ServiceCheck, ServiceCheckRef};
use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Represents a [Service Group](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/service-checks-and-host/service-groups/index.html#Heading-overview) in Opsview.
///
/// Service groups are used to group [`ServiceCheck`] objects and serves as a way to organize
/// them into logical groups.
///
/// This struct defines the structure for a `ServiceGroup` entity as used in Opsview.
///
/// # Example
/// ```rust
/// use opsview::config::ServiceGroup;
/// use opsview::prelude::*;
///
/// let service_group = ServiceGroup::builder()
///  .name("My Service Group")
///  .build()
///  .unwrap();
///
///  assert_eq!(service_group.name, "My Service Group".to_string());
/// ```
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ServiceGroup {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the `ServiceGroup`.
    pub name: String,

    // Optional fields ---------------------------------------------------------------------------//
    /// Optional alias for the `ServiceGroup`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    /// [`ConfigRefMap`] of [`ServiceCheckRef`] objects associated with this `ServiceGroup`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servicechecks: Option<ConfigRefMap<ServiceCheckRef>>,

    // Read-only fields --------------------------------------------------------------------------//
    /// The unique identifier of the `ServiceGroup`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,

    /// A reference string unique to this `ServiceGroup`.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    /// A boolean indicating whether the `ServiceGroup` is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

/// Enables the creation of a [`ServiceGroup`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for ServiceGroup {}

impl ConfigObject for ServiceGroup {
    type Builder = ServiceGroupBuilder;

    /// Returns a builder for constructing a [`ServiceGroup`] object.
    ///
    /// # Returns
    /// A [`ServiceGroupBuilder`] object.
    fn builder() -> Self::Builder {
        ServiceGroupBuilder::new()
    }

    /// Provides the configuration path for a [`ServiceGroup`] object within the Opsview system.
    ///
    /// # Returns
    /// A string representing the API path where service groups are configured.
    fn config_path() -> Option<String> {
        Some("/config/servicegroup".to_string())
    }

    /// Creates a minimal [`ServiceGroup`] object with only the name set.
    ///
    /// # Arguments
    /// * `name` - Name of the [`ServiceGroup`].
    ///
    /// # Returns
    /// A minimal [`ServiceGroup`] object with only the name set, and the rest of the fields in
    /// their default states.
    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            name: validate_and_trim_servicegroup_name(name)?,
            ..Default::default()
        })
    }

    /// Returns the unique name of the [`ServiceGroup`] object.
    ///
    /// This name is used to identify the `ServiceGroup` when building the `HashMap` for an
    /// [`ConfigObjectMap`].
    ///
    /// # Returns
    /// A string representing the unique name of the `ServiceGroup`.
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl Persistent for ServiceGroup {
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
        Some(SERVICEGROUP_NAME_REGEX_STR.to_string())
    }

    fn validated_name(&self, name: &str) -> Result<String, OpsviewConfigError> {
        validate_and_trim_servicegroup_name(name)
    }

    fn set_name(&mut self, new_name: &str) -> Result<String, OpsviewConfigError> {
        self.name = self.validated_name(new_name)?;
        Ok(self.name.clone())
    }

    fn clear_readonly(&mut self) {
        self.id = None;
        self.ref_ = None;
        self.uncommitted = None;
    }
}

impl PersistentMap for ConfigObjectMap<ServiceGroup> {
    fn config_path() -> Option<String> {
        Some("/config/servicegroup".to_string())
    }
}

/// Builder for creating instances of [`ServiceGroup`].
///
/// Provides a fluent interface for constructing a `ServiceGroup` object with optional fields.
#[derive(Clone, Debug, Default)]
pub struct ServiceGroupBuilder {
    // Required fields ---------------------------------------------------------------------------//
    name: Option<String>,
    // Optional fields ---------------------------------------------------------------------------//
    alias: Option<String>,
    servicechecks: Option<ConfigRefMap<ServiceCheckRef>>,
}

impl Builder for ServiceGroupBuilder {
    type ConfigObject = ServiceGroup;

    /// Creates a new [`ServiceGroupBuilder`] instance used to construct a [`ServiceGroup`] object.
    ///
    /// # Returns
    /// A `ServiceGroupBuilder` instance.
    fn new() -> Self {
        Self::default()
    }

    /// Sets the name for the `ServiceGroup`.
    ///
    /// # Arguments
    /// * `name` - The name for the `ServiceGroup`.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Builds the [`ServiceGroup`] object with the configured settings.
    ///
    /// # Returns
    /// A `ServiceGroup` object.
    ///
    /// # Errors
    /// Returns an error if the name is not set.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;
        let validated_alias = validate_opt_string(self.alias, validate_and_trim_description)?;

        Ok(ServiceGroup {
            name: validate_and_trim_servicegroup_name(&name)?,
            alias: validated_alias,
            servicechecks: self.servicechecks,
            id: None,
            ref_: None,
            uncommitted: None,
        })
    }
}

impl ServiceGroupBuilder {
    /// Sets the alias for the `ServiceGroup`.
    ///
    /// # Arguments
    /// * `alias` - The alias for the `ServiceGroup`.
    pub fn alias(mut self, alias: &str) -> Self {
        self.alias = Some(alias.to_string());
        self
    }

    /// Clears the `alias` field.
    pub fn clear_alias(mut self) -> Self {
        self.alias = None;
        self
    }

    /// Clears the `name` field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Clears the `servicechecks` field.
    pub fn clear_servicechecks(mut self) -> Self {
        self.servicechecks = None;
        self
    }

    /// Sets the collection of `ServiceCheck` objects for the `ServiceGroup`.
    ///
    /// # Arguments
    /// * `servicechecks` - A reference to a [`ConfigObjectMap`] of [`ServiceCheck`] objects for the `ServiceGroup`.
    pub fn servicechecks(mut self, servicechecks: &ConfigObjectMap<ServiceCheck>) -> Self {
        self.servicechecks = Some(servicechecks.into());
        self
    }
}

/// A reference version of [`ServiceGroup`] that is used when passing or retrieving a
/// [`ServiceGroup`] object as part of another object.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct ServiceGroupRef {
    name: String,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
}

/// Enables the creation of a [`ServiceGroupRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for ServiceGroupRef {}

impl ConfigRef for ServiceGroupRef {
    type FullObject = ServiceGroup;

    /// Returns the reference string of the [`ServiceGroupRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`ServiceGroupRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`ServiceGroupRef`] object.
    ///
    /// This name is used to identify the `ServiceGroupRef` when building the `HashMap` for a
    /// [`ConfigRefMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl From<ServiceGroup> for ServiceGroupRef {
    /// Creates a [`ServiceGroupRef`] object from a [`ServiceGroup`] object.
    ///
    /// # Arguments
    /// * `service_group` - A [`ServiceGroup`] object.
    ///
    /// # Returns
    /// A `ServiceGroupRef` object.
    fn from(service_group: ServiceGroup) -> Self {
        Self {
            name: service_group.name.clone(),
            ref_: service_group.ref_.clone(),
        }
    }
}

impl From<Arc<ServiceGroup>> for ServiceGroupRef {
    fn from(item: Arc<ServiceGroup>) -> Self {
        let cmd: ServiceGroup = Arc::try_unwrap(item).unwrap_or_else(|arc| (*arc).clone());
        ServiceGroupRef::from(cmd)
    }
}

impl From<&ConfigObjectMap<ServiceGroup>> for ConfigRefMap<ServiceGroupRef> {
    fn from(groups: &ConfigObjectMap<ServiceGroup>) -> Self {
        ref_map_from(groups)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let service_group = ServiceGroup::default();

        assert_eq!(service_group.name, "".to_string());
    }

    #[test]
    fn test_minimal() {
        let service_group = ServiceGroup::minimal("my group");

        assert_eq!(service_group.unwrap().name, "my group".to_string());
    }
}
