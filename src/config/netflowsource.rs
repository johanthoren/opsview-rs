use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Represents the type of flow used by a [`NetflowSource`].
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FlowType {
    /// The `NetflowSource` uses Netflow.
    Netflow,
    /// The `NetflowSource` uses Sflow.
    Sflow,
}

impl Display for FlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlowType::Netflow => write!(f, "netflow"),
            FlowType::Sflow => write!(f, "sflow"),
        }
    }
}

/// Represents a [NetflowSource](https://docs.itrsgroup.com/docs/opsview/6.8.9/rest-api/config/api-config-netflow-sources/index.html) in Opsview.
///
/// # Example
/// ```rust
/// use opsview::config::{NetflowSource, FlowType};
/// use opsview::prelude::*;
///
/// let netflow_collector = NetflowSource::builder()
///   .ip("127.0.0.1")
///   .flowtype(FlowType::Netflow)
///   .active(true)
///   .port(1234)
///   .build()
///   .unwrap();
///
/// assert_eq!(netflow_collector.ip, "127.0.0.1".to_string());
/// assert_eq!(&netflow_collector.flowtype.to_string(), "netflow");
/// ```
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct NetflowSource {
    // Required fields ---------------------------------------------------------------------------//
    /// The flow type of the `NetflowSource`.
    pub flowtype: FlowType,

    /// The ip or address of the `NetflowSource`.
    pub ip: String,

    // Optional fields ---------------------------------------------------------------------------//
    /// A boolean indicating whether the `NetflowSource` is active.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub active: Option<bool>,

    /// The unique identifier of the [`super::NetflowCollector`].
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub collector_id: Option<u64>,

    /// The unique identifier of the Netflow [`super::Host`].
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub host_id: Option<u64>,

    /// TODO: Undocumented field.
    /// Default: Some(false)
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub ip_override: Option<bool>,

    /// The port number used by the `NetflowSource`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub port: Option<u64>,

    // Read-only fields --------------------------------------------------------------------------//
    /// The unique identifier of the `NetflowSource`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,

    /// A reference string unique to this `NetflowSource`.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    /// A boolean indicating whether the `NetflowSource` is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

impl Default for NetflowSource {
    /// Returns a default [`NetflowSource`] object.
    fn default() -> Self {
        NetflowSource {
            // Required fields -------------------------------------------------------------------//
            flowtype: FlowType::Netflow,
            ip: "".to_string(),
            // Optional fields -------------------------------------------------------------------//
            active: None,
            collector_id: None,
            host_id: None,
            ip_override: Some(false),
            port: None,
            // Read-only fields ------------------------------------------------------------------//
            id: None,
            ref_: None,
            uncommitted: None,
        }
    }
}

/// Enables the creation of a [`NetflowSource`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for NetflowSource {}

impl ConfigObject for NetflowSource {
    type Builder = NetflowSourceBuilder;

    /// Returns a builder for constructing a [`NetflowSource`] object.
    ///
    /// # Returns
    /// A [`NetflowSourceBuilder`] object.
    fn builder() -> Self::Builder {
        NetflowSourceBuilder::new()
    }

    /// Provides the configuration path for a [`NetflowSource`] object within the Opsview system.
    ///
    /// # Returns
    /// A string representing the API path where Netflow Sources are configured.
    fn config_path() -> Option<String> {
        Some("/config/netflow_source".to_string())
    }

    /// Returns a minimal `NetflowSource` object with only the name set.
    ///
    /// # Arguments
    /// * `ip` - The ip of the [`NetflowSource`].
    ///
    /// # Returns
    /// A Result containing a minimal `NetflowSource` object with only the name set, and
    /// the rest of the fields in their default states.
    fn minimal(ip: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            ip: ip.to_string(),
            ..Default::default()
        })
    }

    /// Returns the unique name of the [`NetflowSource`] object.
    /// This name is used to identify the `NetflowSource` when building the `HashMap` for a
    /// [`ConfigObjectMap`].
    fn unique_name(&self) -> String {
        format!("{}-{}", self.ip, self.flowtype)
    }
}

impl Persistent for NetflowSource {
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

    /// Returns the name, but since `NetflowSource` does not have a name field, it returns None.
    fn name(&self) -> Option<String> {
        None
    }

    fn name_regex(&self) -> Option<String> {
        None
    }

    // Since `NetflowSource` does not have a name field, this method always returns true.
    fn validated_name(&self, _name: &str) -> Result<String, OpsviewConfigError> {
        Ok(_name.to_string())
    }

    // Since `NetflowSource` does not have a name field, this method always returns Ok(()) and
    // does nothing.
    fn set_name(&mut self, _new_name: &str) -> Result<String, OpsviewConfigError> {
        Ok(_new_name.to_string())
    }

    fn clear_readonly(&mut self) {
        self.id = None;
        self.ref_ = None;
        self.uncommitted = None;
    }
}

impl PersistentMap for ConfigObjectMap<NetflowSource> {
    fn config_path() -> Option<String> {
        Some("/config/netflow_source".to_string())
    }
}

/// Builder for creating instances of [`NetflowSource`].
///
/// Provides a fluent interface for constructing a `NetflowSource` object with optional fields.
#[derive(Clone, Debug)]
pub struct NetflowSourceBuilder {
    // Required fields ---------------------------------------------------------------------------//
    flowtype: Option<FlowType>,
    ip: Option<String>,
    // Optional fields ---------------------------------------------------------------------------//
    active: Option<bool>,
    collector_id: Option<u64>,
    host_id: Option<u64>,
    ip_override: Option<bool>,
    port: Option<u64>,
}

impl Default for NetflowSourceBuilder {
    /// Returns a default [`NetflowSourceBuilder`] object.
    fn default() -> Self {
        NetflowSourceBuilder {
            // Required fields -------------------------------------------------------------------//
            flowtype: None,
            ip: None,
            // Optional fields -------------------------------------------------------------------//
            active: None,
            collector_id: None,
            host_id: None,
            ip_override: Some(false),
            port: None,
        }
    }
}

impl Builder for NetflowSourceBuilder {
    type ConfigObject = NetflowSource;

    /// Creates a new instance of [`NetflowSourceBuilder`] with default values.
    /// Initializes a new builder for creating a [`NetflowSource`] object with all fields in their
    /// default state.
    fn new() -> Self {
        NetflowSourceBuilder::default()
    }

    /// Sets the ip field.
    fn name(mut self, name: &str) -> Self {
        self.ip = Some(name.to_string());
        self
    }

    /// Consumes the builder and returns a [`NetflowSource`] object.
    ///
    /// # Returns
    /// A `NetflowSource` object initialized with the values set in the builder.
    ///
    /// # Errors
    /// Returns an `Error` if the `flowtype` or `ip` fields are not set.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let flowtype = require_field(&self.flowtype, "flowtype")?;
        let ip = require_field(&self.ip, "ip")?;
        let validated_port = self.port.map(validate_port).transpose()?;

        Ok(NetflowSource {
            flowtype,
            ip: validate_and_trim_ipv4(&ip)?,
            active: self.active,
            collector_id: self.collector_id,
            host_id: self.host_id,
            ip_override: self.ip_override,
            port: validated_port,
            id: None,
            ref_: None,
            uncommitted: None,
        })
    }
}

impl NetflowSourceBuilder {
    /// Clears the active field.
    pub fn clear_active(mut self) -> Self {
        self.active = None;
        self
    }

    /// Clears the collector_id field.
    pub fn clear_collector_id(mut self) -> Self {
        self.collector_id = None;
        self
    }

    /// Clears the flowtype field.
    pub fn clear_flowtype(mut self) -> Self {
        self.flowtype = None;
        self
    }

    /// Clears the host_id field.
    pub fn clear_host_id(mut self) -> Self {
        self.host_id = None;
        self
    }

    /// Clears the ip field.
    pub fn clear_ip(mut self) -> Self {
        self.ip = None;
        self
    }

    /// Clears the ip_override field.
    pub fn clear_ip_override(mut self) -> Self {
        self.ip_override = None;
        self
    }

    /// Clears the port field.
    pub fn clear_port(mut self) -> Self {
        self.port = None;
        self
    }

    /// Sets the active field.
    ///
    /// # Arguments
    /// * `active` - Boolean indicating whether the `NetflowSource` is active.
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    /// Sets the collector_id field.
    ///
    /// # Arguments
    /// * `collector_id` - Unique identifier of the [`super::NetflowCollector`].
    pub fn collector_id(mut self, collector_id: u64) -> Self {
        self.collector_id = Some(collector_id);
        self
    }

    /// Sets the flowtype field.
    ///
    /// # Arguments
    /// * `flowtype` - The `FlowType` of the `NetflowSource`.
    pub fn flowtype(mut self, flowtype: FlowType) -> Self {
        self.flowtype = Some(flowtype);
        self
    }

    /// Sets the ip field.
    ///
    /// # Arguments
    /// * `ip` - The ip or address of the `NetflowSource`.
    pub fn ip(mut self, ip: &str) -> Self {
        self.ip = Some(ip.to_string());
        self
    }

    /// Sets the ip_override field.
    ///
    /// # Arguments
    /// * `ip_override` - TODO: Undocumented field.
    pub fn ip_override(mut self, ip_override: bool) -> Self {
        self.ip_override = Some(ip_override);
        self
    }

    /// Sets the host_id field.
    ///
    /// # Arguments
    /// * `host_id` - Unique identifier of the Netflow host.
    pub fn host_id(mut self, host_id: u64) -> Self {
        self.host_id = Some(host_id);
        self
    }

    /// Sets the port field.
    ///
    /// # Arguments
    /// * `port` - Port number used by the `NetflowSource`.
    pub fn port(mut self, port: u64) -> Self {
        self.port = Some(port);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let source = NetflowSource::default();

        assert!(source.ip.is_empty());
    }

    #[test]
    fn test_minimal() {
        let source = NetflowSource::minimal("localhost");

        assert_eq!(source.unwrap().ip, "localhost".to_string());
    }

    #[test]
    fn test_deserialize_netflow_source() {
        let json = r#"{
            "active": "1",
            "collector_id": "1",
            "flowtype": "netflow",
            "host_id": "1",
            "id": "1",
            "ip": "127.0.0.1",
            "ip_override": "0",
            "port": "1234",
            "ref": "some_ref",
            "uncommitted": "0"
        }"#;

        let source: NetflowSource = serde_json::from_str(json).unwrap();

        assert_eq!(source.active, Some(true));
        assert_eq!(source.collector_id, Some(1));
        assert_eq!(source.flowtype, FlowType::Netflow);
        assert_eq!(source.host_id, Some(1));
        assert_eq!(source.id, Some(1));
        assert_eq!(source.ip, "127.0.0.1".to_string());
        assert_eq!(source.ip_override, Some(false));
        assert_eq!(source.port, Some(1234));
        assert_eq!(source.ref_, Some("some_ref".to_string()));
        assert_eq!(source.uncommitted, Some(false));
    }

    #[test]
    fn test_build_and_serialize_netflow_source() {
        let source = NetflowSource::builder()
            .active(true)
            .collector_id(1)
            .flowtype(FlowType::Netflow)
            .host_id(1)
            .ip("127.0.0.1")
            .ip_override(false)
            .port(1234)
            .build()
            .unwrap();

        let json = serde_json::to_string(&source).unwrap();
        let deserialized: NetflowSource = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, source);
    }

    #[test]
    fn test_invalid_ip_fails() {
        let source = NetflowSource::builder()
            .active(true)
            .collector_id(1)
            .flowtype(FlowType::Netflow)
            .host_id(1)
            .ip("256.256.256.256")
            .ip_override(false)
            .port(1234)
            .build();

        assert_eq!(
            source,
            Err(OpsviewConfigError::InvalidIP("256.256.256.256".to_string()))
        );
    }
}
