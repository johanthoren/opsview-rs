use crate::{config::*, prelude::*, util::*};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use std::sync::Arc;

/// Represents [a host entity](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/hosts-groups/host/index.html) in Opsview.
///
/// The `Host` struct defines the structure for a host in the Opsview monitoring system.
/// Hosts are essential entities in Opsview, as they represent the devices that are monitored.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Host {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the `Host`, serving as a unique identifier.
    pub name: String,

    // Semi-required fields ----------------------------------------------------------------------//
    // These fields are required when creating a new host, but are not required when updating an
    // existing host. They are not always included in the JSON representation of a host returned
    // by the Opsview API, so they are represented as `Option` fields.
    /// The [`HostGroupRef`] to which the `Host` belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostgroup: Option<HostGroupRef>,

    /// The IP address of the `Host`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// The [`MonitoringClusterRef`] responsible for monitoring the `Host`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitored_by: Option<MonitoringClusterRef>,

    // Optional fields ---------------------------------------------------------------------------//
    /// The alias of the `Host`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    /// [`ConfigRefMap`] of [`BSMComponentRef`] objects associated with the `Host`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_components: Option<ConfigRefMap<BSMComponentRef>>,

    /// The number of check attempts before a state change.
    /// Default: Some(2)
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub check_attempts: Option<u64>,

    /// The [`HostCheckCommandRef`] that defines the check to be performed on the `Host`.
    /// Default: Some(HostCheckCommandRef{ name: "ping".to_string() })
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_command: Option<HostCheckCommandRef>,

    /// The interval between checks, in seconds.
    /// Default: Some(600)
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub check_interval: Option<u64>,

    /// The [`TimePeriodRef`] during which checks are performed.
    /// Default: Some(TimePeriodRef{ name: "24x7".to_string() })
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_period: Option<TimePeriodRef>,

    /// A boolean indicating whether SNMP is enabled for the `Host`.
    /// Default: Some(false)
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub enable_snmp: Option<bool>,

    /// A string defining the [event handler](https://docs.itrsgroup.com/docs/opsview/6.8.9/monitoring/event-handlers/index.html#Heading-overview) for the `Host`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_handler: Option<String>,

    /// A boolean indicating if the [event handler](https://docs.itrsgroup.com/docs/opsview/6.8.9/monitoring/event-handlers/index.html#Heading-overview) should always execute.
    /// Default: Some(false)
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub event_handler_always_exec: Option<bool>,

    /// A boolean indicating if [flap detection](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/hosts-groups/host/index.html#Heading-flap-detection) is enabled.
    /// Default: Some(true)
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub flap_detection_enabled: Option<bool>,

    /// [`ConfigRefMap`] of [`HostVariableRef`] objects representing [host attributes](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/hosts-groups/host/index.html#Heading-host-variables).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostattributes: Option<ConfigRefMap<HostVariableRef>>,

    /// [`ConfigRefMap`] of [`HostTemplateRef`] objects applied to the `Host`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosttemplates: Option<ConfigRefMap<HostTemplateRef>>,

    /// The [`HostIcon`] associated with the `Host`.
    /// Default: Some(HostIcon{ name: "LOGO - Opsview".to_string() })
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_hosticon"
    )]
    pub icon: Option<HostIcon>,

    /// [`ConfigRefMap`] of [`HashtagRef`] objects associated with the `Host`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<ConfigRefMap<HashtagRef>>,

    // TODO: Add validation of this field to the builder.
    /// A string representing the NMIS node type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nmis_node_type: Option<String>,

    /// The interval between notifications, in seconds.
    /// Default: Some(3600)
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub notification_interval: Option<u64>,

    /// A string representing [notification options](https://docs.itrsgroup.com/docs/opsview/6.8.9/monitoring/notifications/notifications/index.html).
    /// # Available options
    /// * u - Unreachable
    /// * d - Down
    /// * r - Recovery
    /// * f - Flapping
    /// Default: Some("u,d,r".to_string())
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_options: Option<String>,

    /// The [`TimePeriod`] during which notifications are sent.
    /// Default: `Some(TimePeriod{ name: "24x7".to_string() })`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_period: Option<TimePeriodRef>,

    /// A string containing [additional addresses](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/hosts-groups/host/index.html#other-hostnamesips) for the `Host`.
    /// Must be a comma-separated list of valid addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_addresses: Option<String>,

    /// [`ConfigRefMap`] of parent `HostRef` objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<ConfigRefMap<HostRef>>,

    /// A boolean indicating whether RANCID (NetAudit) auto-enable is active.
    /// Default: `Some(false)`
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub rancid_autoenable: Option<bool>,

    /// The type of connection to use for RANCID (NetAudit).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rancid_connection_type: Option<RancidConnectionType>,

    /// The username for RANCID (NetAudit).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rancid_username: Option<String>,

    /// The password for RANCID (NetAudit).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rancid_password: Option<String>,

    /// The `RancidVendor` for the `Host` for NetAudit configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rancid_vendor: Option<RancidVendor>,

    /// The interval between retry checks, in seconds.
    /// Default:`Some(60)`
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub retry_check_interval: Option<u64>,

    /// [`ConfigRefMap`] of [`ServiceCheckHostRef`] objects associated with the `Host`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servicechecks: Option<ConfigRefMap<ServiceCheckHostRef>>,

    /// The SNMP community string for the `Host`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snmp_community: Option<String>,

    /// A boolean indicating whether extended throughput data is collected via SNMP.
    /// Default: Some(false)
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub snmp_extended_throughput_data: Option<bool>,

    // TODO: Add support for the `snmpinterfaces` field.
    // `snmpinterfaces` is an optional column for /config/host that needs to be explicitly requested
    // in the API call. It is not included in the default response.
    //
    /// The maximum SNMP message size.
    /// Default: Some(0)
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub snmp_max_msg_size: Option<u64>,

    /// The SNMP port number.
    /// Default: Some(161)
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub snmp_port: Option<u64>,

    /// A boolean indicating whether SNMP GETNEXT requests are used.
    /// Default: Some(false)
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub snmp_use_getnext: Option<bool>,

    /// A boolean indicating whether SNMP interface names are used.
    /// Default: Some(false)
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub snmp_use_ifname: Option<bool>,

    /// A string specifying the [`SNMPVersion`].
    /// Default: Some(SNMPVersion::V2c)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snmp_version: Option<SNMPVersion>,

    /// Optional authentication protocol for SNMPv3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snmpv3_authprotocol: Option<SNMPV3AuthProtocol>,

    /// Optional authentication password for SNMPv3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snmpv3_authpassword: Option<String>,

    /// Optional privacy protocol for SNMPv3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snmpv3_privprotocol: Option<SNMPV3PrivProtocol>,

    /// Optional privacy password for SNMPv3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snmpv3_privpassword: Option<String>,

    /// Optional username for SNMPv3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snmpv3_username: Option<String>,

    // TODO: Find out what constraints are on this field.
    /// Optional string specifying the interface description level for tidying.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tidy_ifdescr_level: Option<String>,

    /// Optional boolean indicating whether MRTG is used for the `Host`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub use_mrtg: Option<bool>,

    /// Optional boolean indicating whether NMIS is used for the `Host`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub use_nmis: Option<bool>,

    /// Optional boolean indicating whether [RANCID](https://docs.itrsgroup.com/docs/opsview/6.8.9/monitoring/net-audit/netaudit/index.html#Heading-overview) is used for the `Host`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub use_rancid: Option<bool>,

    // Read-only fields --------------------------------------------------------------------------//
    /// The unique identifier of the `Host`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,

    /// TODO: Undocumented field.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub is_master: Option<bool>,

    /// TODO: Undocumented field.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub is_ms: Option<bool>,

    /// TODO: Undocumented field.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub is_netflow: Option<bool>,

    /// A UNIX timestamp indicating the last update timestamp.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub last_updated: Option<u64>,

    /// A reference string unique to this `Host`.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    /// A boolean indicating whether the `Host` is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

impl Default for Host {
    fn default() -> Self {
        let tp_24x7 = TimePeriodRef::from(
            TimePeriod::minimal("24x7").expect("Failed to create TimePeriodRef with name '24x7'"),
        );
        let opsview_logo = HostIcon::minimal("LOGO - Opsview")
            .expect("Failed to create HostIcon with name 'LOGO - Opsview'");
        let ping_check = HostCheckCommandRef::from(
            HostCheckCommand::minimal("ping")
                .expect("Failed to create HostCheckCommandRef with name 'ping'"),
        );

        Host {
            name: "".to_string(),
            hostgroup: None,
            ip: None,
            monitored_by: None,
            alias: None,
            business_components: None,
            check_attempts: Some(2),
            check_command: Some(ping_check),
            check_interval: Some(600),
            check_period: Some(tp_24x7.clone()),
            enable_snmp: Some(false),
            event_handler: None,
            event_handler_always_exec: Some(false),
            flap_detection_enabled: Some(true),
            hostattributes: None,
            hosttemplates: None,
            icon: Some(opsview_logo),
            keywords: None,
            nmis_node_type: None,
            notification_interval: Some(3600),
            notification_options: Some("u,d,r".to_string()),
            notification_period: Some(tp_24x7),
            other_addresses: None,
            parents: None,
            rancid_autoenable: Some(false),
            rancid_connection_type: None,
            rancid_username: None,
            rancid_password: None,
            rancid_vendor: None,
            retry_check_interval: Some(60),
            servicechecks: None,
            snmp_community: None,
            snmp_extended_throughput_data: Some(false),
            snmp_max_msg_size: Some(0),
            snmp_port: Some(161),
            snmp_use_getnext: Some(false),
            snmp_use_ifname: Some(false),
            snmp_version: Some(SNMPVersion::V2c),
            snmpv3_authprotocol: None,
            snmpv3_authpassword: None,
            snmpv3_privprotocol: None,
            snmpv3_privpassword: None,
            snmpv3_username: None,
            tidy_ifdescr_level: None,
            use_mrtg: None,
            use_nmis: None,
            use_rancid: None,
            id: None,
            is_master: None,
            is_ms: None,
            is_netflow: None,
            last_updated: None,
            ref_: None,
            uncommitted: None,
        }
    }
}

/// Enables the creation of a [`Host`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for Host {}

impl ConfigObject for Host {
    type Builder = HostBuilder;

    /// Returns a builder for constructing a [`Host`] object.
    ///
    /// # Returns
    /// A [`HostBuilder`] object.
    fn builder() -> Self::Builder {
        HostBuilder::new()
    }

    /// Provides the configuration path for a [`Host`] object within the Opsview system.
    ///
    /// # Returns
    /// A string representing the API path where hosts are configured.
    fn config_path() -> Option<String> {
        Some("/config/host".to_string())
    }

    /// Returns a minimal `Host` object with only the name set.
    ///
    /// # Arguments
    /// * `name` - Name of the [`Host`].
    ///
    /// # Returns
    /// A Result containing a new instance of `Host` with the specified name.
    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            name: validate_and_trim_host_name(name)?,
            ..Default::default()
        })
    }

    /// Returns the unique name of the [`Host`] object.
    ///
    /// This name is used to identify the `Host` when building the `HashMap` for an
    /// `ConfigObjectMap`.
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl Persistent for Host {
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
        Some(HOST_NAME_REGEX_STR.to_string())
    }

    fn validated_name(&self, name: &str) -> Result<String, OpsviewConfigError> {
        validate_and_trim_host_name(name)
    }

    fn set_name(&mut self, new_name: &str) -> Result<String, OpsviewConfigError> {
        self.name = self.validated_name(new_name)?;
        Ok(self.name.clone())
    }

    fn clear_readonly(&mut self) {
        self.id = None;
        self.is_master = None;
        self.is_ms = None;
        self.is_netflow = None;
        self.last_updated = None;
        self.ref_ = None;
        self.uncommitted = None;
    }
}

impl PersistentMap for ConfigObjectMap<Host> {
    fn config_path() -> Option<String> {
        Some("/config/host".to_string())
    }
}

impl Host {
    /// Returns a boolean indicating whether the [`Host`] has the specified host template.
    ///
    /// # Arguments
    /// * `template` - A reference to the [`HostTemplate`] to check for.
    ///
    /// # Returns
    /// A boolean indicating whether the `Host` has the specified host template.
    pub fn has_template(&self, template: &HostTemplateRef) -> bool {
        if let Some(ref hosttemplates) = self.hosttemplates {
            hosttemplates.contains(&template.unique_name())
        } else {
            false
        }
    }

    /// Update a variable in the [`Host`].
    pub fn update_variable(&mut self, variable: Variable) {
        match self.hostattributes {
            None => {
                let mut vars = ConfigObjectMap::<Variable>::new();
                vars.add(variable);
                let vars = ConfigRefMap::<HostVariableRef>::from(&vars);
                self.hostattributes = Some(vars);
            }
            Some(ref mut vars) => {
                let name = variable.name().unwrap();
                vars.remove_named(&name);
                vars.add_named(&name, HostVariableRef::from(variable));
            }
        }
    }
}

/// Builder for creating instances of [`Host`].
///
/// Provides a fluent interface for constructing a `Host` object with optional fields,
/// allowing for more readable and maintainable object construction.
///
/// # Examples
///
/// ```rust
/// use opsview::config::{Host, HostBuilder, MonitoringCluster, HostGroup, HostTemplate};
/// use opsview::prelude::*;
///
/// let some_monitoring_cluster = MonitoringCluster::builder()
///    .name("example_monitoring_cluster")
///    .build()
///    .unwrap();
///
/// let parent_hostgroup = HostGroup::minimal("Opsview").unwrap();
///
/// let some_hostgroup = HostGroup::builder()
///   .name("example_hostgroup")
///   .parent(parent_hostgroup)
///   .build()
///   .unwrap();
///
/// let some_hosttemplates = ConfigObjectMap::<HostTemplate>::new();
///
/// let host = Host::builder()
///     .name("example_host")
///     .monitored_by(some_monitoring_cluster)   // T: MonitoringClusterRef
///     .hostgroup(some_hostgroup)               // T: HostGroup
///     .hosttemplates(&some_hosttemplates)      // T: ConfigObjectMap<HostTemplate>
///     .ip("192.168.1.1")
///     .check_interval(5)
///     .build()
///     .unwrap();
///
/// assert_eq!(host.name, "example_host".to_string());
/// ```
#[derive(Clone, Debug)]
pub struct HostBuilder {
    name: Option<String>,
    hostgroup: Option<HostGroupRef>,
    ip: Option<String>,
    monitored_by: Option<MonitoringClusterRef>,
    alias: Option<String>,
    business_components: Option<ConfigRefMap<BSMComponentRef>>,
    check_attempts: Option<u64>,
    check_command: Option<HostCheckCommandRef>,
    check_interval: Option<u64>,
    check_period: Option<TimePeriodRef>,
    enable_snmp: Option<bool>,
    event_handler: Option<String>,
    event_handler_always_exec: Option<bool>,
    flap_detection_enabled: Option<bool>,
    hostattributes: Option<ConfigRefMap<HostVariableRef>>,
    hosttemplates: Option<ConfigRefMap<HostTemplateRef>>,
    icon: Option<HostIcon>,
    keywords: Option<ConfigRefMap<HashtagRef>>,
    nmis_node_type: Option<String>,
    notification_interval: Option<u64>,
    notification_options: Option<String>,
    notification_period: Option<TimePeriodRef>,
    other_addresses: Option<String>,
    parents: Option<ConfigRefMap<HostRef>>,
    rancid_autoenable: Option<bool>,
    rancid_connection_type: Option<RancidConnectionType>,
    rancid_username: Option<String>,
    rancid_password: Option<String>,
    rancid_vendor: Option<RancidVendor>,
    retry_check_interval: Option<u64>,
    servicechecks: Option<ConfigRefMap<ServiceCheckHostRef>>,
    snmp_community: Option<String>,
    snmp_extended_throughput_data: Option<bool>,
    snmp_max_msg_size: Option<u64>,
    snmp_port: Option<u64>,
    snmp_use_getnext: Option<bool>,
    snmp_use_ifname: Option<bool>,
    snmp_version: Option<SNMPVersion>,
    snmpv3_authprotocol: Option<SNMPV3AuthProtocol>,
    snmpv3_authpassword: Option<String>,
    snmpv3_privprotocol: Option<SNMPV3PrivProtocol>,
    snmpv3_privpassword: Option<String>,
    snmpv3_username: Option<String>,
    tidy_ifdescr_level: Option<String>,
    use_mrtg: Option<bool>,
    use_nmis: Option<bool>,
    use_rancid: Option<bool>,
}

impl Default for HostBuilder {
    fn default() -> Self {
        let tp_24x7 = TimePeriodRef::from(
            TimePeriod::minimal("24x7").expect("Failed to create TimePeriodRef with name '24x7'"),
        );
        let opsview_logo = HostIcon::builder()
            .img_prefix("/static/images/logos/opsview")
            .name("LOGO - Opsview")
            .build()
            .unwrap();
        let ping_check = HostCheckCommandRef::from(
            HostCheckCommand::minimal("ping")
                .expect("Failed to create HostCheckCommandRef with name 'ping'"),
        );
        HostBuilder {
            name: None,
            hostgroup: None,
            ip: None,
            monitored_by: None,
            alias: None,
            business_components: None,
            check_attempts: None,
            check_command: Some(ping_check),
            check_interval: Some(600),
            check_period: Some(tp_24x7.clone()),
            enable_snmp: Some(false),
            event_handler: None,
            event_handler_always_exec: Some(false),
            flap_detection_enabled: Some(true),
            hostattributes: None,
            hosttemplates: None,
            icon: Some(opsview_logo),
            keywords: None,
            nmis_node_type: None,
            notification_interval: Some(3600),
            notification_options: Some("u,d,r".to_string()),
            notification_period: Some(tp_24x7),
            other_addresses: None,
            parents: None,
            rancid_autoenable: Some(false),
            rancid_connection_type: None,
            rancid_username: None,
            rancid_password: None,
            rancid_vendor: None,
            retry_check_interval: Some(60),
            servicechecks: None,
            snmp_community: None,
            snmp_extended_throughput_data: None,
            snmp_max_msg_size: Some(0),
            snmp_port: Some(161),
            snmp_use_getnext: Some(false),
            snmp_use_ifname: Some(false),
            snmp_version: Some(SNMPVersion::V2c),
            snmpv3_authprotocol: None,
            snmpv3_authpassword: None,
            snmpv3_privprotocol: None,
            snmpv3_privpassword: None,
            snmpv3_username: None,
            tidy_ifdescr_level: None,
            use_mrtg: None,
            use_nmis: None,
            use_rancid: None,
        }
    }
}

impl Builder for HostBuilder {
    type ConfigObject = Host;

    /// Creates a new instance of [`HostBuilder`] with default values.
    ///
    /// Initializes a new builder for creating a [`Host`] object with all fields in their default
    /// state.
    fn new() -> Self {
        HostBuilder::default()
    }

    /// Sets the name field.
    ///
    /// # Arguments
    /// * `name` - Name of the `Host`.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Completes the construction of a [`Host`] object.
    ///
    /// Validates the current state of the [`HostBuilder`] and assembles a `Host` object.
    /// This method checks that all required fields are set and any constraints are met.
    /// If a required field is missing or a constraint is violated, an error is returned.
    ///
    /// # Errors
    /// Returns an error if any required fields are not set or if there are inconsistencies in the
    /// provided data.
    ///
    /// # Returns
    /// On success, returns a `Result` containing the constructed `Host` object.
    /// On failure, returns a `Result` containing an [`OpsviewConfigError`].
    fn build(self) -> Result<Host, OpsviewConfigError> {
        let ip = require_field(&self.ip, "ip")?;
        let name = require_field(&self.name, "name")?;
        let hostgroup = require_field(&self.hostgroup, "hostgroup")?;
        let monitored_by = require_field(&self.monitored_by, "monitored_by")?;

        let mut validated_snmp_version: Option<SNMPVersion> = None;
        if let Some(ref snmp_version) = self.snmp_version {
            match snmp_version {
                SNMPVersion::V1 | SNMPVersion::V2c => {
                    self.validate_snmp_version1_or_2c()?;
                }
                SNMPVersion::V3(_) => {
                    let new_security_level = self.determine_snmpv3_security_level();
                    validated_snmp_version = Some(SNMPVersion::V3(new_security_level));
                }
            }
        }

        let validated_alias = validate_opt_string(self.alias, validate_and_trim_description)?;

        let validated_event_handler =
            validate_opt_string(self.event_handler, validate_and_trim_host_event_handler)?;

        let validated_notification_options = validate_opt_string(
            self.notification_options,
            validate_and_trim_host_notification_options,
        )?;

        let validated_other_addresses =
            validate_opt_string(self.other_addresses, validate_and_trim_other_addresses)?;

        let validated_rancid_password =
            validate_opt_string(self.rancid_password, validate_rancid_password)?;

        let validated_rancid_username =
            validate_opt_string(self.rancid_username, validate_rancid_username)?;

        let validated_snmp_community =
            validate_opt_string(self.snmp_community, validate_and_trim_snmp_community)?;

        let validated_snmp_port = self.snmp_port.map(validate_port).transpose()?;

        let validated_snmpv3_authpassword =
            validate_opt_string(self.snmpv3_authpassword, validate_snmpv3_password)?;

        let validated_snmpv3_privpassword =
            validate_opt_string(self.snmpv3_privpassword, validate_snmpv3_password)?;

        let validated_snmpv3_username =
            validate_opt_string(self.snmpv3_username, validate_snmpv3_username)?;

        Ok(Host {
            name: validate_and_trim_host_name(&name)?,
            hostgroup: Some(hostgroup),
            ip: Some(validate_and_trim_ip_or_hostname(&ip)?),
            monitored_by: Some(monitored_by),
            alias: validated_alias,
            business_components: self.business_components,
            check_attempts: self.check_attempts,
            check_command: self.check_command,
            check_interval: self.check_interval,
            check_period: self.check_period,
            enable_snmp: self.enable_snmp,
            event_handler: validated_event_handler,
            event_handler_always_exec: self.event_handler_always_exec,
            flap_detection_enabled: self.flap_detection_enabled,
            hostattributes: self.hostattributes,
            hosttemplates: self.hosttemplates,
            icon: self.icon,
            keywords: self.keywords,
            nmis_node_type: self.nmis_node_type,
            notification_interval: self.notification_interval,
            notification_options: validated_notification_options,
            notification_period: self.notification_period,
            other_addresses: validated_other_addresses,
            parents: self.parents,
            rancid_autoenable: self.rancid_autoenable,
            rancid_connection_type: self.rancid_connection_type,
            rancid_password: validated_rancid_password,
            rancid_username: validated_rancid_username,
            rancid_vendor: self.rancid_vendor,
            retry_check_interval: self.retry_check_interval,
            servicechecks: self.servicechecks,
            snmp_extended_throughput_data: self.snmp_extended_throughput_data,
            snmp_max_msg_size: self.snmp_max_msg_size,
            snmp_community: validated_snmp_community,
            snmp_port: validated_snmp_port,
            snmp_use_getnext: self.snmp_use_getnext,
            snmp_use_ifname: self.snmp_use_ifname,
            snmp_version: validated_snmp_version,
            snmpv3_authprotocol: self.snmpv3_authprotocol,
            snmpv3_authpassword: validated_snmpv3_authpassword,
            snmpv3_privprotocol: self.snmpv3_privprotocol,
            snmpv3_privpassword: validated_snmpv3_privpassword,
            snmpv3_username: validated_snmpv3_username,
            tidy_ifdescr_level: self.tidy_ifdescr_level,
            use_mrtg: self.use_mrtg,
            use_nmis: self.use_nmis,
            use_rancid: self.use_rancid,
            id: None,
            is_master: None,
            is_ms: None,
            is_netflow: None,
            last_updated: None,
            ref_: None,
            uncommitted: None,
        })
    }
}

impl HostBuilder {
    /// Sets the alias field.
    ///
    /// # Arguments
    /// * `alias` - The alias of the `Host`.
    pub fn alias(mut self, alias: &str) -> Self {
        self.alias = Some(alias.to_string());
        self
    }

    /// Sets the business_components field.
    ///
    /// # Arguments
    /// * `business_components` - A reference to a [`ConfigObjectMap`] of [`BSMComponent`] objects for the `Host`.
    pub fn business_components(
        mut self,
        business_components: &ConfigObjectMap<BSMComponent>,
    ) -> Self {
        self.business_components = Some(business_components.into());
        self
    }

    /// Sets the check_attempts field.
    ///
    /// # Arguments
    /// * `check_attempts` - Number of check attempts before a state change.
    pub fn check_attempts(mut self, check_attempts: u64) -> Self {
        self.check_attempts = Some(check_attempts);
        self
    }

    /// Sets the check_command field.
    ///
    /// # Arguments
    /// * `check_command` - The [`HostCheckCommand`] defining the check to be performed on the `Host`.
    pub fn check_command(mut self, check_command: HostCheckCommand) -> Self {
        self.check_command = Some(HostCheckCommandRef::from(check_command));
        self
    }

    /// Sets the check_interval field.
    ///
    /// # Arguments
    /// * `check_interval` - Interval between checks.
    pub fn check_interval(mut self, check_interval: u64) -> Self {
        self.check_interval = Some(check_interval);
        self
    }

    /// Sets the check_period field.
    ///
    /// # Arguments
    /// * `check_period` - A [`TimePeriod`] during which checks are performed.
    pub fn check_period(mut self, check_period: TimePeriod) -> Self {
        self.check_period = Some(TimePeriodRef::from(check_period));
        self
    }

    /// Clears the alias field.
    pub fn clear_alias(mut self) -> Self {
        self.alias = None;
        self
    }

    /// Clears the business_components field.
    pub fn clear_business_components(mut self) -> Self {
        self.business_components = None;
        self
    }

    /// Clears the check_attempts field.
    pub fn clear_check_attempts(mut self) -> Self {
        self.check_attempts = None;
        self
    }

    /// Clears the check_command field.
    pub fn clear_check_command(mut self) -> Self {
        self.check_command = None;
        self
    }

    /// Clears the check_interval field.
    pub fn clear_check_interval(mut self) -> Self {
        self.check_interval = None;
        self
    }

    /// Clears the check_period field.
    pub fn clear_check_period(mut self) -> Self {
        self.check_period = None;
        self
    }

    /// Clears the enable_snmp field.
    pub fn clear_enable_snmp(mut self) -> Self {
        self.enable_snmp = None;
        self
    }

    /// Clears the event_handler field.
    pub fn clear_event_handler(mut self) -> Self {
        self.event_handler = None;
        self
    }

    /// Clears the event_handler_always_exec field.
    pub fn clear_event_handler_always_exec(mut self) -> Self {
        self.event_handler_always_exec = None;
        self
    }

    /// Clears the flap_detection_enabled field.
    pub fn clear_flap_detection_enabled(mut self) -> Self {
        self.flap_detection_enabled = None;
        self
    }

    /// Clears the keywords field. Alias for `clear_keywords()`.
    pub fn clear_hashtags(mut self) -> Self {
        self.keywords = None;
        self
    }

    /// Clears the `Host`attributes field.
    pub fn clear_hostattributes(mut self) -> Self {
        self.hostattributes = None;
        self
    }

    /// Clears the `Host`group field.
    pub fn clear_hostgroup(mut self) -> Self {
        self.hostgroup = None;
        self
    }

    /// Clears the `Host`templates field.
    pub fn clear_hosttemplates(mut self) -> Self {
        self.hosttemplates = None;
        self
    }

    /// Clears the icon field.
    pub fn clear_icon(mut self) -> Self {
        self.icon = None;
        self
    }

    /// Clears the ip field.
    pub fn clear_ip(mut self) -> Self {
        self.ip = None;
        self
    }

    /// Clears the keywords field.
    pub fn clear_keywords(mut self) -> Self {
        self.keywords = None;
        self
    }

    /// Clears the monitored_by field.
    pub fn clear_monitored_by(mut self) -> Self {
        self.monitored_by = None;
        self
    }

    /// Clears the name field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Clears the nmis_node_type field.
    pub fn clear_nmis_node_type(mut self) -> Self {
        self.nmis_node_type = None;
        self
    }

    /// Clears the notification_interval field.
    pub fn clear_notification_interval(mut self) -> Self {
        self.notification_interval = None;
        self
    }

    /// Clears the notification_options field.
    pub fn clear_notification_options(mut self) -> Self {
        self.notification_options = None;
        self
    }

    /// Clears the notification_period field.
    pub fn clear_notification_period(mut self) -> Self {
        self.notification_period = None;
        self
    }

    /// Clears the other_addresses field.
    pub fn clear_other_addresses(mut self) -> Self {
        self.other_addresses = None;
        self
    }

    /// Clears the parents field.
    pub fn clear_parents(mut self) -> Self {
        self.parents = None;
        self
    }

    /// Clears the rancid_autoenable field.
    pub fn clear_rancid_autoenable(mut self) -> Self {
        self.rancid_autoenable = None;
        self
    }

    /// Clears the rancid_connection_type field.
    pub fn clear_rancid_connection_type(mut self) -> Self {
        self.rancid_connection_type = None;
        self
    }

    /// Clears the rancid_password field.
    pub fn clear_rancid_password(mut self) -> Self {
        self.rancid_password = None;
        self
    }

    /// Clears the rancid_username field.
    pub fn clear_rancid_username(mut self) -> Self {
        self.rancid_username = None;
        self
    }

    /// Clears the rancid_vendor field.
    pub fn clear_rancid_vendor(mut self) -> Self {
        self.rancid_vendor = None;
        self
    }

    /// Clears the retry_check_interval field.
    pub fn clear_retry_check_interval(mut self) -> Self {
        self.retry_check_interval = None;
        self
    }

    /// Clears the servicechecks field.
    pub fn clear_servicechecks(mut self) -> Self {
        self.servicechecks = None;
        self
    }

    /// Clears the snmp_community field.
    pub fn clear_snmp_community(mut self) -> Self {
        self.snmp_community = None;
        self
    }

    /// Clears the snmp_extended_throughput_data field.
    pub fn clear_snmp_extended_throughput_data(mut self) -> Self {
        self.snmp_extended_throughput_data = None;
        self
    }

    /// Clears the snmp_max_msg_size field.
    pub fn clear_snmp_max_msg_size(mut self) -> Self {
        self.snmp_max_msg_size = None;
        self
    }

    /// Clears the snmp_port field.
    pub fn clear_snmp_port(mut self) -> Self {
        self.snmp_port = None;
        self
    }

    /// Clears the snmp_use_getnext field.
    pub fn clear_snmp_use_getnext(mut self) -> Self {
        self.snmp_use_getnext = None;
        self
    }

    /// Clears the snmp_use_ifname field.
    pub fn clear_snmp_use_ifname(mut self) -> Self {
        self.snmp_use_ifname = None;
        self
    }

    /// Clears the snmp_version field.
    pub fn clear_snmp_version(mut self) -> Self {
        self.snmp_version = None;
        self
    }

    /// Clears the snmpv3_authprotocol field.
    pub fn clear_snmpv3_authprotocol(mut self) -> Self {
        self.snmpv3_authprotocol = None;
        self
    }

    /// Clears the snmpv3_privprotocol field.
    pub fn clear_snmpv3_privprotocol(mut self) -> Self {
        self.snmpv3_privprotocol = None;
        self
    }

    /// Clears the snmpv3_username field.
    pub fn clear_snmpv3_username(mut self) -> Self {
        self.snmpv3_username = None;
        self
    }

    /// Clears the tidy_ifdescr_level field.
    pub fn clear_tidy_ifdescr_level(mut self) -> Self {
        self.tidy_ifdescr_level = None;
        self
    }

    /// Clears the use_mrtg field.
    pub fn clear_use_mrtg(mut self) -> Self {
        self.use_mrtg = None;
        self
    }

    /// Clears the use_nmis field.
    pub fn clear_use_nmis(mut self) -> Self {
        self.use_nmis = None;
        self
    }

    /// Clears the use_rancid field.
    pub fn clear_use_rancid(mut self) -> Self {
        self.use_rancid = None;
        self
    }

    /// Sets the enable_snmp field.
    pub fn enable_snmp(mut self, enable_snmp: bool) -> Self {
        self.enable_snmp = Some(enable_snmp);
        self
    }

    /// Sets the event_handler field.
    ///
    /// # Arguments
    /// * `event_handler` - The event handler for the `Host`.
    pub fn event_handler(mut self, event_handler: &str) -> Self {
        self.event_handler = Some(event_handler.to_string());
        self
    }

    /// Sets the event_handler_always_exec field.
    ///
    /// # Arguments
    /// * `event_handler_always_exec` - Boolean indicating if the event handler should always execute.
    pub fn event_handler_always_exec(mut self, event_handler_always_exec: bool) -> Self {
        self.event_handler_always_exec = Some(event_handler_always_exec);
        self
    }

    /// Sets the flap_detection_enabled field.
    ///
    /// # Arguments
    /// * `flap_detection_enabled` - Boolean indicating if flap detection is enabled.
    pub fn flap_detection_enabled(mut self, flap_detection_enabled: bool) -> Self {
        self.flap_detection_enabled = Some(flap_detection_enabled);
        self
    }

    /// Sets the keywords field. Alias for `keywords()`.
    ///
    /// # Arguments
    /// * `keywords` - A reference to a [`ConfigObjectMap`] of [`Hashtag`] objects for the `Host`.
    pub fn hashtags(mut self, hashtags: &ConfigObjectMap<Hashtag>) -> Self {
        self.keywords = Some(hashtags.into());
        self
    }

    /// Sets the hostattributes field.
    ///
    /// # Arguments
    /// * `hostattributes` - A reference to a [`ConfigObjectMap`] of [`Variable`] objects for the `Host`.
    pub fn hostattributes(mut self, hostattributes: &ConfigObjectMap<Variable>) -> Self {
        self.hostattributes = Some(hostattributes.into());
        self
    }

    /// Sets the hostgroup field.
    ///
    /// # Arguments
    /// * `hostgroup` - The [`HostGroup`] object for the `Host`.
    pub fn hostgroup(mut self, hostgroup: HostGroup) -> Self {
        self.hostgroup = Some(HostGroupRef::from(hostgroup));
        self
    }

    /// Sets the hosttemplates field.
    ///
    /// # Arguments
    /// * `hosttemplates` - A reference to a [`ConfigObjectMap`] of [`HostTemplate`] objects for the `Host`.
    pub fn hosttemplates(mut self, hosttemplates: &ConfigObjectMap<HostTemplate>) -> Self {
        self.hosttemplates = Some(hosttemplates.into());
        self
    }

    /// Sets the icon field.
    ///
    /// # Arguments
    /// * `icon` - The [`HostIcon`] associated with the `Host`.
    pub fn icon(mut self, icon: HostIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the ip field.
    ///
    /// # Arguments
    /// * `ip` - IP address of the `Host`.
    pub fn ip(mut self, ip: &str) -> Self {
        self.ip = Some(ip.to_string());
        self
    }

    /// Sets the keywords field.
    ///
    /// # Arguments
    /// * `keywords` - A reference to a [`ConfigObjectMap`] of [`Hashtag`] objects for the `Host`.
    pub fn keywords(mut self, keywords: &ConfigObjectMap<Hashtag>) -> Self {
        self.keywords = Some(keywords.into());
        self
    }

    /// Sets the monitored_by field.
    ///
    /// # Arguments
    /// * `monitored_by` - The [`MonitoringCluster`] responsible for monitoring the `Host`.
    pub fn monitored_by(mut self, monitored_by: MonitoringCluster) -> Self {
        self.monitored_by = Some(MonitoringClusterRef::from(monitored_by));
        self
    }

    /// Sets the nmis_node_type field.
    ///
    /// # Arguments
    /// * `nmis_node_type` - NMIS node type.
    pub fn nmis_node_type(mut self, nmis_node_type: &str) -> Self {
        self.nmis_node_type = Some(nmis_node_type.to_string());
        self
    }

    /// Sets the notification_interval field.
    ///
    /// # Arguments
    /// * `notification_interval` - Interval between notifications.
    pub fn notification_interval(mut self, notification_interval: u64) -> Self {
        self.notification_interval = Some(notification_interval);
        self
    }

    /// Sets the notification_options field.
    ///
    /// # Arguments
    /// * `notification_options` - Notification options for the `Host`.
    ///  Valid options are:
    ///  * `d` - send notifications on DOWN state
    ///  * `u` - send notifications on UNREACHABLE state
    ///  * `r` - send notifications on recovery (UP state)
    ///  * `f` - send notifications when the host starts and stops flapping
    ///
    ///  Multiple options can be specified, separated by commas.
    ///
    ///  # Example
    ///  "d,u,r"
    pub fn notification_options(mut self, notification_options: &str) -> Self {
        self.notification_options = Some(notification_options.to_string());
        self
    }

    /// Sets the notification_period field.
    ///
    /// # Arguments
    /// * `notification_period` - A [`TimePeriod`] during which notifications are sent.
    pub fn notification_period(mut self, notification_period: TimePeriod) -> Self {
        self.notification_period = Some(TimePeriodRef::from(notification_period));
        self
    }

    /// Sets the other_addresses field.
    ///
    /// # Arguments
    /// * `other_addresses` - Additional addresses for the `Host`.
    pub fn other_addresses(mut self, other_addresses: &str) -> Self {
        self.other_addresses = Some(other_addresses.to_string());
        self
    }

    /// Sets the parents field.
    ///
    /// # Arguments
    /// * `parents` - A reference to a [`ConfigObjectMap`] of parent `Host` objects.
    pub fn parents(mut self, parents: &ConfigObjectMap<Host>) -> Self {
        self.parents = Some(parents.into());
        self
    }

    /// Sets the rancid_autoenable field.
    ///
    /// # Arguments
    /// * `rancid_autoenable` - Boolean indicating whether RANCID auto-enable is active.
    pub fn rancid_autoenable(mut self, rancid_autoenable: bool) -> Self {
        self.rancid_autoenable = Some(rancid_autoenable);
        self
    }

    /// Sets the rancid_connection_type field.
    ///
    /// # Arguments
    /// * `rancid_connection_type` - RANCID connection type.
    pub fn rancid_connection_type(mut self, rancid_connection_type: RancidConnectionType) -> Self {
        self.rancid_connection_type = Some(rancid_connection_type);
        self
    }

    /// Sets the rancid_password field.
    ///
    /// # Arguments
    /// * `rancid_password` - Password for RANCID.
    pub fn rancid_password(mut self, rancid_password: &str) -> Self {
        self.rancid_password = Some(rancid_password.to_string());
        self
    }

    /// Sets the rancid_username field.
    ///
    /// # Arguments
    /// * `rancid_username` - Username for RANCID.
    pub fn rancid_username(mut self, rancid_username: &str) -> Self {
        self.rancid_username = Some(rancid_username.to_string());
        self
    }

    /// Sets the rancid_vendor field.
    ///
    /// # Arguments
    /// * `rancid_vendor` - The [`RancidVendor`] for the `Host`.
    pub fn rancid_vendor(mut self, rancid_vendor: RancidVendor) -> Self {
        self.rancid_vendor = Some(rancid_vendor);
        self
    }

    /// Sets the retry_check_interval field.
    ///
    /// # Arguments
    /// * `retry_check_interval` - Interval between retry checks.
    pub fn retry_check_interval(mut self, retry_check_interval: u64) -> Self {
        self.retry_check_interval = Some(retry_check_interval);
        self
    }

    /// Sets the servicechecks field.
    ///
    /// # Arguments
    /// * `servicechecks` - A reference to a [`ConfigObjectMap`] of [`ServiceCheck`] objects for the `Host`.
    pub fn servicechecks(mut self, servicechecks: &ConfigObjectMap<ServiceCheck>) -> Self {
        self.servicechecks = Some(servicechecks.into());
        self
    }

    /// Sets the snmp_community field.
    ///
    /// # Arguments
    /// * `snmp_community` - SNMP community string.
    pub fn snmp_community(mut self, snmp_community: &str) -> Self {
        self.snmp_community = Some(snmp_community.to_string());
        self
    }

    /// Sets the snmp_extended_throughput_data field.
    ///
    /// # Arguments
    /// * `snmp_extended_throughput_data` - Boolean indicating whether extended throughput data is collected via SNMP.
    pub fn snmp_extended_throughput_data(mut self, snmp_extended_throughput_data: bool) -> Self {
        self.snmp_extended_throughput_data = Some(snmp_extended_throughput_data);
        self
    }

    /// Sets the snmp_max_msg_size field.
    ///
    /// # Arguments
    /// * `snmp_max_msg_size` - Maximum SNMP message size.
    pub fn snmp_max_msg_size(mut self, snmp_max_msg_size: u64) -> Self {
        self.snmp_max_msg_size = Some(snmp_max_msg_size);
        self
    }

    /// Sets the snmp_port field.
    ///
    /// # Arguments
    /// * `snmp_port` - SNMP port number.
    pub fn snmp_port(mut self, snmp_port: u64) -> Self {
        self.snmp_port = Some(snmp_port);
        self
    }

    /// Sets the snmp_use_getnext field.
    ///
    /// # Arguments
    /// * `snmp_use_getnext` - Boolean indicating whether SNMP GETNEXT requests are used.
    pub fn snmp_use_getnext(mut self, snmp_use_getnext: bool) -> Self {
        self.snmp_use_getnext = Some(snmp_use_getnext);
        self
    }

    /// Sets the snmp_use_ifname field.
    ///
    /// # Arguments
    /// * `snmp_use_ifname` - Boolean indicating whether SNMP interface names are used.
    pub fn snmp_use_ifname(mut self, snmp_use_ifname: bool) -> Self {
        self.snmp_use_ifname = Some(snmp_use_ifname);
        self
    }

    /// Sets the snmp_version field.
    ///
    /// # Arguments
    /// * `version` - The [`SNMPVersion`] to use.
    pub fn snmp_version(mut self, version: SNMPVersion) -> Self {
        self.snmp_version = Some(version);
        self
    }

    /// Sets the snmpv3_authprotocol field.
    ///
    /// # Arguments
    /// * `protocol` - [`SNMPV3AuthProtocol`] for SNMPv3.
    pub fn snmpv3_authprotocol(mut self, protocol: SNMPV3AuthProtocol) -> Self {
        self.snmpv3_authprotocol = Some(protocol);
        self
    }

    /// Sets the snmpv3_authpassword field.
    ///
    /// # Arguments
    /// * `snmpv3_authpassword` - Authentication password for SNMPv3.
    pub fn snmpv3_authpassword(mut self, snmpv3_authpassword: &str) -> Self {
        self.snmpv3_authpassword = Some(snmpv3_authpassword.to_string());
        self
    }

    /// Sets the snmpv3_privprotocol field.
    ///
    /// # Arguments
    /// * `protocol` - [`SNMPV3PrivProtocol`] for SNMPv3.
    pub fn snmpv3_privprotocol(mut self, protocol: SNMPV3PrivProtocol) -> Self {
        self.snmpv3_privprotocol = Some(protocol);
        self
    }

    /// Sets the snmpv3_privpassword field.
    ///
    /// # Arguments
    /// * `snmpv3_privpassword` - Privacy password for SNMPv3.
    pub fn snmpv3_privpassword(mut self, snmpv3_privpassword: &str) -> Self {
        self.snmpv3_privpassword = Some(snmpv3_privpassword.to_string());
        self
    }

    /// Sets the snmpv3_username field.
    ///
    /// # Arguments
    /// * `snmpv3_username` - Username for SNMPv3.
    pub fn snmpv3_username(mut self, snmpv3_username: &str) -> Self {
        self.snmpv3_username = Some(snmpv3_username.to_string());
        self
    }

    /// Sets the tidy_ifdescr_level field.
    ///
    /// # Arguments
    /// * `tidy_ifdescr_level` - Interface description level for tidying.
    pub fn tidy_ifdescr_level(mut self, tidy_ifdescr_level: &str) -> Self {
        self.tidy_ifdescr_level = Some(tidy_ifdescr_level.to_string());
        self
    }

    /// Sets the use_mrtg field.
    ///
    /// # Arguments
    /// * `use_mrtg` - Boolean indicating whether MRTG is used for the `Host`.
    pub fn use_mrtg(mut self, use_mrtg: bool) -> Self {
        self.use_mrtg = Some(use_mrtg);
        self
    }

    /// Sets the use_nmis field.
    ///
    /// # Arguments
    /// * `use_nmis` - Boolean indicating whether NMIS is used for the `Host`.
    pub fn use_nmis(mut self, use_nmis: bool) -> Self {
        self.use_nmis = Some(use_nmis);
        self
    }

    /// Sets the use_rancid field.
    ///
    /// # Arguments
    /// * `use_rancid` - Boolean indicating whether RANCID is used for the `Host`.
    pub fn use_rancid(mut self, use_rancid: bool) -> Self {
        self.use_rancid = Some(use_rancid);
        self
    }

    /// Helper method to validate the snmp version 1 or 2c fields.
    fn validate_snmp_version1_or_2c(&self) -> Result<(), OpsviewConfigError> {
        if self.enable_snmp.is_some_and(std::convert::identity) {
            self.snmp_community.as_ref().ok_or_else(|| {
                OpsviewConfigError::InvalidSNMPConfig(
                    "snmp_community must be set for version 1 or 2c when enable_snmp is true"
                        .to_string(),
                )
            })?;

            let fields = [
                (self.snmpv3_authprotocol.is_some(), "snmpv3_authprotocol"),
                (self.snmpv3_authpassword.is_some(), "snmpv3_authpassword"),
                (self.snmpv3_privprotocol.is_some(), "snmpv3_privprotocol"),
                (self.snmpv3_privpassword.is_some(), "snmpv3_privpassword"),
                (self.snmpv3_username.is_some(), "snmpv3_username"),
            ];

            for (condition, field_name) in fields {
                if condition {
                    return Err(OpsviewConfigError::InvalidSNMPConfig(format!(
                        "{} must not be set when SNMP version 1 or 2c is enabled.",
                        field_name
                    )));
                }
            }
        }

        Ok(())
    }

    /// Helper method to determine the snmpv3 security level.
    fn determine_snmpv3_security_level(&self) -> SNMPV3SecurityLevel {
        match (
            self.snmpv3_authprotocol.as_ref(),
            self.snmpv3_privprotocol.as_ref(),
        ) {
            (None, None) => SNMPV3SecurityLevel::NoAuthNoPriv,
            // Can't have priv without auth.
            (None, Some(_)) => SNMPV3SecurityLevel::NoAuthNoPriv,
            // If auth was empty, we don't care about priv.
            (Some(SNMPV3AuthProtocol::Unspecified), _) => SNMPV3SecurityLevel::NoAuthNoPriv,
            // If priv was empty, we don't care about auth.
            (_, Some(SNMPV3PrivProtocol::Unspecified)) => SNMPV3SecurityLevel::NoAuthNoPriv,
            (Some(a), None) => SNMPV3SecurityLevel::AuthNoPriv(a.clone()),
            (Some(a), Some(p)) => SNMPV3SecurityLevel::AuthPriv(a.clone(), p.clone()),
        }
    }
}

/// Serializes a [`HostIcon`] object into JSON without the `img_prefix` field.
///
/// This is needed when serializing a `HostIcon` object as part of a [`Host`] object.
fn serialize_hosticon<S>(hosticon: &Option<HostIcon>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match hosticon {
        Some(icon) => {
            let mut state = serializer.serialize_struct("HostIcon", 2)?;
            state.serialize_field("name", &icon.name)?;
            // Note: We skip `img_prefix` here
            if let Some(ref ref_) = icon.ref_ {
                state.serialize_field("ref", ref_)?;
            }
            state.end()
        }
        None => serializer.serialize_none(),
    }
}

/// A reference version of [`Host`] that is used when passing or retrieving a
/// [`Host`] object as part of another object.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct HostRef {
    name: String,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
}

/// Enables the creation of a [`HostRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for HostRef {}

impl ConfigRef for HostRef {
    type FullObject = Host;

    /// Returns the reference string of the [`HostRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`HostRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`HostRef`] object.
    ///
    /// This name is used to identify the `HostRef` when building the `HashMap` for a
    /// [`ConfigRefMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl From<Host> for HostRef {
    /// Creates a new [`HostRef`] instance from a [`Host`] object.
    ///
    /// # Arguments
    /// * `host` - The `Host` object from which to create the `HostRef`.
    fn from(host: Host) -> Self {
        HostRef {
            name: host.name.clone(),
            ref_: host.ref_.clone(),
        }
    }
}

impl From<Arc<Host>> for HostRef {
    fn from(item: Arc<Host>) -> Self {
        let host: Host = Arc::try_unwrap(item).unwrap_or_else(|arc| (*arc).clone());
        HostRef::from(host)
    }
}

impl From<&ConfigObjectMap<Host>> for ConfigRefMap<HostRef> {
    fn from(hosts: &ConfigObjectMap<Host>) -> Self {
        ref_map_from(hosts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal() {
        let host = Host::minimal("foo");

        assert_eq!(host.unwrap().name, "foo");
    }

    #[test]
    fn test_default() {
        let host = Host::default();

        assert!(host.name.is_empty());
    }

    #[test]
    fn test_build() {
        let host = Host::builder()
            .name("foo")
            .hostgroup(
                HostGroup::minimal("bar")
                    .expect("Unable to create a HostGroup with the name 'bar'"),
            )
            .ip("192.168.1.100")
            .monitored_by(
                MonitoringCluster::minimal("baz")
                    .expect("Unable to create a MonitoringCluster with the name 'baz'"),
            )
            .build()
            .unwrap();

        assert_eq!(host.name, "foo");
        assert_eq!(host.hostgroup.unwrap().name().as_str(), "bar");
        assert_eq!(host.ip.unwrap(), "192.168.1.100");
        assert_eq!(host.monitored_by.unwrap().name(), "baz");
    }

    #[test]
    fn test_build_fails_on_invalid_name() {
        let host = Host::builder().name("foo bar").build();
        assert!(host.is_err());

        let host2 = Host::builder().name(" foobar").build();
        assert!(host2.is_err());
    }

    #[test]
    fn test_minimal_fails_on_invalid_name() {
        let invalid_names = [
            "foo bar",
            "foo@bar.com",
            "foo#bar",
            "foo$bar",
            "foo%bar",
            "foo^bar",
        ];

        for &name in &invalid_names {
            let host = Host::minimal(name);
            assert!(host.is_err(), "Expected error for invalid name: {}", name);
        }
    }

    #[test]
    fn test_minimal_succeeds_on_valid_name() {
        let valid_names = ["foo.bar", "foo_bar", "foo-bar", " foo", "foo "]; // Strings will be trimmed.

        for &name in &valid_names {
            let host = Host::minimal(name);
            assert!(host.is_ok(), "Expected success for valid name: {}", name);
        }
    }

    #[test]
    fn test_valid_notification_options() {
        let cluster = MonitoringCluster::minimal("foo").unwrap();
        let hostgroup = HostGroup::minimal("Opsview").unwrap();
        let ip = "192.168.1.1";

        let test_host = |notification_options: &str| {
            let host = Host::builder()
                .name("foo")
                .notification_options(notification_options)
                .hostgroup(hostgroup.clone())
                .ip(ip)
                .monitored_by(cluster.clone())
                .build()
                .unwrap();

            assert_eq!(host.notification_options.unwrap(), notification_options);
        };

        let options = [
            "", "d,u,r,f", "d,u,r", "d,u", "d", "u,r,f", "u,r", "u", "f,r,u,d", "f,r,u",
        ];
        for &option in &options {
            test_host(option);
        }
    }

    #[test]
    fn test_invalid_notification_options() {
        let cluster = MonitoringCluster::minimal("foo").unwrap();
        let hostgroup = HostGroup::minimal("Opsview").unwrap();
        let ip = "192.168.1.1";

        let test_host = |notification_options: &str| {
            let result = Host::builder()
                .name("foo")
                .notification_options(notification_options)
                .hostgroup(hostgroup.clone())
                .ip(ip)
                .monitored_by(cluster.clone())
                .build();

            assert!(
                result.is_err(),
                "'{}' was actually a valid notification option",
                notification_options
            );
        };

        let options = [
            "d,u,r,f,f",
            "d,u,r,r,f",
            "d,u,u,r,f",
            "f,u,b,a,r",
            "d,u,r,f,",
            "d,u,r,f, ",
            "d,u,r,f, ,",
        ];

        for &option in &options {
            test_host(option);
        }
    }

    #[test]
    fn test_valid_ip_or_hostname_succeeds() {
        let valid_inputs = [
            "192.168.1.1",
            "localhost",
            "foo.bar",
            "foo-bar",
            "foo_bar",
            "foo_bar.baz",
            "foo_bar-baz",
            "foo_bar.baz-qux",
            "foo_bar.baz-qux.quux",
            "foo_bar.baz-qux.quux.corge",
            "foo_bar.baz-qux.quux.corge.grault",
            "10.1.2",
        ];

        for &input in &valid_inputs {
            assert!(
                validate_and_trim_ip_or_hostname(input).is_ok(),
                "Expected '{}' to be valid",
                input
            );
        }
    }

    #[test]
    fn test_is_valid_ip_or_hostname_fails() {
        let invalid_inputs = ["12.1.1.2.1", "", "127.0.1..1", "foo?bar", "foo@bar"];

        for &input in &invalid_inputs {
            assert!(
                validate_and_trim_ip_or_hostname(input).is_err(),
                "Expected '{}' to be invalid",
                input
            );
        }
    }

    #[test]
    fn test_build_with_invalid_ip_fails() {
        let cluster = MonitoringCluster::minimal("foo").unwrap();
        let hostgroup = HostGroup::minimal("Opsview").unwrap();

        let host = Host::builder()
            .name("foo")
            .hostgroup(hostgroup)
            .ip("10.0.0.1.1")
            .monitored_by(cluster.clone())
            .build();

        match host {
            Ok(_) => panic!("Expected OpsviewConfigError::InvalidIP, but got Ok"),
            Err(OpsviewConfigError::InvalidIP(ip)) => {
                assert_eq!(ip, "10.0.0.1.1");
            }
            Err(_) => {
                panic!("Expected OpsviewConfigError::InvalidIP, but got a different error");
            }
        }
    }

    fn setup_host_builder(name: &str, password: &str) -> Result<Host, OpsviewConfigError> {
        let cluster = MonitoringCluster::minimal("foo").unwrap();
        let hostgroup = HostGroup::minimal("Opsview").unwrap();

        Host::builder()
            .name(name)
            .hostgroup(hostgroup)
            .ip("127.0.0.1")
            .monitored_by(cluster)
            .snmpv3_authpassword(password)
            .build()
    }

    #[test]
    fn test_snmpv3_password_length_constraints() {
        let tests = vec![
            ("a".repeat(7), Err(OpsviewConfigError::StringTooShort(8, 7))),
            ("a".repeat(8), Ok(())),
            ("a".repeat(1600), Ok(())),
            (
                "a".repeat(1601),
                Err(OpsviewConfigError::StringTooLong(1600, 1601)),
            ),
        ];

        for (password, expected) in tests {
            let result = setup_host_builder("test", &password);

            assert_eq!(result.map(|_| ()), expected);
        }
    }
}
