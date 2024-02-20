use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};

/// Represents a [MonitoringServer](https://docs.itrsgroup.com/docs/opsview/6.8.9/administration/distributed-monitoring/distributed-monitoring/index.html#Heading-overview) in Opsview.
///
/// Monitoring clusters consist of an Opsview Orchestrator, or one or more Opsview Collectors. This
/// struct defines the structure for a monitoring cluster entity as used in Opsview.
///
/// # Example
/// ```rust
/// use opsview::config::{MonitoringCluster, MonitoringServer};
/// use opsview::prelude::*;
///
/// let monitoring_server = MonitoringServer::minimal("My Monitoring Server").unwrap();
///
/// let mut collectors = ConfigObjectMap::<MonitoringServer>::new();
/// collectors.add(monitoring_server);
///
/// let collectors = collectors;
///
/// let monitoring_cluster = MonitoringCluster::builder()
///   .name("My Monitoring Cluster")
///   .collectors(collectors)
///   .build()
///   .unwrap();
///
///   assert_eq!(monitoring_cluster.name, "My Monitoring Cluster");
/// ```
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct MonitoringServer {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the `MonitoringServer`.
    pub name: String,

    // Optional fields ---------------------------------------------------------------------------//
    /// A reference string unique to this Monitoring Cluster.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,
}

/// Enables the creation of a [`MonitoringServer`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for MonitoringServer {}

impl ConfigObject for MonitoringServer {
    type Builder = MonitoringServerBuilder;

    /// Returns a builder for constructing a [`MonitoringServer`] object.
    ///
    /// # Returns
    /// A [`MonitoringServerBuilder`] object.
    fn builder() -> Self::Builder {
        MonitoringServerBuilder::new()
    }

    /// Provides the configuration path for a [`MonitoringServer`] object within the Opsview system.
    ///
    /// # Returns
    /// None, as `MonitoringServer` objects are not directly configurable through the Opsview API.
    fn config_path() -> Option<String> {
        None
    }

    /// Returns the unique name of the [`MonitoringServer`] object.
    /// This name is used to identify the `MonitoringServer` when building the `HashMap` for an
    /// [`ConfigObjectMap`].
    fn unique_name(&self) -> String {
        match &self.ref_ {
            Some(ref_) => ref_.clone(),
            None => self.name.clone(),
        }
    }
}

impl Persistent for MonitoringServer {
    /// Returns the unique identifier.
    fn id(&self) -> Option<u64> {
        None
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
        self.ref_ = None;
    }
}

impl PersistentMap for ConfigObjectMap<MonitoringServer> {
    fn config_path() -> Option<String> {
        None
    }
}

/// Builder for creating instances of [`MonitoringServer`].
///
/// Provides a fluent interface for constructing a `MonitoringServer` object with optional fields.
#[derive(Clone, Debug, Default)]
pub struct MonitoringServerBuilder {
    name: Option<String>,
}

impl Builder for MonitoringServerBuilder {
    type ConfigObject = MonitoringServer;

    /// Creates a new instance of [`MonitoringServerBuilder`] with default values.
    /// Initializes a new builder for creating a [`MonitoringServer`] object with all fields in their
    /// default state.
    fn new() -> Self {
        MonitoringServerBuilder::default()
    }

    /// Sets the name field.
    ///
    /// # Arguments
    /// * `name` - The name of the `MonitoringServer`.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Consumes the builder and returns a [`MonitoringServer`] object.
    ///
    /// # Returns
    /// A `MonitoringServer` object with the values specified in the builder.
    ///
    /// # Errors
    /// Returns an `OpsviewConfigError` if the name field is not set.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;
        Ok(MonitoringServer {
            name: validate_and_trim_monitoringcluster_name(&name)?,
            ref_: None,
        })
    }
}

impl MonitoringServerBuilder {
    /// Clears the name field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let server = MonitoringServer::default();

        assert!(server.name.is_empty());
    }

    #[test]
    fn test_minimal() {
        let server = MonitoringServer::minimal("My Monitoring Server");

        assert_eq!(server.unwrap().name, "My Monitoring Server".to_string());
    }
}
