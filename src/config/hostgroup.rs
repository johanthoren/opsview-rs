use super::{Host, HostRef};
use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Represents a [Host Group](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/hosts-groups/hosts-groups-concept/index.html#overview) in Opsview.
///
/// Host groups are used to organize and categorize hosts, allowing for structured monitoring and
/// management. A [`Host`] can only belong to one host group, but host groups can be nested to
/// create a hierarchical structure. Only leaf nodes in the hierarchy can have hosts assigned to
/// them.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct HostGroup {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the `HostGroup`.
    pub name: String,

    // Optional fields ---------------------------------------------------------------------------//
    /// [`ConfigRefMap`] of child `HostGroupRef` objects, representing a hierarchical structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<ConfigRefMap<HostGroupRef>>,

    /// [`ConfigRefMap`] of [`HostRef`] objects that belong to this `HostGroup`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: Option<ConfigRefMap<HostRef>>,

    /// The parent `HostGroupRef` of this group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<HostGroupRef>,

    // Read-only fields --------------------------------------------------------------------------//
    /// The unique identifier of the `HostGroup`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,

    /// A boolean indicating whether this `HostGroup` is a leaf node (i.e., has no children).
    /// Only leaf nodes can have hosts assigned to them.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub is_leaf: Option<bool>,

    /// The materialized path representing the group's position in the hierarchy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matpath: Option<String>,

    /// A reference string unique to this `HostGroup`.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    /// A boolean indicating whether the `HostGroup` is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

/// Enables the creation of a [`HostGroup`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for HostGroup {}

impl ConfigObject for HostGroup {
    type Builder = HostGroupBuilder;

    /// Returns a builder for constructing a [`HostGroup`] object.
    ///
    /// # Returns
    /// A [`HostGroupBuilder`] object.
    fn builder() -> Self::Builder {
        HostGroupBuilder::new()
    }

    /// Provides the configuration path for a [`HostGroup`] object within the Opsview system.
    ///
    /// # Returns
    /// A string representing the API path where host groups are configured.
    fn config_path() -> Option<String> {
        Some("/config/hostgroup".to_string())
    }

    /// Returns a minimal `HostGroup` object with only the name set.
    ///
    /// # Arguments
    /// * `name` - Name of the [`HostGroup`].
    ///
    /// # Returns
    /// A minimal `HostGroup` object with only the name set, and the rest of the fields in their
    /// default states.
    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            name: validate_and_trim_hostgroup_name(name)?,
            ..Default::default()
        })
    }

    /// Returns the unique name of the [`HostGroup`] object. In this case the matpath, since they
    /// are unique while hostgroup names are not.
    ///
    /// This name is used to identify the `HostGroup` when building the `HashMap` for an
    /// [`ConfigObjectMap`].
    fn unique_name(&self) -> String {
        if let Some(matpath) = &self.matpath {
            matpath.to_string()
        } else {
            self.name.clone()
        }
    }
}

impl Persistent for HostGroup {
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
        Some(HOSTGROUP_NAME_REGEX_STR.to_string())
    }

    fn validated_name(&self, name: &str) -> Result<String, OpsviewConfigError> {
        validate_and_trim_hostgroup_name(name)
    }

    fn set_name(&mut self, new_name: &str) -> Result<String, OpsviewConfigError> {
        self.name = self.validated_name(new_name)?;
        Ok(self.name.clone())
    }

    fn clear_readonly(&mut self) {
        self.id = None;
        self.is_leaf = None;
        self.matpath = None;
        self.ref_ = None;
        self.uncommitted = None;
    }
}

/// Builder for creating instances of [`HostGroup`].
///
/// Provides a fluent interface for constructing a `HostGroup` object. This approach allows for
/// customizable construction, ensuring that the created object conforms to the required parameters
/// and defaults.
///
/// # Examples
/// ```
/// use opsview::config::HostGroup;
/// use opsview::prelude::*;
/// let existing_parent = HostGroup::minimal("Opsview").unwrap();
///
/// let builder = HostGroup::builder()
///     .name("Example Group")
///     .parent(existing_parent)
///     .build()
///     .unwrap();
///
/// assert_eq!(builder.name, "Example Group".to_string());
/// ```
#[derive(Clone, Debug, Default)]
pub struct HostGroupBuilder {
    children: Option<ConfigRefMap<HostGroupRef>>,
    hosts: Option<ConfigRefMap<HostRef>>,
    name: Option<String>,
    parent: Option<HostGroupRef>,
}

impl Builder for HostGroupBuilder {
    type ConfigObject = HostGroup;

    /// Initializes a new builder for creating a [`HostGroup`] object with all fields in their default
    /// state.
    ///
    /// # Returns
    /// A new instance of [`HostGroupBuilder`].
    fn new() -> Self {
        HostGroupBuilder::default()
    }

    /// Sets the name field.
    ///
    /// # Arguments
    /// * `name` - The name of the `HostGroup`.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Completes the construction of a [`HostGroup`] object.
    ///
    /// Validates the current state of the [`HostGroupBuilder`] and assembles a `HostGroup` object.
    ///
    /// # Errors
    /// Returns an error if the name is not set.
    ///
    /// # Returns
    /// On success, returns a `Result` containing the constructed `HostGroup` object.
    /// On failure, returns an error detailing the missing field or inconsistency.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;

        Ok(HostGroup {
            name: validate_and_trim_hostgroup_name(&name)?,
            children: self.children,
            parent: self.parent,
            hosts: self.hosts,
            matpath: None,
            id: None,
            is_leaf: None,
            ref_: None,
            uncommitted: None,
        })
    }
}

impl HostGroupBuilder {
    /// Sets the children field.
    ///
    /// # Arguments
    /// * `children` - The children of the `HostGroup` as a collection of `HostGroup` objects.
    pub fn children(mut self, children: &ConfigObjectMap<HostGroup>) -> Self {
        self.children = Some(children.into());
        self
    }

    /// Clears the children field.
    pub fn clear_children(mut self) -> Self {
        self.children = None;
        self
    }

    /// Clears the hosts field.
    pub fn clear_hosts(mut self) -> Self {
        self.hosts = None;
        self
    }

    /// Clears the name field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Clears the parent field.
    pub fn clear_parent(mut self) -> Self {
        self.parent = None;
        self
    }

    /// Sets the hosts field.
    ///
    /// # Arguments
    /// * `hosts` - A reference to a [`ConfigObjectMap`] of [`Host`] objects that belong to this `HostGroup`.
    pub fn hosts(mut self, hosts: &ConfigObjectMap<Host>) -> Self {
        self.hosts = Some(hosts.into());
        self
    }

    /// Sets the parent field.
    ///
    /// # Arguments
    /// * `parent` - The parent of the `HostGroup`.
    pub fn parent(mut self, parent: HostGroup) -> Self {
        self.parent = Some(HostGroupRef::from(parent));
        self
    }
}

/// A reference version of [`HostGroup`] that is used when passing or retrieving a [`HostGroup`]
/// object as part of another object.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct HostGroupRef {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    matpath: Option<String>,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
}

/// Enables the creation of a [`HostGroupRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for HostGroupRef {}

impl ConfigRef for HostGroupRef {
    type FullObject = HostGroup;

    /// Returns the reference string of the [`HostGroupRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`HostGroupRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`HostGroupRef`] object.
    /// This name is used to identify the `HostGroupRef` when building the `HashMap` for a
    /// [`ConfigRefMap`].
    fn unique_name(&self) -> String {
        if let Some(matpath) = &self.matpath {
            matpath.to_string()
        } else {
            self.name.clone()
        }
    }
}

impl PersistentMap for ConfigObjectMap<HostGroup> {
    fn config_path() -> Option<String> {
        Some("/config/hostgroup".to_string())
    }
}

impl From<HostGroup> for HostGroupRef {
    fn from(hostgroup: HostGroup) -> Self {
        HostGroupRef {
            name: hostgroup.name.clone(),
            matpath: hostgroup.matpath.clone(),
            ref_: hostgroup.ref_.clone(),
        }
    }
}

impl From<Arc<HostGroup>> for HostGroupRef {
    fn from(item: Arc<HostGroup>) -> Self {
        let hostgroup: HostGroup = Arc::try_unwrap(item).unwrap_or_else(|arc| (*arc).clone());
        HostGroupRef::from(hostgroup)
    }
}

impl From<&ConfigObjectMap<HostGroup>> for ConfigRefMap<HostGroupRef> {
    fn from(host_groups: &ConfigObjectMap<HostGroup>) -> Self {
        ref_map_from(host_groups)
    }
}

impl HostGroupRef {
    /// Returns the materialized path of the [`HostGroupRef`].
    pub fn matpath(&self) -> Option<String> {
        self.matpath.clone()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_default() {
        let hostgroup = HostGroup::default();

        assert!(hostgroup.name.is_empty());
    }

    #[test]
    fn test_minimal() {
        let hostgroup = HostGroup::minimal("My HostGroup");

        assert_eq!(hostgroup.unwrap().name, "My HostGroup".to_string());
    }

    #[test]
    fn test_is_valid_hostgroup_name() {
        // Test valid names
        assert!(validate_and_trim_hostgroup_name("Host Group 1").is_ok());
        assert!(validate_and_trim_hostgroup_name("Another-Valid_HostGroup/Name+123").is_ok());
        assert!(validate_and_trim_hostgroup_name(&"A".repeat(128)).is_ok());

        // Test invalid names
        assert!(validate_and_trim_hostgroup_name("").is_err()); // Empty name
        assert!(validate_and_trim_hostgroup_name(&"A".repeat(129)).is_err());
        assert!(validate_and_trim_hostgroup_name("//foo").is_err());
        assert!(validate_and_trim_hostgroup_name("/").is_err());
    }
}
