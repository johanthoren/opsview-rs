use super::{Host, HostRef, ManagementURL, ServiceCheck, ServiceCheckHostRef};
use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Represents a [HostTemplate](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/service-checks-and-host/host-templates/index.html#Heading-overview) in Opsview.
///
/// Host templates are used to define a set of shared [`ServiceCheck`]s and [`ManagementURL`]s that can be
/// applied to multiple [`Host`]s. This allows users to define a set of common checks and URLs that can
/// be applied to multiple hosts, rather than having to define them individually for each host.
///
/// This struct defines the structure for a host template entity as used in Opsview.
///
/// # Example
/// ```rust
/// use opsview::config::HostTemplate;
/// use opsview::prelude::*;
///
/// let host_template = HostTemplate::builder()
///    .name("My Host Template")
///    .build()
///    .unwrap();
///
/// assert_eq!(host_template.name, "My Host Template");
/// ```    
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct HostTemplate {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the `HostTemplate`.
    pub name: String,

    // Optional fields ---------------------------------------------------------------------------//
    /// Optional description of the `HostTemplate`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Unix Timestamp indicating when the icon was last updated, or 0 if there is no icon.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub has_icon: Option<u64>,

    /// [`ConfigRefMap`] of [`HostRef`] objects associated with this `HostTemplate`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: Option<ConfigRefMap<HostRef>>,

    /// [`ConfigObjectMap`] of [`ManagementURL`]s associated with this `HostTemplate`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managementurls: Option<ConfigObjectMap<ManagementURL>>,

    /// [`ConfigRefMap`] of [`ServiceCheckHostRef`] objects associated with this `HostTemplate`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servicechecks: Option<ConfigRefMap<ServiceCheckHostRef>>,

    // Read-only fields --------------------------------------------------------------------------//
    /// The unique identifier of the `HostTemplate`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,

    /// A reference string unique to this template.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    /// A boolean indicating whether the `HostTemplate` is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

/// Enables the creation of a [`HostTemplate`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for HostTemplate {}

impl ConfigObject for HostTemplate {
    type Builder = HostTemplateBuilder;

    /// Returns a builder for constructing a [`HostTemplate`] object.
    ///
    /// # Returns
    /// A [`HostTemplateBuilder`] object.
    fn builder() -> Self::Builder {
        HostTemplateBuilder::new()
    }

    /// Provides the configuration path for a [`HostTemplate`] object within the Opsview system.
    ///
    /// # Returns
    /// A string representing the API path where host templates are configured.
    fn config_path() -> Option<String> {
        Some("/config/hosttemplate".to_string())
    }

    /// Returns a minimal `HostTemplate` object with only the name set.
    ///
    /// # Arguments
    /// * `name` - Name of the [`HostTemplate`].
    ///
    /// # Returns
    /// A Result containing a minimal `HostTemplate` object with only the name set, and
    /// the rest of the fields in their default states.
    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            name: validate_and_trim_hosttemplate_name(name)?,
            ..Default::default()
        })
    }

    /// Returns the unique name of the [`HostTemplate`] object.
    /// This name is used to identify the `HostTemplate` when building the `HashMap` for an
    /// [`ConfigObjectMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl Persistent for HostTemplate {
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
        Some(HOSTTEMPLATE_NAME_REGEX_STR.to_string())
    }

    fn validated_name(&self, name: &str) -> Result<String, OpsviewConfigError> {
        validate_and_trim_hosttemplate_name(name)
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

impl PersistentMap for ConfigObjectMap<HostTemplate> {
    fn config_path() -> Option<String> {
        Some("/config/hosttemplate".to_string())
    }
}

/// Builder for creating instances of [`HostTemplate`].
///
/// Provides a fluent interface for constructing a `HostTemplate` object with optional fields.
#[derive(Clone, Debug, Default)]
pub struct HostTemplateBuilder {
    name: Option<String>,
    description: Option<String>,
    has_icon: Option<u64>,
    hosts: Option<ConfigRefMap<HostRef>>,
    managementurls: Option<ConfigObjectMap<ManagementURL>>,
    servicechecks: Option<ConfigRefMap<ServiceCheckHostRef>>,
}

impl Builder for HostTemplateBuilder {
    type ConfigObject = HostTemplate;

    /// Creates a new instance of [`HostTemplateBuilder`] with default values. Initializes a new
    /// builder for creating a [`HostTemplate`] object with all fields in their default state.
    fn new() -> Self {
        HostTemplateBuilder::default()
    }

    /// Sets the name field.
    ///
    /// # Arguments
    /// * `name` - The name of the `HostTemplate`.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Consumes the builder and returns a [`HostTemplate`] object.
    ///
    /// # Returns
    /// A `HostTemplate` object with the values specified by the builder.
    ///
    /// # Errors
    /// Returns an error if the name field is not set.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;

        let validated_description =
            validate_opt_string(self.description, validate_and_trim_description)?;

        if self
            .has_icon
            .is_some_and(|ts| !is_valid_past_unix_timestamp(ts))
        {
            return Err(OpsviewConfigError::InvalidTimestamp(format!(
                "has_icon timestamp is in the future: {}",
                self.has_icon.unwrap()
            )));
        }

        Ok(HostTemplate {
            name: validate_and_trim_hosttemplate_name(&name)?,
            description: validated_description,
            has_icon: self.has_icon,
            hosts: self.hosts,
            managementurls: self.managementurls,
            servicechecks: self.servicechecks,
            id: None,
            ref_: None,
            uncommitted: None,
        })
    }
}

impl HostTemplateBuilder {
    /// Clears the description field.
    pub fn clear_description(mut self) -> Self {
        self.description = None;
        self
    }

    /// Clears the has_icon field.
    pub fn clear_has_icon(mut self) -> Self {
        self.has_icon = None;
        self
    }

    /// Clears the hosts field.
    pub fn clear_hosts(mut self) -> Self {
        self.hosts = None;
        self
    }

    /// Clears the managementurls field.
    pub fn clear_managementurls(mut self) -> Self {
        self.managementurls = None;
        self
    }

    /// Clears the name field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Clears the servicechecks field.
    pub fn clear_servicechecks(mut self) -> Self {
        self.servicechecks = None;
        self
    }

    /// Sets the description field.
    ///
    /// # Arguments
    /// * `description` - The description of the `HostTemplate`.
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Sets the has_icon field.
    ///
    /// # Arguments
    /// * `has_icon` - A Unix timestamp `u64` indicating when the icon was set, or 0 for no icon.
    pub fn has_icon(mut self, has_icon: u64) -> Self {
        self.has_icon = Some(has_icon);
        self
    }

    /// Sets the hosts field.
    ///
    /// # Arguments
    /// * `hosts` - A reference to a [`ConfigObjectMap`] of [`Host`] objects associated with this `HostTemplate`.
    pub fn hosts(mut self, hosts: &ConfigObjectMap<Host>) -> Self {
        self.hosts = Some(hosts.into());
        self
    }

    /// Sets the managementurls field.
    ///
    /// # Arguments
    /// * `managementurls` - [`ConfigObjectMap`] of [`ManagementURL`]s associated with this `HostTemplate`.
    pub fn managementurls(mut self, managementurls: ConfigObjectMap<ManagementURL>) -> Self {
        self.managementurls = Some(managementurls);
        self
    }

    /// Sets the servicechecks field.
    ///
    /// # Arguments
    /// * `servicechecks` - A reference to a [`ConfigObjectMap`] of [`ServiceCheck`] objects associated with this `HostTemplate`.
    pub fn servicechecks(mut self, servicechecks: &ConfigObjectMap<ServiceCheck>) -> Self {
        self.servicechecks = Some(servicechecks.into());
        self
    }
}

/// A reference version of [`HostTemplate`] that is used when passing or retrieving a
/// [`HostTemplate`] object as part of another object.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct HostTemplateRef {
    name: String,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
}

/// Enables the creation of a [`HostTemplateRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for HostTemplateRef {}

impl ConfigRef for HostTemplateRef {
    type FullObject = HostTemplate;

    /// Returns the reference string of the [`HostTemplateRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`HostTemplateRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`HostTemplateRef`] object.
    /// This name is used to identify the `HostTemplateRef` when building the `HashMap` for a
    /// [`ConfigRefMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl From<HostTemplate> for HostTemplateRef {
    /// Creates a [`HostTemplateRef`] object from a full [`HostTemplate`] object.
    ///
    /// # Arguments
    /// * `host_template` - A full [`HostTemplate`] object.
    ///
    /// # Returns
    /// A [`HostTemplateRef`] object with the same name and reference string as the full
    /// [`HostTemplate`] object.
    fn from(host_template: HostTemplate) -> Self {
        Self {
            name: host_template.name.clone(),
            ref_: host_template.ref_.clone(),
        }
    }
}

impl From<Arc<HostTemplate>> for HostTemplateRef {
    fn from(item: Arc<HostTemplate>) -> Self {
        let cmd: HostTemplate = Arc::try_unwrap(item).unwrap_or_else(|arc| (*arc).clone());
        HostTemplateRef::from(cmd)
    }
}

impl From<&ConfigObjectMap<HostTemplate>> for ConfigRefMap<HostTemplateRef> {
    fn from(host_templates: &ConfigObjectMap<HostTemplate>) -> Self {
        ref_map_from(host_templates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let template = HostTemplate::default();

        assert!(template.name.is_empty());
    }

    #[test]
    fn test_minimal() {
        let template = HostTemplate::minimal("My Host Template");

        assert_eq!(template.unwrap().name, "My Host Template".to_string());
    }

    #[test]
    fn test_thin_from_full_host_template() {
        let obj = HostTemplate {
            name: "My Host Template".to_string(),
            description: Some("My Host Template Description".to_string()),
            has_icon: Some(0),
            hosts: None,
            managementurls: None,
            servicechecks: None,
            id: Some(1),
            ref_: Some("my-host-template-ref".to_string()),
            uncommitted: Some(false),
        };

        let obj_ref = HostTemplateRef::from(obj.clone());

        assert_eq!(obj_ref.name, obj.name);
        assert_eq!(obj_ref.ref_, obj.ref_);
    }
}
