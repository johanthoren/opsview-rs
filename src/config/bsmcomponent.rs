//! Contains the [`BSMComponent`] struct and implementations.
//!
//! In Opsview, a [Business Service Management
//! (BSM)](https://docs.itrsgroup.com/docs/opsview/current/monitoring/business-service-monitoring/business-service-monitoring/index.html)
//! Component is a building block for creating [BSM
//! Services](https://docs.itrsgroup.com/docs/opsview/current/monitoring/business-service-monitoring/create-business-service/index.html#create-a-business-service).

use super::{Host, HostRef, HostTemplate, HostTemplateRef};
use crate::{prelude::*, util::*};
use lazy_static::lazy_static;
use log::debug;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Represents a [Business Service Management
/// (BSM)](https://docs.itrsgroup.com/docs/opsview/current/monitoring/business-service-monitoring/business-service-monitoring/index.html)
/// Component in Opsview.
///
/// This struct encapsulates the data structure for a [BSM
/// Component](https://docs.itrsgroup.com/docs/opsview/current/monitoring/business-service-monitoring/create-business-service/index.html#create-a-component)
/// as used in Opsview. BSM Components are building blocks for creating [BSM
/// Services](https://docs.itrsgroup.com/docs/opsview/current/monitoring/business-service-monitoring/create-business-service/index.html#create-a-business-service).
///
/// # Example
/// ```rust
/// use opsview::config::{BSMComponent, Host, HostGroup, HostTemplate, MonitoringCluster};
/// use opsview::prelude::*;
///
/// let host_template = HostTemplate::builder()
///    .name("My Host Template")
///    .build()
///    .unwrap();
///
/// let mut host_templates = ConfigObjectMap::<HostTemplate>::new();
/// host_templates.add(host_template.clone());
///
/// // Shadowing host_templates to avoid mutable objects after adding the host template to the map.
/// let host_templates = host_templates;
///
/// let parent_group = HostGroup::minimal("Opsview")
///     .expect("Failed to create a minimal HostGroup with name 'Opsview'");
///
/// let host_group = HostGroup::builder()
///   .name("My Host Group")
///   .parent(parent_group)
///   .build()
///   .unwrap();
///
/// let cluster_1 = MonitoringCluster::minimal("Cluster 1")
///     .expect("Failed to create a minimal MonitoringCluster with name 'Cluster 1'");
///
/// let host = Host::builder()
///   .name("my_host")
///   .alias("My Host")
///   .ip("127.0.0.1")
///   .hostgroup(host_group)
///   .hosttemplates(&host_templates)
///   .monitored_by(cluster_1)
///   .build()
///   .unwrap();
///
/// let mut hosts = ConfigObjectMap::<Host>::new();
/// hosts.add(host);;
///
/// // Shadowing hosts to avoid mutable objects after adding the host to the map.
/// let hosts = hosts;
///
/// let bsm_component = BSMComponent::builder()
///  .name("My BSM Component")
///  .host_template(host_template)
///  .hosts(&hosts)
///  .quorum_pct("100.00")
///  .build()
///  .unwrap();
///
///  assert_eq!(bsm_component.name, "My BSM Component".to_string());
///  assert_eq!(bsm_component.hosts.as_ref().unwrap().len(), 1);
///  assert_eq!(bsm_component.hosts.unwrap().get("my_host").unwrap().name(), "my_host".to_string());
/// ```
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct BSMComponent {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the [`BSMComponent`].
    pub name: String,

    // Semi-optional fields ----------------------------------------------------------------------//
    // Required when building a new object, but not always present from the API, so optional for
    // serializing purposes.
    /// The [`HostTemplate`] associated with the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_template: Option<HostTemplateRef>,

    /// A unique identifier for the [`HostTemplate`].
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub host_template_id: Option<u64>,

    /// A [`ConfigRefMap`] of [`HostRef`] objects associated with this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: Option<ConfigRefMap<HostRef>>,

    /// A string representing the quorum percentage for the component.
    ///
    /// The quorum percentage is a string representing a percentage with exactly 2 decimals.
    /// The percentage must be a valid ratio for the number of hosts associated with the component.
    ///
    /// For example, if the component has 3 hosts, the quorum percentage must be one of the
    /// following:
    /// * 0.00
    /// * 33.33
    /// * 66.67
    /// * 100.00
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quorum_pct: Option<String>,

    // Read-only fields --------------------------------------------------------------------------//
    /// Unix Timestamp indicating when the icon was last updated, or 0 if there is no icon.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub has_icon: Option<u64>,

    /// The unique identifier of the [`BSMComponent`].
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,

    /// A reference string unique to this [`BSMComponent`].
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    /// A boolean indicating whether the component is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

/// Implementation of the [`CreateFromJson`] trait for `BSMComponent`.
///
/// Enables the creation of a `BSMComponent` instance from a JSON representation,
/// typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for BSMComponent {}

/// Implementation of the [`ConfigObject`] trait for `BSMComponent`.
///
/// Provides specific behavior for [`BSMComponent`]s, such as determining the configuration path
/// and unique name within the Opsview system.
impl ConfigObject for BSMComponent {
    type Builder = BSMComponentBuilder;

    /// Returns a builder for constructing a `BSMComponent` object.
    ///
    /// # Returns
    /// A `BSMComponentBuilder` object.
    fn builder() -> Self::Builder {
        BSMComponentBuilder::new()
    }

    /// Returns the API configuration path for [`BSMComponent`]s.
    ///
    /// # Returns
    /// A string representing the API path where [`BSMComponent`]s are configured.
    fn config_path() -> Option<String> {
        Some("/config/bsmcomponent".to_string())
    }

    /// Returns a minimal `BSMComponent` object with only the name set.
    ///
    /// # Arguments
    /// * `name` - Name of the [`BSMComponent`].
    ///
    /// # Returns
    /// A minimal `BSMComponent` object with only the name set and all other fields in their default
    /// state.
    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            name: validate_and_trim_bsmcomponent_name(name)?,
            ..Default::default()
        })
    }

    /// Returns the unique name of the [`BSMComponent`].
    ///
    /// This name is used to identify the `BSMComponent` when building the `HashMap` for a
    /// [`ConfigObjectMap`].
    ///
    /// Since names are not required to be unique for BSMComponent objects in Opsview, we have to
    /// add the id at the end of the string, if it's present.
    ///
    /// If the id is not present, but the ref_ is, use the ref_ instead as is.
    ///
    /// If neither is present, use the name and hope for the best. // TODO: Investigate a better approach.
    ///
    /// # Returns
    /// A string representing the unique name of the [`BSMComponent`].
    fn unique_name(&self) -> String {
        let name = self.name.clone();
        match (self.id.as_ref(), self.ref_.as_ref()) {
            (Some(id), _) => format!("{}-{}", name, id),
            (_, Some(ref_)) => ref_.clone(),
            _ => name,
        }
    }
}

impl Persistent for BSMComponent {
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
        Some(BSM_COMPONENT_NAME_REGEX_STR.to_string())
    }

    fn validated_name(&self, name: &str) -> Result<String, OpsviewConfigError> {
        validate_and_trim_bsmcomponent_name(name)
    }

    fn set_name(&mut self, new_name: &str) -> Result<String, OpsviewConfigError> {
        self.name = self.validated_name(new_name)?;
        Ok(self.name.clone())
    }

    /// Clears the read-only fields.
    fn clear_readonly(&mut self) {
        self.has_icon = None;
        self.id = None;
        self.ref_ = None;
        self.uncommitted = None;
    }
}

impl PersistentMap for ConfigObjectMap<BSMComponent> {
    fn config_path() -> Option<String> {
        Some("/config/bsmcomponent".to_string())
    }
}

/// Builder for creating instances of [`BSMComponent`].
///
/// This struct provides a fluent interface for constructing a `BSMComponent` object.
#[derive(Clone, Debug, Default)]
pub struct BSMComponentBuilder {
    name: Option<String>,
    host_template: Option<HostTemplateRef>,
    host_template_id: Option<u64>,
    hosts: Option<ConfigRefMap<HostRef>>,
    quorum_pct: Option<String>,
}

impl Builder for BSMComponentBuilder {
    type ConfigObject = BSMComponent;

    /// Creates a new instance of [`BSMComponentBuilder`] with default values.
    ///
    /// Initializes a new builder for creating a [`BSMComponent`] object with all fields in their
    /// default state.
    fn new() -> Self {
        BSMComponentBuilder::default()
    }

    /// A fluent method that sets the `name` field and returns Self, allowing for method
    /// chaining.
    ///
    /// # Arguments
    /// * `name` - Name of the [`BSMComponent`].
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Builds the [`BSMComponent`] with the specified properties.
    ///
    /// Constructs a new `BSMComponent` based on the current state of the builder.
    /// This method performs validations and returns an error if any required field
    /// is not set.
    ///
    /// # Returns
    /// A `Result` containing the constructed `BSMComponent` or an error if the component
    /// could not be built due to missing required fields.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;
        let host_template = require_field(&self.host_template, "host_template")?;
        let hosts = require_field(&self.hosts, "hosts")?;
        let quorum_pct = require_field(&self.quorum_pct, "quorum_pct")?;

        // TODO: Assert that self.host_template_id == host_template.id if both are present.

        Ok(BSMComponent {
            // Required fields
            name: validate_and_trim_bsmcomponent_name(&name)?,
            host_template: Some(host_template),
            host_template_id: self.host_template_id,
            quorum_pct: Some(validated_pct_and_ratio(&quorum_pct, hosts.len())?),
            hosts: Some(hosts),
            // Read-only fields
            has_icon: None,
            id: None,
            ref_: None,
            uncommitted: None,
        })
    }
}

impl BSMComponentBuilder {
    /// Clears the `host_template` field.
    pub fn clear_host_template(mut self) -> Self {
        self.host_template = None;
        self
    }

    /// Clears the `host_template_id` field.
    pub fn clear_host_template_id(mut self) -> Self {
        self.host_template_id = None;
        self
    }

    /// Clears the `hosts` field.
    pub fn clear_hosts(mut self) -> Self {
        self.hosts = None;
        self
    }

    /// Clears the `name` field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Clears the `quorum_pct` field.
    pub fn clear_quorum_pct(mut self) -> Self {
        self.quorum_pct = None;
        self
    }

    /// A fluent method that sets the `host_template` field and returns Self, allowing for method
    /// chaining.
    ///
    /// # Arguments
    /// * `host_template` - `HostTemplate` associated with the component.
    pub fn host_template(mut self, host_template: HostTemplate) -> Self {
        self.host_template = Some(HostTemplateRef::from(host_template));
        self
    }

    /// A fluent method that sets the `host_template_id` field and returns Self, allowing for method
    /// chaining.
    ///
    /// # Arguments
    /// * `host_template_id` - Unique identifier for the `HostTemplate`.
    pub fn host_template_id(mut self, host_template_id: u64) -> Self {
        self.host_template_id = Some(host_template_id);
        self
    }

    /// A fluent method that sets the `hosts` field and returns Self, allowing for method
    /// chaining.
    ///
    /// # Arguments
    /// * `hosts` - A reference to a [`ConfigObjectMap`] of [`Host`] objects associated with this component.
    pub fn hosts(mut self, hosts: &ConfigObjectMap<Host>) -> Self {
        if let Some(ref host_template) = self.host_template {
            for host in hosts.values() {
                if !host.has_template(host_template) {
                    panic!(
                        "Host '{}' does not have the template '{}'",
                        host.name,
                        host_template.name()
                    );
                }
            }
        } else {
            panic!("host_template must be set before hosts");
        }

        self.hosts = Some(hosts.into());
        self
    }

    /// A fluent method that sets the `quorum_pct` field and returns Self, allowing for method
    /// chaining.
    ///
    /// # Arguments
    /// * `quorum_pct` - String representing the quorum percentage for the component.
    pub fn quorum_pct(mut self, quorum_pct: &str) -> Self {
        self.quorum_pct = Some(quorum_pct.to_string());
        self
    }
}

/// A reference version of [`BSMComponent`] that is used when passing or retrieving a [`BSMComponent`]
/// object as part of another object.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct BSMComponentRef {
    name: String,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
}

/// Enables the creation of a [`BSMComponentRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for BSMComponentRef {}

impl ConfigRef for BSMComponentRef {
    type FullObject = BSMComponent;

    /// Returns the reference string of the [`BSMComponentRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`BSMComponentRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`BSMComponentRef`].
    ///
    /// This name is used to identify the `BSMComponentRef` when building the `HashMap` for a
    /// [`ConfigRefMap`].
    ///
    /// If the name is not present, but the ref_ is, use the ref_ instead as is.
    ///
    /// If neither is present, use the name and hope for the best. // TODO: Investigate a better approach.
    ///
    /// # Returns
    /// A string representing the unique name of the [`BSMComponentRef`].
    fn unique_name(&self) -> String {
        let name = self.name.clone();
        match self.ref_.as_ref() {
            Some(ref_) => ref_.clone(),
            _ => name,
        }
    }
}

impl From<BSMComponent> for BSMComponentRef {
    fn from(component: BSMComponent) -> Self {
        Self {
            name: component.name.clone(),
            ref_: component.ref_.clone(),
        }
    }
}

impl From<Arc<BSMComponent>> for BSMComponentRef {
    fn from(item: Arc<BSMComponent>) -> Self {
        let component: BSMComponent = Arc::try_unwrap(item).unwrap_or_else(|arc| (*arc).clone());
        BSMComponentRef::from(component)
    }
}

impl From<&ConfigObjectMap<BSMComponent>> for ConfigRefMap<BSMComponentRef> {
    fn from(components: &ConfigObjectMap<BSMComponent>) -> Self {
        ref_map_from(components)
    }
}

lazy_static! {
    static ref QUORUM_PCT_REGEX: Regex = regex::Regex::new(r"^\d{1,3}\.\d{2}$").unwrap();
}

/// Validates the format of the quorum percentage.
///
/// # Arguments
/// * `quorum_pct` - String representing the quorum percentage.
///
/// # Returns
/// A Result indicating whether the quorum percentage is valid or not.
fn validated_pct_and_ratio(
    percentage: &str,
    number_of_hosts: usize,
) -> Result<String, OpsviewConfigError> {
    debug!(
        "is_valid_pct_and_ratio: percentage: {}, number_of_hosts: {}",
        percentage, number_of_hosts
    );

    if percentage == "0.00" {
        return Ok(percentage.to_string());
    }

    if percentage == "100.00" {
        return Ok(percentage.to_string());
    }

    if number_of_hosts == 0 {
        return Err(OpsviewConfigError::InvalidQuorum(
            "The number of hosts must be greater than 0".to_string(),
        ));
    }

    // Validate the percentage format
    if !QUORUM_PCT_REGEX.is_match(percentage) {
        return Err(OpsviewConfigError::InvalidQuorum(
            "Must be a number with exactly 2 decimals".to_string(),
        ));
    }

    // Generate valid percentages as strings
    let mut valid_percentages = Vec::new();
    for host_count in 0..=number_of_hosts {
        let pct = 100.0 * host_count as f64 / number_of_hosts as f64;
        valid_percentages.push(format!("{:.2}", pct));
    }

    // Check if the provided percentage is in the list of valid percentages
    if valid_percentages.contains(&percentage.to_string()) {
        Ok(percentage.to_string())
    } else {
        Err(OpsviewConfigError::InvalidQuorum(format!(
            "The percentage '{}' is not a valid ratio for '{}' hosts",
            percentage, number_of_hosts
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{HostGroup, MonitoringCluster};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_is_valid_pct_and_ratio() {
        assert!(validated_pct_and_ratio("100.00", 1).is_ok());
        assert!(validated_pct_and_ratio("99.99", 10000).is_ok());
        assert!(validated_pct_and_ratio("0.00", 0).is_ok());
        assert!(validated_pct_and_ratio("0.01", 10000).is_ok());
        assert!(validated_pct_and_ratio("0.99", 10000).is_ok());
        assert!(validated_pct_and_ratio("1.00", 100).is_ok());
        assert!(validated_pct_and_ratio("1.01", 10000).is_ok());
        assert!(validated_pct_and_ratio("99.99", 10000).is_ok());
        assert!(validated_pct_and_ratio("100.00", 1).is_ok());
        assert!(validated_pct_and_ratio("50.00", 0).is_err());
        assert!(validated_pct_and_ratio("100.01", 10000).is_err());
        assert!(validated_pct_and_ratio("999.99", 1000).is_err());
        assert!(validated_pct_and_ratio("999.99", 10).is_err());
        assert!(validated_pct_and_ratio("999.99", 100).is_err());
        assert!(validated_pct_and_ratio("100", 1).is_err());
        assert!(validated_pct_and_ratio("1", 100).is_err());
        assert!(validated_pct_and_ratio("0.00", 3).is_ok()); // 0/3
        assert!(validated_pct_and_ratio("100.00", 3).is_ok()); // 3/3
        assert!(validated_pct_and_ratio("33.33", 3).is_ok()); // 1/3
        assert!(validated_pct_and_ratio("66.67", 3).is_ok()); // 2/3
        assert!(validated_pct_and_ratio("100", 3).is_err());
        assert!(validated_pct_and_ratio("0.00", 2).is_ok());
        assert!(validated_pct_and_ratio("50.00", 2).is_ok());
        assert!(validated_pct_and_ratio("100.00", 2).is_ok());
        assert!(validated_pct_and_ratio("90.00", 2).is_err());

        let host_template = HostTemplate::builder()
            .name("Host Template ")
            .build()
            .unwrap();

        let mut host_templates = ConfigObjectMap::<HostTemplate>::new();
        host_templates.add(host_template.clone());

        let host_templates = host_templates;

        let root_hostgroup = HostGroup::builder()
            .name("Opsview")
            .clear_parent()
            .build()
            .unwrap();

        let cluster = MonitoringCluster::minimal("Cluster 1")
            .expect("Failed to create cluster with name 'Cluster 1'");

        let host = Host::builder()
            .name("Host_1")
            .alias("Host 1")
            .ip("127.0.0.1")
            .hostgroup(root_hostgroup)
            .monitored_by(cluster)
            .hosttemplates(&host_templates)
            .build()
            .unwrap();

        let mut hosts = ConfigObjectMap::<Host>::new();
        hosts.add(host);

        let bsm_comp_1 = BSMComponent::builder()
            .name("Comp 1")
            .host_template(host_template.clone())
            .hosts(&hosts)
            .quorum_pct("100.00")
            .build();

        assert!(bsm_comp_1.is_ok());

        let bsm_comp_2 = BSMComponent::builder()
            .name("Comp 1")
            .host_template(host_template.clone())
            .hosts(&hosts)
            .quorum_pct("100")
            .build();

        assert!(bsm_comp_2.is_err());
        assert_eq!(
            bsm_comp_2.err().unwrap().to_string(),
            "Invalid quorum: Must be a number with exactly 2 decimals",
        );
        let bsm_comp_3 = BSMComponent::builder()
            .name("Comp 1")
            .host_template(host_template)
            .hosts(&hosts)
            .quorum_pct("90.00")
            .build();

        assert!(bsm_comp_3.is_err());
        assert_eq!(
            bsm_comp_3.err().unwrap().to_string(),
            "Invalid quorum: The percentage '90.00' is not a valid ratio for '1' hosts",
        );
    }

    #[test]
    fn test_default() {
        let bsm_component = BSMComponent::default();

        assert!(bsm_component.name.is_empty());
    }

    #[test]
    fn test_minimal() {
        let bsm_component = BSMComponent::minimal("My BSM Component");

        assert_eq!(bsm_component.unwrap().name, "My BSM Component".to_string());
    }

    #[test]
    fn test_is_valid_bsmcomponent_name() {
        // Test valid names
        assert!(validate_and_trim_bsmcomponent_name("ValidComponent123").is_ok());
        assert!(validate_and_trim_bsmcomponent_name("Valid_Component-With.Symbols!").is_ok());
        assert!(validate_and_trim_bsmcomponent_name("A").is_ok());
        assert!(validate_and_trim_bsmcomponent_name(
            "A component name with spaces and symbols *&^%$#@!"
        )
        .is_ok());
        assert!(validate_and_trim_bsmcomponent_name(&"a".repeat(255)).is_ok()); // Max length

        // Test invalid names
        assert!(validate_and_trim_bsmcomponent_name("").is_err()); // Empty name
        assert!(validate_and_trim_bsmcomponent_name(" ").is_err()); // Name with only space
        assert!(validate_and_trim_bsmcomponent_name(&"a".repeat(256)).is_err()); // Exceeds max length
        assert!(validate_and_trim_bsmcomponent_name("Invalid\nComponent").is_err()); // Contains newline
        assert!(validate_and_trim_bsmcomponent_name("Invalid\tComponent").is_err()); // Contains tab
        assert!(validate_and_trim_bsmcomponent_name("Invalid\rComponent").is_err());
        // Contains carriage return
    }
}
