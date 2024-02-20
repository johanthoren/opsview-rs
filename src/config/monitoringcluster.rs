use super::{Host, HostRef, MonitoringServer, RoleRef};
use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Represents a [MonitoringCluster](https://docs.itrsgroup.com/docs/opsview/6.8.9/administration/distributed-monitoring/distributed-monitoring/index.html#Heading-overview) in Opsview.
///
/// Monitoring clusters consist of an Opsview Orchestrator, or one or more Opsview Collectors. This
/// struct defines the structure for a monitoring cluster entity as used in Opsview.
///
/// # Example
/// ```rust
/// use opsview::config::MonitoringCluster;
/// use opsview::prelude::*;
///
/// let monitoring_cluster = MonitoringCluster::builder()
///   .name("My Monitoring Cluster")
///   .build()
///   .unwrap();
///
///   assert_eq!(monitoring_cluster.name, "My Monitoring Cluster");
/// ```
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct MonitoringCluster {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the `MonitoringCluster`.
    pub name: String,

    // Optional fields ---------------------------------------------------------------------------//
    /// A boolean indicating whether the `MonitoringCluster` is activated.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub activated: Option<bool>,

    /// A boolean indicating whether active [`Host`] checks are enabled for the `MonitoringCluster`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub active_host_checks_enabled: Option<bool>,

    /// A boolean indicating whether active [`super::ServiceCheck`]s are enabled for the `MonitoringCluster`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub active_service_checks_enabled: Option<bool>,

    /// [`ConfigObjectMap`] of `MonitoringServer` objects associated with this monitoring cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectors: Option<ConfigObjectMap<MonitoringServer>>,

    /// A boolean indicating whether [event handlers](https://docs.itrsgroup.com/docs/opsview/6.8.9/monitoring/event-handlers/index.html#Heading-overview) are enabled for the `MonitoringCluster`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub event_handlers_enabled: Option<bool>,

    /// A boolean indicating whether the `MonitoringCluster` is passive.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub passive: Option<bool>,

    // Read-only fields --------------------------------------------------------------------------//
    /// A boolean indicating whether the `MonitoringCluster` is managed by the CloudOps team.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub cloudops_owned: Option<bool>,

    /// The unique identifier of the `MonitoringCluster`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,

    /// [`ConfigRefMap`] of [`HostRef`] objects associated with this monitoring cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitors: Option<ConfigRefMap<HostRef>>,

    /// A reference string unique to this Monitoring Cluster.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    // TODO: This field is undocumented. Investigate whether or not this can be an optional field.
    /// A boolean indicating whether the `MonitoringCluster` should collect network topology data.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub network_topology_enabled: Option<bool>,

    // TODO: This field is undocumented. Investigate whether or not this can be an optional field.
    /// A boolean indicating whether the `MonitoringCluster` is remotely managed.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub remotely_managed: Option<bool>,

    /// [`ConfigRefMap`] of [`RoleRef`] objects associated with this monitoring cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<ConfigRefMap<RoleRef>>,

    /// A boolean indicating whether the `MonitoringCluster` is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

/// Enables the creation of a [`MonitoringCluster`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for MonitoringCluster {}

impl ConfigObject for MonitoringCluster {
    type Builder = MonitoringClusterBuilder;

    /// Returns a builder for constructing a [`MonitoringCluster`] object.
    ///
    /// # Returns
    /// A [`MonitoringClusterBuilder`] object.
    fn builder() -> Self::Builder {
        MonitoringClusterBuilder::new()
    }

    /// Provides the configuration path for a [`MonitoringCluster`] object within the Opsview system.
    ///
    /// # Returns
    /// A string representing the API path where monitoring clusters are configured.
    fn config_path() -> Option<String> {
        Some("/config/monitoringcluster".to_string())
    }

    /// Returns the unique name of the [`MonitoringCluster`] object.
    /// This name is used to identify the `MonitoringCluster` when building the `HashMap` for an
    /// [`ConfigObjectMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl Persistent for MonitoringCluster {
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
        Some(MONITORINGCLUSTER_NAME_REGEX_STR.to_string())
    }

    fn validated_name(&self, name: &str) -> Result<String, OpsviewConfigError> {
        validate_and_trim_monitoringcluster_name(name)
    }

    fn set_name(&mut self, new_name: &str) -> Result<String, OpsviewConfigError> {
        self.name = self.validated_name(new_name)?;
        Ok(self.name.clone())
    }

    fn clear_readonly(&mut self) {
        self.cloudops_owned = None;
        self.id = None;
        self.monitors = None;
        self.network_topology_enabled = None;
        self.ref_ = None;
        self.remotely_managed = None;
        self.roles = None;
        self.uncommitted = None;
    }
}

impl PersistentMap for ConfigObjectMap<MonitoringCluster> {
    fn config_path() -> Option<String> {
        Some("/config/monitoringcluster".to_string())
    }
}

/// Builder for creating instances of [`MonitoringCluster`].
///
/// Provides a fluent interface for constructing a `MonitoringCluster` object with optional fields.
#[derive(Clone, Debug, Default)]
pub struct MonitoringClusterBuilder {
    name: Option<String>,
    activated: Option<bool>,
    active_host_checks_enabled: Option<bool>,
    active_service_checks_enabled: Option<bool>,
    collectors: Option<ConfigObjectMap<MonitoringServer>>,
    event_handlers_enabled: Option<bool>,
    monitors: Option<ConfigRefMap<HostRef>>,
    passive: Option<bool>,
}

impl Builder for MonitoringClusterBuilder {
    type ConfigObject = MonitoringCluster;

    /// Creates a new instance of [`MonitoringClusterBuilder`] with default values.
    /// Initializes a new builder for creating a [`MonitoringCluster`] object with all fields in their
    /// default state.
    fn new() -> Self {
        MonitoringClusterBuilder::default()
    }

    /// Sets the name field.
    ///
    /// # Arguments
    /// * `name` - The name of the `MonitoringCluster`.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Consumes the builder and returns a [`MonitoringCluster`] object.
    ///
    /// # Returns
    /// A `MonitoringCluster` object with the values specified in the builder.
    ///
    /// # Errors
    /// Returns an `OpsviewClientError` if the name field is not set.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;

        Ok(MonitoringCluster {
            name: validate_and_trim_monitoringcluster_name(&name)?,
            activated: self.activated,
            active_host_checks_enabled: self.active_host_checks_enabled,
            active_service_checks_enabled: self.active_service_checks_enabled,
            collectors: self.collectors,
            event_handlers_enabled: self.event_handlers_enabled,
            monitors: self.monitors,
            passive: self.passive,
            cloudops_owned: None,
            id: None,
            network_topology_enabled: None,
            ref_: None,
            remotely_managed: None,
            roles: None,
            uncommitted: None,
        })
    }
}

impl MonitoringClusterBuilder {
    /// Sets the activated field.
    ///
    /// # Arguments
    /// * `activated` - The boolean indicating whether the `MonitoringCluster` is activated.
    pub fn activated(mut self, activated: bool) -> Self {
        self.activated = Some(activated);
        self
    }

    /// Sets the active_host_checks_enabled field.
    ///
    /// # Arguments
    /// * `active_host_checks_enabled` - The boolean indicating whether active [`Host`] checks are enabled for the `MonitoringCluster`.
    pub fn active_host_checks_enabled(mut self, active_host_checks_enabled: bool) -> Self {
        self.active_host_checks_enabled = Some(active_host_checks_enabled);
        self
    }

    /// Sets the active_service_checks_enabled field.
    ///
    /// # Arguments
    /// * `active_service_checks_enabled` - The boolean indicating whether active [`super::ServiceCheck`]s are enabled for the `MonitoringCluster`.
    pub fn active_service_checks_enabled(mut self, active_service_checks_enabled: bool) -> Self {
        self.active_service_checks_enabled = Some(active_service_checks_enabled);
        self
    }

    /// Clears the activated field.
    pub fn clear_activated(mut self) -> Self {
        self.activated = None;
        self
    }

    /// Clears the active_host_checks_enabled field.
    pub fn clear_active_host_checks_enabled(mut self) -> Self {
        self.active_host_checks_enabled = None;
        self
    }

    /// Clears the active_service_checks_enabled field.
    pub fn clear_active_service_checks_enabled(mut self) -> Self {
        self.active_service_checks_enabled = None;
        self
    }

    /// Clears the collectors field.
    pub fn clear_collectors(mut self) -> Self {
        self.collectors = None;
        self
    }

    /// Clears the event_handlers_enabled field.
    pub fn clear_event_handlers_enabled(mut self) -> Self {
        self.event_handlers_enabled = None;
        self
    }

    /// Clears the monitors field.
    pub fn clear_monitors(mut self) -> Self {
        self.monitors = None;
        self
    }

    /// Clears the name field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Clears the passive field.
    pub fn clear_passive(mut self) -> Self {
        self.passive = None;
        self
    }

    /// Sets the collectors field.
    ///
    /// # Arguments
    /// * `collectors` - A [`ConfigObjectMap`] of `MonitoringServer` objects associated with this monitoring cluster.
    pub fn collectors(mut self, collectors: ConfigObjectMap<MonitoringServer>) -> Self {
        self.collectors = Some(collectors);
        self
    }

    /// Sets the event_handlers_enabled field.
    ///
    /// # Arguments
    /// * `event_handlers_enabled` - The boolean indicating whether [event handlers](https://docs.itrsgroup.com/docs/opsview/6.8.9/monitoring/event-handlers/index.html#Heading-overview) are enabled for the `MonitoringCluster`.
    pub fn event_handlers_enabled(mut self, event_handlers_enabled: bool) -> Self {
        self.event_handlers_enabled = Some(event_handlers_enabled);
        self
    }

    /// Sets the monitors field.
    ///
    /// # Arguments
    /// * `monitors` - A reference to a [`ConfigObjectMap`] of [`Host`] objects associated with this monitoring cluster.
    pub fn monitors(mut self, monitors: &ConfigObjectMap<Host>) -> Self {
        self.monitors = Some(monitors.into());
        self
    }

    /// Sets the passive field.
    ///
    /// # Arguments
    /// * `passive` - The boolean indicating whether the `MonitoringCluster` is passive.
    pub fn passive(mut self, passive: bool) -> Self {
        self.passive = Some(passive);
        self
    }
}

/// A reference version of [`MonitoringCluster`] that is used when passing or retrieving a
/// [`MonitoringCluster`] object as part of another object.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct MonitoringClusterRef {
    name: String,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
}

/// Enables the creation of a [`MonitoringClusterRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for MonitoringClusterRef {}

impl ConfigRef for MonitoringClusterRef {
    type FullObject = MonitoringCluster;

    /// Returns the reference string of the [`MonitoringClusterRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`MonitoringClusterRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`MonitoringClusterRef`] object.
    /// This name is used to identify the `MonitoringClusterRef` when building the `HashMap` for a
    /// [`ConfigRefMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl From<MonitoringCluster> for MonitoringClusterRef {
    /// Creates a [`MonitoringClusterRef`] object from a [`MonitoringCluster`] object.
    ///
    /// # Arguments
    /// * `monitoring_cluster` - A [`MonitoringCluster`] object.
    ///
    /// # Returns
    /// A [`MonitoringClusterRef`] object.
    fn from(monitoring_cluster: MonitoringCluster) -> Self {
        MonitoringClusterRef {
            name: monitoring_cluster.name.clone(),
            ref_: monitoring_cluster.ref_.clone(),
        }
    }
}

impl From<Arc<MonitoringCluster>> for MonitoringClusterRef {
    fn from(item: Arc<MonitoringCluster>) -> Self {
        let cmd: MonitoringCluster = Arc::try_unwrap(item).unwrap_or_else(|arc| (*arc).clone());
        MonitoringClusterRef::from(cmd)
    }
}

impl From<&ConfigObjectMap<MonitoringCluster>> for ConfigRefMap<MonitoringClusterRef> {
    fn from(clusters: &ConfigObjectMap<MonitoringCluster>) -> Self {
        ref_map_from(clusters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let cluster = MonitoringCluster::default();

        assert!(cluster.name.is_empty());
    }

    #[test]
    fn test_minimal() {
        let cluster = MonitoringCluster::minimal("My Monitoring Cluster");

        assert_eq!(cluster.unwrap().name, "My Monitoring Cluster".to_string());
    }
    #[test]
    fn test_monitoringcluster_name_regex() {
        // Test valid names
        assert!(validate_and_trim_monitoringcluster_name("Cluster 1").is_ok());
        assert!(validate_and_trim_monitoringcluster_name("Cluster-1").is_ok());
        assert!(validate_and_trim_monitoringcluster_name("Cluster_1").is_ok());
        assert!(validate_and_trim_monitoringcluster_name("Cluster1").is_ok());
        assert!(validate_and_trim_monitoringcluster_name(&"a".repeat(64)).is_ok());

        // Test names with characters not allowed by the regex
        assert!(validate_and_trim_monitoringcluster_name("Cluster!").is_err());
        assert!(validate_and_trim_monitoringcluster_name("Cluster@").is_err());
        assert!(validate_and_trim_monitoringcluster_name("Cluster#").is_err());
        assert!(validate_and_trim_monitoringcluster_name("Cluster$").is_err());
        assert!(validate_and_trim_monitoringcluster_name(&"a".repeat(65)).is_err());
    }
}
