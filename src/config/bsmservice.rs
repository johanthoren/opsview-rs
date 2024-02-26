use super::{BSMComponent, BSMComponentRef, MonitoringCluster, MonitoringClusterRef};
use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Represents a [Business Service Management (BSM)](https://docs.itrsgroup.com/docs/opsview/current/monitoring/business-service-monitoring/business-service-monitoring/index.html) service in Opsview.
///
/// The `BSMService` struct defines the structure for a [BSM
/// Service](https://docs.itrsgroup.com/docs/opsview/current/monitoring/business-service-monitoring/create-business-service/index.html#create-a-business-service)
/// entity as used in Opsview. BSM services are used to group components and represent higher-level
/// business services for monitoring purposes.
///
/// # Example
/// ```rust
/// use opsview::config::{BSMComponent, BSMService, Host, HostGroup, HostTemplate, MonitoringCluster};
/// use opsview::prelude::*;
///
/// let host_template = HostTemplate::minimal("My Host Template")
///     .expect("Failed to create a minimal HostTemplate with the name 'My Host Template'");
///
/// let mut host_templates = ConfigObjectMap::<HostTemplate>::new();
///
/// host_templates.add(host_template.clone());
///
/// // Shadow the host_template variable to make it immutable.
/// let host_templates = host_templates;
///
/// let parent_group = HostGroup::minimal("Opsview")
///     .expect("Failed to create a minimal HostGroup with the name 'Opsview'");
///
/// let host_group = HostGroup::builder()
///   .name("My Host Group")
///   .parent(parent_group)
///   .build()
///   .unwrap();
///
/// let cluster_1 = MonitoringCluster::minimal("My Monitoring Cluster")
///     .expect("Failed to create a minimal MonitoringCluster with the name 'My Monitoring Cluster'");
///
/// let host = Host::builder()
///   .name("My_Host")
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
/// // Shadow the hosts variable to make it immutable.
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
///  let mut components = ConfigObjectMap::<BSMComponent>::new();
///  components.add(bsm_component);
///
///  // Shadow the components variable to make it immutable.
///  let components = components;
///
/// let bsm_service = BSMService::builder()
///  .name("My BSM Service")
///  .components(&components)
///  .build()
///  .unwrap();
///
/// assert_eq!(bsm_service.name, "My BSM Service".to_string());
/// ```
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct BSMService {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the BSMService.
    pub name: String,

    // Semi-optional fields ----------------------------------------------------------------------//
    /// A collection of [`BSMComponent`] objects that are part of this service.
    #[serde(alias = "business_components")]
    pub components: Option<ConfigRefMap<BSMComponentRef>>,

    // Optional fields ---------------------------------------------------------------------------//
    /// A reference to the [`MonitoringCluster`] which will handle the notifications for the
    /// service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_cluster: Option<MonitoringClusterRef>,

    // Read-only fields --------------------------------------------------------------------------//
    /// The unique identifier of the BSMService.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,

    /// An optional reference string unique to this BSMService.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    /// A boolean indicating whether the BSMService is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

/// Enables the creation of a [`BSMService`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for BSMService {}

impl ConfigObject for BSMService {
    type Builder = BSMServiceBuilder;

    /// Returns a builder for constructing a [`BSMService`] object.
    ///
    /// # Returns
    /// A `BSMServiceBuilder` object.
    fn builder() -> Self::Builder {
        BSMServiceBuilder::new()
    }

    /// Provides the configuration path for a [`BSMService`] object within the Opsview system.
    ///
    /// # Returns
    /// A string representing the API path where BSM Services are configured.
    fn config_path() -> Option<String> {
        Some("/config/bsmservice".to_string())
    }

    /// Returns a minimal `BSMService` object with only the name set.
    ///
    /// # Arguments
    /// * `name` - Name of the [`BSMService`].
    ///
    /// # Returns
    /// A minimal `BSMService` object with only the name set, and the rest of the fields in their
    /// default states.
    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            name: validate_and_trim_bsmservice_name(name)?,
            ..Default::default()
        })
    }

    /// Returns the unique name of the [`BSMService`] object.
    ///
    /// This name is used to identify the `BSMService` when building the `HashMap` for a
    /// [`ConfigObjectMap`].
    ///
    /// Since names are not required to be unique for BSMService objects in Opsview, we have to
    /// add the id at the end of the string, if it's present.
    ///
    /// If the id is not present, but the ref_ is, use the ref_ instead as is.
    ///
    /// # Returns
    /// A string representing the unique name of the BSM service.
    fn unique_name(&self) -> String {
        let name = self.name.clone();
        match (self.id.as_ref(), self.ref_.as_ref()) {
            (Some(id), _) => format!("{}-{}", name, id),
            (_, Some(ref_)) => ref_.clone(),
            _ => name,
        }
    }
}

impl Persistent for BSMService {
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
        validate_and_trim_bsmservice_name(name)
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

impl PersistentMap for ConfigObjectMap<BSMService> {
    fn config_path() -> Option<String> {
        Some("/config/bsmservice".to_string())
    }
}

/// Builder for creating instances of [`BSMService`].
///
/// Provides a fluent interface for constructing a `BSMService` object with optional fields.
#[derive(Clone, Debug, Default)]
pub struct BSMServiceBuilder {
    name: Option<String>,
    components: Option<ConfigRefMap<BSMComponentRef>>,
    monitoring_cluster: Option<MonitoringClusterRef>,
}

impl Builder for BSMServiceBuilder {
    type ConfigObject = BSMService;

    /// Creates a new instance of [`BSMServiceBuilder`] with default values.
    ///
    /// Initializes a new builder for creating a [`BSMService`] object with all fields in their
    /// default state.
    fn new() -> Self {
        BSMServiceBuilder::default()
    }

    /// Sets the name field.
    ///
    /// # Arguments
    /// * `name` - Name of the BSMService.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Builds the [`BSMService`] object with the specified properties.
    ///
    /// Constructs a new `BSMService` object based on the current state of the builder.
    /// Returns an error if any required field is not set.
    ///
    /// # Returns
    /// A `Result` containing the constructed `BSMService` object or an error if the object
    /// could not be built due to missing required fields.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;
        let components = require_field(&self.components, "components")?;

        Ok(BSMService {
            name: validate_and_trim_bsmservice_name(&name)?,
            components: Some(components),
            monitoring_cluster: self.monitoring_cluster,
            id: None,
            ref_: None,
            uncommitted: None,
        })
    }
}

impl BSMServiceBuilder {
    /// Clears the components field.
    pub fn clear_components(mut self) -> Self {
        self.components = None;
        self
    }

    /// Clears the `monitoring_cluster` field.
    pub fn clear_monitoring_cluster(mut self) -> Self {
        self.monitoring_cluster = None;
        self
    }

    /// Clears the name field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Sets the components field.
    ///
    /// # Arguments
    /// * `components` - Collection of `BSMComponent` objects for the service.
    pub fn components(mut self, components: &ConfigObjectMap<BSMComponent>) -> Self {
        self.components = Some(components.into());
        self
    }

    /// A fluent method that sets the `monitoring_cluster` field and returns Self, allowing for
    /// method chaining.
    ///
    /// # Arguments
    /// * `monitoring_cluster` - [`MonitoringCluster`] which will handle the notifications for the service.
    pub fn monitoring_cluster(mut self, monitoring_cluster: MonitoringCluster) -> Self {
        self.monitoring_cluster = Some(MonitoringClusterRef::from(monitoring_cluster));
        self
    }
}

/// A reference version of [`BSMService`] that is used when passing or retrieving a [`BSMService`]
/// object as part of another object.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct BSMServiceRef {
    name: String,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
}

/// Enables the creation of a [`BSMServiceRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for BSMServiceRef {}

impl ConfigRef for BSMServiceRef {
    type FullObject = BSMService;

    /// Returns the reference string of the [`BSMServiceRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`BSMServiceRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`BSMServiceRef`].
    ///
    /// This name is used to identify the `BSMServiceRef` when building the `HashMap` for a
    /// [`ConfigRefMap`].
    ///
    /// If the name is not present, but the ref_ is, use the ref_ instead as is.
    ///
    /// If neither is present, use the name and hope for the best. // TODO: Investigate a better approach.
    ///
    /// # Returns
    /// A string representing the unique name of the [`BSMServiceRef`].
    fn unique_name(&self) -> String {
        let name = self.name.clone();
        match self.ref_.as_ref() {
            Some(ref_) => ref_.clone(),
            _ => name,
        }
    }
}

impl From<BSMService> for BSMServiceRef {
    fn from(full_object: BSMService) -> Self {
        Self {
            name: full_object.name.clone(),
            ref_: full_object.ref_.clone(),
        }
    }
}

impl From<Arc<BSMService>> for BSMServiceRef {
    fn from(item: Arc<BSMService>) -> Self {
        let service: BSMService = Arc::try_unwrap(item).unwrap_or_else(|arc| (*arc).clone());
        BSMServiceRef::from(service)
    }
}

impl From<&ConfigObjectMap<BSMService>> for ConfigRefMap<BSMServiceRef> {
    fn from(services: &ConfigObjectMap<BSMService>) -> Self {
        ref_map_from(services)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let service = BSMService::default();

        assert!(service.name.is_empty());
    }

    #[test]
    fn test_minimal() {
        let service = BSMService::minimal("My BSM Service");

        assert_eq!(service.unwrap().name, "My BSM Service".to_string());
    }

    #[test]
    fn test_is_valid_bsmservice_name() {
        // Test valid names
        assert!(validate_and_trim_bsmservice_name("Valid Service Name").is_ok());
        assert!(validate_and_trim_bsmservice_name("Another-Valid_Service.Name123").is_ok());
        assert!(validate_and_trim_bsmservice_name(&"A".repeat(255)).is_ok()); // Max length

        // Test invalid names
        assert!(validate_and_trim_bsmservice_name("").is_err()); // Empty name
        assert!(validate_and_trim_bsmservice_name(&"A".repeat(256)).is_err());
        // Exceeds max length
    }
}
