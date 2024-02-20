use super::{MonitoringCluster, MonitoringClusterRef, NetflowSource};
use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};

/// Represents a [NetflowCollector](https://docs.itrsgroup.com/docs/opsview/6.8.9/rest-api/config/api-config-netflow-collectors/index.html) in Opsview.
///
/// Netflow collectors are used to collect Netflow data from network devices. This struct defines
/// the structure for a Netflow collector entity as used in Opsview.
///
/// # Example
/// ```rust
/// use opsview::config::{NetflowCollector, MonitoringCluster};
/// use opsview::prelude::*;
///
/// let my_monitoring_cluster = MonitoringCluster::minimal("My Monitoring Cluster").unwrap();
///
/// let netflow_collector = NetflowCollector::builder()
///   .name("My Netflow Collector")
///   .monitoring_server(my_monitoring_cluster)
///   .build()
///   .unwrap();
///
///   assert_eq!(netflow_collector.name, "My Netflow Collector".to_string());
/// ```
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct NetflowCollector {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the `NetflowCollector`.
    pub name: String,

    // TODO: Investigate if this needs to be a MonitoringServer instead of a MonitoringCluster.
    /// The [`MonitoringClusterRef`] associated with this `NetflowCollector`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_server: Option<MonitoringClusterRef>,

    // Semi-optional fields ----------------------------------------------------------------------//
    /// The port number used by the `NetflowCollector`.
    /// Default: Some(9995)
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub port: Option<u64>,

    /// [`ConfigObjectMap`] of [`NetflowSource`] objects associated with this `NetflowCollector`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<ConfigObjectMap<NetflowSource>>,

    /// The port number used by the `NetflowCollector` for secure connections.
    /// Default: Some(6343)
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub sport: Option<u64>,

    // Read-only fields --------------------------------------------------------------------------//
    /// The unique identifier of the `NetflowCollector`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,

    /// A reference string unique to this `NetflowCollector`.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    /// A boolean indicating whether the collector is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

impl Default for NetflowCollector {
    /// Creates a new instance of [`NetflowCollector`] with default values.
    ///
    /// Initializes a new `NetflowCollector` object with all fields in their default state.
    fn default() -> Self {
        NetflowCollector {
            name: "".to_string(),
            monitoring_server: None,
            port: Some(9995),
            sources: None,
            sport: Some(6343),
            id: None,
            ref_: None,
            uncommitted: None,
        }
    }
}

/// Enables the creation of a [`NetflowCollector`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for NetflowCollector {}

impl ConfigObject for NetflowCollector {
    type Builder = NetflowCollectorBuilder;

    /// Returns a builder for constructing a [`NetflowCollector`] object.
    ///
    /// # Returns
    /// A [`super::MonitoringClusterBuilder`] object.
    fn builder() -> Self::Builder {
        NetflowCollectorBuilder::new()
    }

    /// Provides the configuration path for a [`NetflowCollector`] object within the Opsview system.
    ///
    /// # Returns
    /// A string representing the API path where NetflowCollectors are configured.
    fn config_path() -> Option<String> {
        Some("/config/netflow_collector".to_string())
    }

    /// Returns the unique name of the [`NetflowCollector`] object.
    /// This name is used to identify the `NetflowCollector` when building the `HashMap` for an
    /// [`ConfigObjectMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }

    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            name: validate_and_trim_netflowcollector_name(name)?,
            ..Default::default()
        })
    }
}

impl Persistent for NetflowCollector {
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
        validate_and_trim_netflowcollector_name(name)
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

impl PersistentMap for ConfigObjectMap<NetflowCollector> {
    fn config_path() -> Option<String> {
        Some("/config/netflow_collector".to_string())
    }
}

/// Builder for creating instances of [`NetflowCollector`].
///
/// Provides a fluent interface for constructing a `NetflowCollector` object with optional fields.
#[derive(Clone, Debug)]
pub struct NetflowCollectorBuilder {
    name: Option<String>,
    monitoring_server: Option<MonitoringClusterRef>,
    port: Option<u64>,
    sources: Option<ConfigObjectMap<NetflowSource>>,
    sport: Option<u64>,
}

impl Default for NetflowCollectorBuilder {
    /// Creates a new instance of [`NetflowCollectorBuilder`] with default values.
    ///
    /// Initializes a new builder for creating a [`NetflowCollector`] object with all fields in their
    /// default state.
    fn default() -> Self {
        NetflowCollectorBuilder {
            name: None,
            monitoring_server: None,
            port: Some(9995),
            sources: None,
            sport: Some(6343),
        }
    }
}

impl Builder for NetflowCollectorBuilder {
    type ConfigObject = NetflowCollector;

    /// Creates a new instance of [`NetflowCollectorBuilder`] with default values.
    /// Initializes a new builder for creating a [`NetflowCollector`] object with all fields in their
    /// default state.
    fn new() -> Self {
        NetflowCollectorBuilder::default()
    }

    /// Sets the name of the `NetflowCollector` object.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Consumes the builder and returns a [`NetflowCollector`] object.
    ///
    /// # Returns
    /// A `NetflowCollector` object initialized with the values set in the builder.
    ///
    /// # Errors
    /// Returns an error if the name field is not set.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;
        let monitoring_server = require_field(&self.monitoring_server, "monitoring_server")?;
        let validated_port = self.port.map(validate_port).transpose()?;
        let validated_sport = self.sport.map(validate_port).transpose()?;

        Ok(NetflowCollector {
            name: validate_and_trim_netflowcollector_name(&name)?,
            monitoring_server: Some(monitoring_server),
            port: validated_port,
            sources: self.sources,
            sport: validated_sport,
            id: None,
            ref_: None,
            uncommitted: None,
        })
    }
}

impl NetflowCollectorBuilder {
    /// Clears the name field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Clears the Monitoring Cluster associated with the `NetflowCollector` object.
    pub fn clear_monitoring_server(mut self) -> Self {
        self.monitoring_server = None;
        self
    }

    /// Clears the port number used by the `NetflowCollector` object.
    pub fn clear_port(mut self) -> Self {
        self.port = None;
        self
    }

    /// Clear the [`ConfigObjectMap`] of [`NetflowSource`] objects associated with the `NetflowCollector` object.
    pub fn clear_sources(mut self) -> Self {
        self.sources = None;
        self
    }

    /// Clears the port number used by the `NetflowCollector` object for secure connections.
    pub fn clear_sport(mut self) -> Self {
        self.sport = None;
        self
    }

    /// Sets the [`MonitoringCluster`] associated with the `NetflowCollector` object.
    ///
    /// # Arguments
    /// * `monitoring_server` - The [`MonitoringCluster`] associated with the `NetflowCollector` object.
    pub fn monitoring_server(mut self, monitoring_server: MonitoringCluster) -> Self {
        self.monitoring_server = Some(MonitoringClusterRef::from(monitoring_server));
        self
    }

    /// Sets the port number used by the `NetflowCollector` object.
    ///
    /// # Arguments
    /// * `port` - The port number used by the `NetflowCollector` object.
    pub fn port(mut self, port: u64) -> Self {
        self.port = Some(port);
        self
    }

    /// Sets the [`ConfigObjectMap`] of [`NetflowSource`] objects associated with the `NetflowCollector` object.
    ///
    /// # Arguments
    /// * `sources` - The [`ConfigObjectMap`] of [`NetflowSource`] objects associated with the `NetflowCollector` object.
    pub fn sources(mut self, sources: ConfigObjectMap<NetflowSource>) -> Self {
        self.sources = Some(sources);
        self
    }

    /// Sets the port number used by the `NetflowCollector`
    /// object for secure connections.
    ///
    /// # Arguments
    /// * `sport` - The port number used by the `NetflowCollector` object for secure connections.
    pub fn sport(mut self, sport: u64) -> Self {
        self.sport = Some(sport);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let collector = NetflowCollector::default();

        assert!(collector.name.is_empty());
    }

    #[test]
    fn test_minimal() {
        let collector = NetflowCollector::minimal("My Netflow Collector");

        assert_eq!(collector.unwrap().name, "My Netflow Collector".to_string());
    }
}
