use crate::{config::*, prelude::*, util::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Represents a [Service Check](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/service-checks-and-host/intro-to-service-checks/index.html#overview) in Opsview.
///
/// Defines a check used to perform monitoring tests on services within the Opsview monitoring
/// system. Service Checks are the primary means of monitoring services or [`Host`] resources within
/// Opsview.
///
/// This struct defines the structure for a `ServiceCheck` entity as used in Opsview.
///
/// # Example
/// ```rust
/// use opsview::config::{Plugin, ServiceCheck, ServiceGroup};
/// use opsview::prelude::*;
///
/// let my_plugin = Plugin::builder()
///   .name("My Plugin")
///   .build()
///   .unwrap();
///
/// let my_service_group = ServiceGroup::builder()
///   .name("My Service Group")
///   .build()
///   .unwrap();
///   
/// let service_check = ServiceCheck::builder()
///   .name("My Service Check")
///   .args("--foo")
///   .plugin(my_plugin)
///   .servicegroup(my_service_group)
///   .build()
///   .unwrap();
///
/// assert_eq!(service_check.name, "My Service Check".to_string());
/// ```
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ServiceCheck {
    // Required fields ---------------------------------------------------------------------------//
    /// The unique name of the `ServiceCheck`.
    pub name: String,

    // Semi-optional fields ----------------------------------------------------------------------//
    // These fields are required when creating a new object, but optional because they are not
    // always returned by the Opsview API.
    /// The arguments for the `ServiceCheck`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<String>,

    /// The [`Plugin`] used by the `ServiceCheck`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<Plugin>,

    /// the [`ServiceGroupRef`] that the `ServiceCheck` belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servicegroup: Option<ServiceGroupRef>,

    // Optional fields ---------------------------------------------------------------------------//
    /// An integer indicating from which failure the `ServiceCheck` should alert.
    /// Default: `Some(1)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub alert_from_failure: Option<u64>,

    /// Optional [`VariableRef`] for the `ServiceCheck`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<VariableRef>,

    /// A string indicating how the rate should be calculated.
    /// `Some("no")`.
    ///
    /// # Available options:
    /// * `no` - Do not calculate rate.
    /// * `per_second` - Calculate rate per second.
    /// * `per_minute` - Calculate rate per minute.
    /// * `per_hour` - Calculate rate per hour.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculate_rate: Option<String>,

    /// Optional `ServiceCheckRef` from which this `ServiceCheck` is cascaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascaded_from: Option<ServiceCheckRef>,

    /// The number of check attempts for the `ServiceCheck`.
    /// Default: `Some(1)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub check_attempts: Option<u64>,

    /// A boolean indicating whether to check freshness.
    /// Default: `Some(true)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub check_freshness: Option<bool>,

    /// The check interval for the `ServiceCheck`.
    /// Default: `Some(300)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub check_interval: Option<u64>,

    /// The [`TimePeriodRef`] for the `ServiceCheck`.
    /// Default: `Some(TimePeriodRef{ name: "24x7".to_string() })`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_period: Option<TimePeriodRef>,

    // TODO: Make this a private field and provide a getter/setter for it since the API will not
    // accept this field to be changed after creation.
    /// The [`CheckType`] for the `ServiceCheck`.
    /// Default: `Some(CheckType::Active)`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checktype: Option<CheckType>,

    /// Optional critical comparison for the `ServiceCheck`.
    ///
    /// # Available options:
    /// * `eq` - Equal to.
    /// * `ne` - Not equal to.
    /// * `regex` - Regular expression.
    /// * `==` - Equal to.
    /// * `<` - Less than.
    /// * `>` - Greater than.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical_comparison: Option<String>,

    /// Optional critical value for the `ServiceCheck`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical_value: Option<String>,

    /// [`ConfigRefMap`] of `ServiceCheckRef` objects on which this `ServiceCheck` depends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<ConfigRefMap<ServiceCheckRef>>,

    /// Optional description of the `ServiceCheck`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional [event handler](https://docs.itrsgroup.com/docs/opsview/6.8.9/monitoring/event-handlers/index.html#Heading-overview) for the `ServiceCheck`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_handler: Option<String>,

    /// Optional boolean indicating whether to always execute the [event handler](https://docs.itrsgroup.com/docs/opsview/6.8.9/monitoring/event-handlers/index.html#Heading-overview).
    /// `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub event_handler_always_exec: Option<bool>,

    /// Optional boolean indicating whether to enable [flap detection](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/service-checks-and-host/active-checks/index.html#flap-detection).
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub flap_detection_enabled: Option<bool>,

    /// TODO: Undocumented field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freshness_type: Option<String>,

    /// [`ConfigRefMap`] of [`HostRef`] objects associated with this `ServiceCheck`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: Option<ConfigRefMap<HostRef>>,

    /// [`ConfigRefMap`] of [`HostTemplateRef`] objects associated with this `ServiceCheck`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosttemplates: Option<ConfigRefMap<HostTemplateRef>>,

    /// Optional boolean indicating whether to invert the results of the `ServiceCheck`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub invertresults: Option<bool>,

    /// [`ConfigRefMap`] of [`HashtagRef`] objects associated with this `ServiceCheck`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<ConfigRefMap<HashtagRef>>,

    /// Optional SNMP poll check label for the `ServiceCheck`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// TODO: Undocumented field.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub markdown_filter: Option<bool>,

    /// Optional interval for the `ServiceCheck` notifications.
    /// TODO: This field needs better documentation.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub notification_interval: Option<u64>,

    /// Optional notification options for the `ServiceCheck`.
    ///
    /// # Available options:
    /// * `w` - Send notifications on warning state.
    /// * `c` - Send notifications on critical state.
    /// * `r` - Send notifications on recovery.
    /// * `u` - Send notifications on unknown state.
    /// * `f` - Send notifications on flap detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_options: Option<String>,

    /// Optional [`TimePeriodRef`] for notifications for the `ServiceCheck`.
    /// Default: `Some(TimePeriodRef{ name: "24x7".to_string() })`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_period: Option<TimePeriodRef>,

    /// Optional OID for the `ServiceCheck`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<String>,

    /// Optional retry check interval for the `ServiceCheck`, in seconds.
    /// Default: `Some(300)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub retry_check_interval: Option<u64>,

    /// Optional boolean indicating if the arguments to the `ServiceCheck` are sensitive.
    /// `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub sensitive_arguments: Option<bool>,

    /// [`ConfigObjectMap`] of [`SNMPTrapRule`]s for the `ServiceCheck`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snmptraprules: Option<ConfigObjectMap<SNMPTrapRule>>,

    /// Optional stale [`ServiceCheckState`] for the `ServiceCheck`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale_state: Option<ServiceCheckState>,

    /// Optional stale text for the `ServiceCheck`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale_text: Option<String>,

    /// Optional stale threshold seconds for the `ServiceCheck`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub stale_threshold_seconds: Option<u64>,

    /// Optional string containing the `ServiceCheck` stalking options.
    ///
    /// # Available options:
    /// * `o` - Stalk on OK state.
    /// * `w` - Stalk on warning state.
    /// * `c` - Stalk on critical state.
    /// * `u` - Stalk on unknown state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stalking: Option<String>,

    /// Optional boolean indicating whether the `ServiceCheck` is volatile.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub volatile: Option<bool>,

    /// Optional warning comparison for the `ServiceCheck`.
    ///
    /// # Available options:
    /// * `==` - Equal to.
    /// * `<` - Less than.
    /// * `>` - Greater than.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning_comparison: Option<String>,

    /// Optional warning value for the `ServiceCheck`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning_value: Option<String>,

    // Read-only fields --------------------------------------------------------------------------//
    /// The unique identifier of the `ServiceCheck`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,

    /// A reference string unique to this `ServiceCheck`.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    /// A boolean indicating whether the `ServiceCheck` is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}
impl Default for ServiceCheck {
    fn default() -> Self {
        let tp = TimePeriodRef::from(
            TimePeriod::minimal("24x7").expect("Failed to create TimePeriod with the name 24x7"),
        );
        ServiceCheck {
            name: "".to_string(),
            alert_from_failure: Some(1),
            calculate_rate: Some("no".to_string()),
            check_attempts: Some(1),
            check_freshness: Some(true),
            check_interval: Some(300),
            check_period: Some(tp.clone()),
            checktype: Some(CheckType::Active),
            event_handler_always_exec: Some(false),
            notification_period: Some(tp),
            retry_check_interval: Some(300),
            sensitive_arguments: Some(false),
            // All other fields are None by default.
            args: None,
            attribute: None,
            cascaded_from: None,
            critical_comparison: None,
            critical_value: None,
            dependencies: None,
            description: None,
            event_handler: None,
            flap_detection_enabled: None,
            freshness_type: None,
            hosts: None,
            hosttemplates: None,
            invertresults: None,
            keywords: None,
            label: None,
            markdown_filter: None,
            notification_interval: None,
            notification_options: None,
            oid: None,
            plugin: None,
            servicegroup: None,
            snmptraprules: None,
            stale_state: None,
            stale_text: None,
            stale_threshold_seconds: None,
            stalking: None,
            volatile: None,
            warning_comparison: None,
            warning_value: None,
            id: None,
            ref_: None,
            uncommitted: None,
        }
    }
}

/// Enables the creation of a [`ServiceCheck`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for ServiceCheck {}

impl ConfigObject for ServiceCheck {
    type Builder = ServiceCheckBuilder;

    /// Returns a builder for constructing a [`ServiceCheck`] object.
    ///
    /// # Returns
    /// A [`ServiceCheckBuilder`] object.
    fn builder() -> Self::Builder {
        ServiceCheckBuilder::new()
    }

    /// Provides the configuration path for a [`ServiceCheck`] object within the Opsview system.
    ///
    /// # Returns
    /// A string representing the API path where service checks are configured.
    fn config_path() -> Option<String> {
        Some("/config/servicecheck".to_string())
    }

    /// Returns a minimal `ServiceCheck` object with only the name set.
    ///
    /// # Arguments
    /// * `name` - Name of the [`ServiceCheck`].
    ///
    /// # Returns
    /// A minimal `ServiceCheck` object with only the name set, and the rest of the fields in their
    /// default states.
    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            name: validate_and_trim_servicecheck_name(name)?,
            ..Default::default()
        })
    }

    /// Returns the unique name of the [`ServiceCheck`] object.
    ///
    /// This name is used to identify the `ServiceCheck` when building the `HashMap` for an
    /// [`ConfigObjectMap`].
    ///
    /// # Returns
    /// A string representing the unique name of the `ServiceCheck`.
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl Persistent for ServiceCheck {
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
        Some(SERVICECHECK_NAME_REGEX_STR.to_string())
    }

    fn validated_name(&self, name: &str) -> Result<String, OpsviewConfigError> {
        validate_and_trim_servicecheck_name(name)
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

impl PersistentMap for ConfigObjectMap<ServiceCheck> {
    fn config_path() -> Option<String> {
        Some("/config/servicecheck".to_string())
    }
}

/// Builder for creating instances of [`ServiceCheck`].
///
/// Provides a fluent interface for constructing a `ServiceCheck` object with optional fields.
#[derive(Clone, Debug)]
pub struct ServiceCheckBuilder {
    // Required fields ---------------------------------------------------------------------------//
    name: Option<String>,
    args: Option<String>,
    plugin: Option<Plugin>,
    servicegroup: Option<ServiceGroupRef>,
    // Optional fields ---------------------------------------------------------------------------//
    alert_from_failure: Option<u64>,
    attribute: Option<VariableRef>,
    calculate_rate: Option<String>,
    cascaded_from: Option<ServiceCheckRef>,
    check_attempts: Option<u64>,
    check_freshness: Option<bool>,
    check_interval: Option<u64>,
    check_period: Option<TimePeriodRef>,
    checktype: Option<CheckType>,
    critical_comparison: Option<String>,
    critical_value: Option<String>,
    dependencies: Option<ConfigRefMap<ServiceCheckRef>>,
    description: Option<String>,
    event_handler: Option<String>,
    event_handler_always_exec: Option<bool>,
    flap_detection_enabled: Option<bool>,
    freshness_type: Option<String>,
    hosts: Option<ConfigRefMap<HostRef>>,
    hosttemplates: Option<ConfigRefMap<HostTemplateRef>>,
    invertresults: Option<bool>,
    keywords: Option<ConfigRefMap<HashtagRef>>,
    label: Option<String>,
    markdown_filter: Option<bool>,
    notification_interval: Option<u64>,
    notification_options: Option<String>,
    notification_period: Option<TimePeriodRef>,
    oid: Option<String>,
    retry_check_interval: Option<u64>,
    sensitive_arguments: Option<bool>,
    snmptraprules: Option<ConfigObjectMap<SNMPTrapRule>>,
    stale_state: Option<ServiceCheckState>,
    stale_text: Option<String>,
    stale_threshold_seconds: Option<u64>,
    stalking: Option<String>,
    volatile: Option<bool>,
    warning_comparison: Option<String>,
    warning_value: Option<String>,
}

impl Default for ServiceCheckBuilder {
    fn default() -> Self {
        let tp = TimePeriodRef::from(
            TimePeriod::minimal("24x7")
                .expect("Failed to create a TimePeriod with the name '24x7'"),
        );

        ServiceCheckBuilder {
            alert_from_failure: Some(1),
            calculate_rate: Some("no".to_string()),
            check_attempts: Some(1),
            check_freshness: Some(true),
            check_interval: Some(300),
            check_period: Some(tp.clone()),
            checktype: Some(CheckType::Active),
            event_handler_always_exec: Some(false),
            notification_period: Some(tp),
            retry_check_interval: Some(300),
            sensitive_arguments: Some(false),
            // All other fields are None by default.
            args: None,
            attribute: None,
            cascaded_from: None,
            critical_comparison: None,
            critical_value: None,
            dependencies: None,
            description: None,
            event_handler: None,
            flap_detection_enabled: None,
            freshness_type: None,
            hosts: None,
            hosttemplates: None,
            invertresults: None,
            keywords: None,
            label: None,
            markdown_filter: None,
            name: None,
            notification_interval: None,
            notification_options: None,
            oid: None,
            plugin: None,
            servicegroup: None,
            snmptraprules: None,
            stale_state: None,
            stale_text: None,
            stale_threshold_seconds: None,
            stalking: None,
            volatile: None,
            warning_comparison: None,
            warning_value: None,
        }
    }
}

impl Builder for ServiceCheckBuilder {
    type ConfigObject = ServiceCheck;

    /// Creates a new instance of [`ServiceCheckBuilder`] with default values.
    ///
    /// Initializes a new builder for creating a [`ServiceCheck`] object with all fields in their
    /// default state.
    fn new() -> Self {
        ServiceCheckBuilder::default()
    }

    /// Sets the name field.
    ///
    /// # Arguments
    /// * `name` - The name of the `ServiceCheck`.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Builds a new `ServiceCheck` object using the values specified in the `ServiceCheckBuilder`.
    ///
    /// # Returns
    /// A `Result` containing the new `ServiceCheck` object, or an `Error` if the name field is not
    /// set.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;
        let args = require_field(&self.args, "args")?;
        let plugin = require_field(&self.plugin, "plugin")?;
        let servicegroup = require_field(&self.servicegroup, "servicegroup")?;

        let validated_calculate_rate =
            validate_opt_string(self.calculate_rate, validate_calculate_rate)?;

        let validated_critical_comparison = validate_opt_string(
            self.critical_comparison,
            validate_and_trim_critical_comparison,
        )?;

        let validated_critical_value =
            validate_opt_string(self.critical_value, validate_and_trim_critical_value)?;

        let validated_description =
            validate_opt_string(self.description, validate_and_trim_description)?;

        let validated_label =
            validate_opt_string(self.label, validate_and_trim_servicecheck_label)?;

        let validated_notification_options = validate_opt_string(
            self.notification_options,
            validate_and_trim_servicecheck_notification_options,
        )?;

        let validated_oid = validate_opt_string(self.oid, validate_servicecheck_oid)?;

        let validated_stalking =
            validate_opt_string(self.stalking, validate_and_trim_servicecheck_stalking)?;

        let validated_warning_comparison = validate_opt_string(
            self.warning_comparison,
            validate_and_trim_warning_comparison,
        )?;

        let validated_warning_value =
            validate_opt_string(self.warning_value, validate_and_trim_warning_value)?;

        Ok(ServiceCheck {
            name: validate_and_trim_servicecheck_name(&name)?,
            args: Some(validate_servicecheck_args(&args)?),
            plugin: Some(plugin),
            servicegroup: Some(servicegroup),
            alert_from_failure: self.alert_from_failure,
            attribute: self.attribute,
            calculate_rate: validated_calculate_rate,
            cascaded_from: self.cascaded_from,
            check_attempts: self.check_attempts,
            check_freshness: self.check_freshness,
            check_interval: self.check_interval,
            check_period: self.check_period,
            checktype: self.checktype,
            critical_comparison: validated_critical_comparison,
            critical_value: validated_critical_value,
            dependencies: self.dependencies,
            description: validated_description,
            event_handler: self.event_handler,
            event_handler_always_exec: self.event_handler_always_exec,
            flap_detection_enabled: self.flap_detection_enabled,
            freshness_type: self.freshness_type,
            hosts: self.hosts,
            hosttemplates: self.hosttemplates,
            invertresults: self.invertresults,
            keywords: self.keywords,
            label: validated_label,
            markdown_filter: self.markdown_filter,
            notification_interval: self.notification_interval,
            notification_options: validated_notification_options,
            notification_period: self.notification_period,
            oid: validated_oid,
            retry_check_interval: self.retry_check_interval,
            sensitive_arguments: self.sensitive_arguments,
            snmptraprules: self.snmptraprules,
            stale_state: self.stale_state,
            stale_text: self.stale_text,
            stale_threshold_seconds: self.stale_threshold_seconds,
            stalking: validated_stalking,
            volatile: self.volatile,
            warning_comparison: validated_warning_comparison,
            warning_value: validated_warning_value,
            id: None,
            ref_: None,
            uncommitted: None,
        })
    }
}

impl ServiceCheckBuilder {
    /// Sets the alert_from_failure field.
    ///
    /// # Arguments
    /// * `alert_from_failure` - The failure at which alerts should start for this `ServiceCheck`.
    pub fn alert_from_failure(mut self, alert_from_failure: u64) -> Self {
        self.alert_from_failure = Some(alert_from_failure);
        self
    }

    /// Sets the args field.
    ///
    /// # Arguments
    /// * `args` - The arguments for the `ServiceCheck`.
    pub fn args(mut self, args: &str) -> Self {
        self.args = Some(args.to_string());
        self
    }

    /// Sets the attribute field.
    ///
    /// # Arguments
    /// * `attribute` - A variable for the `ServiceCheck`.
    pub fn attribute(mut self, attribute: Variable) -> Self {
        self.attribute = Some(VariableRef::from(attribute));
        self
    }

    /// Sets the calculate_rate field.
    ///
    /// # Arguments
    /// * `calculate_rate` - A boolean indicating if rate should be calculated or not.
    pub fn calculate_rate(mut self, calculate_rate: &str) -> Self {
        self.calculate_rate = Some(calculate_rate.to_string());
        self
    }

    /// Sets the cascaded_from field.
    ///
    /// # Arguments
    /// * `cascaded_from` - The `ServiceCheck` from which this `ServiceCheck` is cascaded.
    pub fn cascaded_from(mut self, cascaded_from: ServiceCheck) -> Self {
        self.cascaded_from = Some(ServiceCheckRef::from(cascaded_from));
        self
    }

    /// Sets the check_attempts field.
    ///
    /// # Arguments
    /// * `check_attempts` - The number of check attempts for the `ServiceCheck`.
    pub fn check_attempts(mut self, check_attempts: u64) -> Self {
        self.check_attempts = Some(check_attempts);
        self
    }

    /// Sets the check_freshness field.
    ///
    /// # Arguments
    /// * `check_freshness` - Boolean indicating whether to check freshness.
    pub fn check_freshness(mut self, check_freshness: bool) -> Self {
        self.check_freshness = Some(check_freshness);
        self
    }

    /// Sets the check_interval field.
    ///
    /// # Arguments
    /// * `check_interval` - The check interval for the `ServiceCheck`.
    pub fn check_interval(mut self, check_interval: u64) -> Self {
        self.check_interval = Some(check_interval);
        self
    }

    /// Sets the check_period field.
    ///
    /// # Arguments
    /// * `check_period` - The [`TimePeriod`] for the `ServiceCheck`.
    pub fn check_period(mut self, check_period: TimePeriod) -> Self {
        self.check_period = Some(TimePeriodRef::from(check_period));
        self
    }

    /// Sets the checktype field.
    ///
    /// # Arguments
    /// * `checktype` - The [`CheckType`] for the `ServiceCheck`.
    pub fn checktype(mut self, checktype: CheckType) -> Self {
        self.checktype = Some(checktype);
        self
    }

    /// Sets the critical_comparison field.
    ///
    /// # Arguments
    /// * `critical_comparison` - The critical comparison for the `ServiceCheck`.
    pub fn clear_alert_from_failure(mut self) -> Self {
        self.alert_from_failure = None;
        self
    }

    /// Clears the args field.
    pub fn clear_args(mut self) -> Self {
        self.args = None;
        self
    }

    /// Clears the attribute field.
    pub fn clear_attribute(mut self) -> Self {
        self.attribute = None;
        self
    }

    /// Clears the calculate_rate field.
    pub fn clear_calculate_rate(mut self) -> Self {
        self.calculate_rate = None;
        self
    }

    /// Clears the cascaded_from field.
    pub fn clear_cascaded_from(mut self) -> Self {
        self.cascaded_from = None;
        self
    }

    /// Clears the check_attempts field.
    pub fn clear_check_attempts(mut self) -> Self {
        self.check_attempts = None;
        self
    }

    /// Clears the check_freshness field.
    pub fn clear_check_freshness(mut self) -> Self {
        self.check_freshness = None;
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

    /// Clears the checktype field.
    pub fn clear_checktype(mut self) -> Self {
        self.checktype = None;
        self
    }

    /// Clears the critical_comparison field.
    pub fn clear_critical_comparison(mut self) -> Self {
        self.critical_comparison = None;
        self
    }

    /// Clears the critical_value field.
    pub fn clear_critical_value(mut self) -> Self {
        self.critical_value = None;
        self
    }

    /// Clears the dependencies field.
    pub fn clear_dependencies(mut self) -> Self {
        self.dependencies = None;
        self
    }

    /// Clears the description field.
    pub fn clear_description(mut self) -> Self {
        self.description = None;
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

    /// Clears the freshness_type field.
    pub fn clear_freshness_type(mut self) -> Self {
        self.freshness_type = None;
        self
    }

    /// Clears the hosts field.
    pub fn clear_hosts(mut self) -> Self {
        self.hosts = None;
        self
    }

    /// Clears the hosttemplates field.
    pub fn clear_hosttemplates(mut self) -> Self {
        self.hosttemplates = None;
        self
    }

    /// Clears the invertresults field.
    pub fn clear_invertresults(mut self) -> Self {
        self.invertresults = None;
        self
    }

    /// Clears the keywords field.
    pub fn clear_keywords(mut self) -> Self {
        self.keywords = None;
        self
    }

    /// Clears the label field.
    pub fn clear_label(mut self) -> Self {
        self.label = None;
        self
    }

    /// Clears the markdown_filter field.
    pub fn clear_markdown_filter(mut self) -> Self {
        self.markdown_filter = None;
        self
    }

    /// Clears the name field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
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

    /// Clears the oid field.
    pub fn clear_oid(mut self) -> Self {
        self.oid = None;
        self
    }

    /// Clears the plugin field.
    pub fn clear_plugin(mut self) -> Self {
        self.plugin = None;
        self
    }

    /// Clears the retry_check_interval field.
    pub fn clear_retry_check_interval(mut self) -> Self {
        self.retry_check_interval = None;
        self
    }

    /// Clears the sensitive_arguments field.
    pub fn clear_sensitive_arguments(mut self) -> Self {
        self.sensitive_arguments = None;
        self
    }

    /// Clears the servicegroup field.
    pub fn clear_servicegroup(mut self) -> Self {
        self.servicegroup = None;
        self
    }

    /// Clears the snmptraprules field.
    pub fn clear_snmptraprules(mut self) -> Self {
        self.snmptraprules = None;
        self
    }

    /// Clears the stale_state field.
    pub fn clear_stale_state(mut self) -> Self {
        self.stale_state = None;
        self
    }

    /// Clears the stale_text field.
    pub fn clear_stale_text(mut self) -> Self {
        self.stale_text = None;
        self
    }

    /// Clears the stale_threshold_seconds field.
    pub fn clear_stale_threshold_seconds(mut self) -> Self {
        self.stale_threshold_seconds = None;
        self
    }

    /// Clears the stalking field.
    pub fn clear_stalking(mut self) -> Self {
        self.stalking = None;
        self
    }

    /// Clears the volatile field.
    pub fn clear_volatile(mut self) -> Self {
        self.volatile = None;
        self
    }

    /// Clears the warning_comparison field.
    pub fn clear_warning_comparison(mut self) -> Self {
        self.warning_comparison = None;
        self
    }

    /// Clears the warning_value field.
    pub fn clear_warning_value(mut self) -> Self {
        self.warning_value = None;
        self
    }

    /// Sets the critical_comparison field.
    ///
    /// # Arguments
    /// * `critical_comparison` - The critical comparison for the `ServiceCheck`.
    pub fn critical_comparison(mut self, critical_comparison: &str) -> Self {
        self.critical_comparison = Some(critical_comparison.to_string());
        self
    }

    /// Sets the critical_value field.
    ///
    /// # Arguments
    /// * `critical_value` - The critical value for the `ServiceCheck`.
    pub fn critical_value(mut self, critical_value: &str) -> Self {
        self.critical_value = Some(critical_value.to_string());
        self
    }

    /// Sets the dependencies field.
    ///
    /// # Arguments
    /// * `dependencies` - A reference to a [`ConfigObjectMap`] of `ServiceCheck` objects on which this `ServiceCheck`
    pub fn dependencies(mut self, dependencies: &ConfigObjectMap<ServiceCheck>) -> Self {
        self.dependencies = Some(dependencies.into());
        self
    }

    /// Sets the description field.
    ///
    /// # Arguments
    /// * `description` - The description of the `ServiceCheck`.
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Sets the event_handler field.
    ///
    /// # Arguments
    /// * `event_handler` - The [event handler](https://docs.itrsgroup.com/docs/opsview/6.8.9/monitoring/event-handlers/index.html#Heading-overview) for the `ServiceCheck`.
    pub fn event_handler(mut self, event_handler: &str) -> Self {
        self.event_handler = Some(event_handler.to_string());
        self
    }

    /// Sets the event_handler_always_exec field.
    ///
    /// # Arguments
    /// * `event_handler_always_exec` - Boolean indicating whether to always execute the [event handler](https://docs.itrsgroup.com/docs/opsview/6.8.9/monitoring/event-handlers/index.html#Heading-overview).
    pub fn event_handler_always_exec(mut self, event_handler_always_exec: bool) -> Self {
        self.event_handler_always_exec = Some(event_handler_always_exec);
        self
    }

    /// Sets the flap_detection_enabled field.
    ///
    /// # Arguments
    /// * `flap_detection_enabled` - Boolean indicating whether to enable [flap detection](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/service-checks-and-host/active-checks/index.html#flap-detection).
    pub fn flap_detection_enabled(mut self, flap_detection_enabled: bool) -> Self {
        self.flap_detection_enabled = Some(flap_detection_enabled);
        self
    }

    /// Sets the freshness_type field:
    ///
    /// # Arguments
    /// * `freshness_type` - The freshness type for the `ServiceCheck`.
    pub fn freshness_type(mut self, freshness_type: &str) -> Self {
        self.freshness_type = Some(freshness_type.to_string());
        self
    }

    /// Sets the hosts field.
    ///
    /// # Arguments
    /// * `hosts` - A reference to a [`ConfigObjectMap`] of [`Host`] objects associated with this `ServiceCheck`.
    pub fn hosts(mut self, hosts: &ConfigObjectMap<Host>) -> Self {
        self.hosts = Some(hosts.into());
        self
    }

    /// Sets the hosttemplates field.
    ///
    /// # Arguments
    /// * `hosttemplates` - A reference to a [`ConfigObjectMap`] of [`HostTemplate`] objects associated with this service
    pub fn hosttemplates(mut self, hosttemplates: &ConfigObjectMap<HostTemplate>) -> Self {
        self.hosttemplates = Some(hosttemplates.into());
        self
    }

    /// Sets the invertresults field.
    ///
    /// # Arguments
    /// * `invertresults` - Boolean indicating whether to invert the results of the `ServiceCheck`.
    pub fn invertresults(mut self, invertresults: bool) -> Self {
        self.invertresults = Some(invertresults);
        self
    }

    /// Sets the keywords field.
    ///
    /// # Arguments
    /// * `keywords` - A reference to a [`ConfigObjectMap`] of [`Hashtag`] objects associated with this `ServiceCheck`.
    pub fn keywords(mut self, keywords: &ConfigObjectMap<Hashtag>) -> Self {
        self.keywords = Some(keywords.into());
        self
    }

    /// Sets the label field.
    ///
    /// # Arguments
    /// * `label` - The label for the `ServiceCheck`.
    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    /// Sets the markdown_filter field.
    ///
    /// # Arguments
    /// * `markdown_filter` - A boolean indicating if the markdown filter should be used.
    pub fn markdown_filter(mut self, markdown_filter: bool) -> Self {
        self.markdown_filter = Some(markdown_filter);
        self
    }

    /// Sets the notification_interval field.
    ///
    /// # Arguments
    /// * `notification_interval` - The interval for the `ServiceCheck` notifications, in seconds.
    pub fn notification_interval(mut self, notification_interval: u64) -> Self {
        self.notification_interval = Some(notification_interval);
        self
    }

    /// Sets the notification_options field.
    ///
    /// # Arguments
    /// * `notification_options` - The notification options for the `ServiceCheck`.
    ///
    /// # Available options:
    /// * `w` - Send notifications on warning state.
    /// * `c` - Send notifications on critical state.
    /// * `r` - Send notifications on recovery.
    /// * `u` - Send notifications on unknown state.
    /// * `f` - Send notifications on flap detection.
    pub fn notification_options(mut self, notification_options: &str) -> Self {
        self.notification_options = Some(notification_options.to_string());
        self
    }

    /// Sets the notification_period field.
    ///
    /// # Arguments
    /// * `notification_period` - The [`TimePeriod`] for notifications for the `ServiceCheck`.
    pub fn notification_period(mut self, notification_period: TimePeriod) -> Self {
        self.notification_period = Some(TimePeriodRef::from(notification_period));
        self
    }

    /// Sets the oid field.
    ///
    /// # Arguments
    /// * `oid` - The OID for the `ServiceCheck`.
    pub fn oid(mut self, oid: &str) -> Self {
        self.oid = Some(oid.to_string());
        self
    }

    /// Sets the plugin field.
    ///
    /// # Arguments
    /// * `plugin` - The [`Plugin`] for the `ServiceCheck`.
    pub fn plugin(mut self, plugin: Plugin) -> Self {
        self.plugin = Some(plugin);
        self
    }

    /// Sets the retry_check_interval field.
    ///
    /// # Arguments
    /// * `retry_check_interval` - The retry check interval for the `ServiceCheck`.
    pub fn retry_check_interval(mut self, retry_check_interval: u64) -> Self {
        self.retry_check_interval = Some(retry_check_interval);
        self
    }

    /// Sets the sensitive_arguments field.
    ///
    /// # Arguments
    /// * `sensitive_arguments` - A boolean indicating whether the arguments to the `ServiceCheck` are sensitive.
    pub fn sensitive_arguments(mut self, sensitive_arguments: bool) -> Self {
        self.sensitive_arguments = Some(sensitive_arguments);
        self
    }

    /// Sets the servicegroup field.
    ///
    /// # Arguments
    /// * `servicegroup` - The [`ServiceGroup`] for the `ServiceCheck`.
    pub fn servicegroup(mut self, servicegroup: ServiceGroup) -> Self {
        self.servicegroup = Some(ServiceGroupRef::from(servicegroup));
        self
    }

    /// Sets the snmptraprules field.
    ///
    /// # Arguments
    /// * `snmptraprules` - [`ConfigObjectMap`] of [`SNMPTrapRule`]s for the `ServiceCheck`.
    pub fn snmptraprules(mut self, snmptraprules: ConfigObjectMap<SNMPTrapRule>) -> Self {
        self.snmptraprules = Some(snmptraprules);
        self
    }

    /// Sets the stale_state field.
    ///
    /// # Arguments
    /// * `stale_state` - The stale [`ServiceCheckState`] for the `ServiceCheck`:
    pub fn stale_state(mut self, stale_state: ServiceCheckState) -> Self {
        self.stale_state = Some(stale_state);
        self
    }

    /// Sets the stale_text field.
    ///
    /// # Arguments
    /// * `stale_text` - The stale text for the `ServiceCheck`.
    pub fn stale_text(mut self, stale_text: &str) -> Self {
        self.stale_text = Some(stale_text.to_string());
        self
    }

    /// Sets the stale_threshold_seconds field.
    ///
    /// # Arguments
    /// * `stale_threshold_seconds` - The stale threshold seconds for the `ServiceCheck`.
    pub fn stale_threshold_seconds(mut self, stale_threshold_seconds: u64) -> Self {
        self.stale_threshold_seconds = Some(stale_threshold_seconds);
        self
    }

    /// Sets the stalking field.
    ///
    /// # Arguments
    /// * `stalking` - A string containing the `ServiceCheck` stalking options.
    ///
    /// # Available options:
    /// * `o` - Stalk on OK state.
    /// * `w` - Stalk on warning state.
    /// * `c` - Stalk on critical state.
    /// * `u` - Stalk on unknown state.
    pub fn stalking(mut self, stalking: &str) -> Self {
        self.stalking = Some(stalking.to_string());
        self
    }

    /// Sets the volatile field.
    ///
    /// # Arguments
    /// * `volatile` - Boolean indicating whether the `ServiceCheck` is volatile.
    pub fn volatile(mut self, volatile: bool) -> Self {
        self.volatile = Some(volatile);
        self
    }

    /// Sets the warning_comparison field.
    ///
    /// # Arguments
    /// * `warning_comparison` - The warning comparison for the `ServiceCheck`.
    pub fn warning_comparison(mut self, warning_comparison: &str) -> Self {
        self.warning_comparison = Some(warning_comparison.to_string());
        self
    }

    /// Sets the warning_value field.
    ///
    /// # Arguments
    /// * `warning_value` - The warning value for the `ServiceCheck`.
    pub fn warning_value(mut self, warning_value: &str) -> Self {
        self.warning_value = Some(warning_value.to_string());
        self
    }
}

/// A reference version of [`ServiceCheck`] that is used when passing or retrieving a
/// [`ServiceCheck`] object as part of another object.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ServiceCheckRef {
    name: String,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
}

impl Default for ServiceCheckRef {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            ref_: None,
        }
    }
}

/// Enables the creation of a [`ServiceCheckRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for ServiceCheckRef {}

impl ConfigRef for ServiceCheckRef {
    type FullObject = ServiceCheck;

    /// Returns the reference string of the [`ServiceCheckRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`ServiceCheckRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`ServiceCheckRef`] object.
    ///
    /// This name is used to identify the `ServiceCheckRef` when building the `HashMap` for a
    /// [`ConfigRefMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl From<ServiceCheck> for ServiceCheckRef {
    fn from(full_object: ServiceCheck) -> Self {
        Self {
            name: full_object.name.clone(),
            ref_: full_object.ref_.clone(),
        }
    }
}

impl From<Arc<ServiceCheck>> for ServiceCheckRef {
    fn from(item: Arc<ServiceCheck>) -> Self {
        let cmd: ServiceCheck = Arc::try_unwrap(item).unwrap_or_else(|arc| (*arc).clone());
        ServiceCheckRef::from(cmd)
    }
}

impl From<&ConfigObjectMap<ServiceCheck>> for ConfigRefMap<ServiceCheckRef> {
    fn from(servicechecks: &ConfigObjectMap<ServiceCheck>) -> Self {
        ref_map_from(servicechecks)
    }
}

/// A reference version of [`ServiceCheck`] that is used when passing or retrieving a
/// [`ServiceCheck`] object as part of [`Host`] or [`HostTemplate`] object.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ServiceCheckHostRef {
    name: String,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_handler: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exception: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    remove_servicecheck: Option<bool>,
    // TODO: timed_exception field
    //       "timed_exception": {
    //         "properties": {
    //             "args": {
    //                 "type": "string"
    //             },
    //             "timeperiod": {
    //                 "$ref": "#/definitions/reference",
    //                 "required": true
    //             }
    //         },
    //         "type": [
    //             "object",
    //             "null"
    //         ]
    //     }
}

impl Default for ServiceCheckHostRef {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            ref_: None,
            event_handler: None,
            exception: None,
            remove_servicecheck: Some(false),
        }
    }
}

/// Enables the creation of a [`ServiceCheckHostRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for ServiceCheckHostRef {}

impl ConfigRef for ServiceCheckHostRef {
    type FullObject = ServiceCheck;

    /// Returns the reference string of the [`ServiceCheckHostRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`ServiceCheckHostRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`ServiceCheckHostRef`] object.
    ///
    /// This name is used to identify the `ServiceCheckHostRef` when building the `HashMap` for a
    /// [`ConfigRefMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl From<ServiceCheck> for ServiceCheckHostRef {
    fn from(full_object: ServiceCheck) -> Self {
        Self {
            name: full_object.name.clone(),
            ref_: full_object.ref_.clone(),
            event_handler: full_object.event_handler.clone(),
            ..Default::default()
        }
    }
}

impl From<Arc<ServiceCheck>> for ServiceCheckHostRef {
    fn from(full_object: Arc<ServiceCheck>) -> Self {
        Self {
            name: full_object.name.clone(),
            ref_: full_object.ref_.clone(),
            event_handler: full_object.event_handler.clone(),
            ..Default::default()
        }
    }
}

impl From<&ConfigObjectMap<ServiceCheck>> for ConfigRefMap<ServiceCheckHostRef> {
    fn from(servicechecks: &ConfigObjectMap<ServiceCheck>) -> Self {
        ref_map_from(servicechecks)
    }
}

impl ServiceCheckHostRef {
    /// Clears the `event_handler` field.
    pub fn clear_event_handler(mut self) -> Self {
        self.event_handler = None;
        self
    }

    /// Sets the `event_handler` field.
    ///
    /// # Arguments
    /// * `event_handler` - The event handler for the `ServiceCheckHostRef`.
    pub fn set_event_handler(mut self, event_handler: &str) -> Self {
        self.event_handler = Some(event_handler.to_string());
        self
    }

    /// Gets the value of the `event_handler` field.
    pub fn get_event_handler(&self) -> Option<String> {
        self.event_handler.clone()
    }

    /// Clears the `exception` field.
    pub fn clear_exception(mut self) -> Self {
        self.exception = None;
        self
    }

    /// Sets the `exception` field.
    ///
    /// # Arguments
    /// * `exception` - The exception for the `ServiceCheckHostRef`.
    pub fn set_exception(mut self, exception: &str) -> Self {
        self.exception = Some(exception.to_string());
        self
    }

    /// Gets the value of the `exception` field.
    pub fn get_exception(&self) -> Option<String> {
        self.exception.clone()
    }

    /// Clears the `remove_servicecheck` field.
    pub fn clear_remove_servicecheck(mut self) -> Self {
        self.remove_servicecheck = None;
        self
    }

    /// Sets the `remove_servicecheck` field.
    ///
    /// # Arguments
    /// * `b`: A boolean indicating whethere or not the `ServiceCheck` should be removed.
    pub fn set_remove_servicecheck(mut self, b: bool) -> Self {
        self.remove_servicecheck = Some(b);
        self
    }

    /// Gets the value of the `remove_servicecheck` field.
    pub fn get_remove_servicecheck(&self) -> Option<bool> {
        self.remove_servicecheck
    }
}

fn validate_calculate_rate(s: &str) -> Result<String, OpsviewConfigError> {
    match s {
        "no" | "per_second" | "per_minute" | "per_hour" => Ok(s.to_string()),
        _ => Err(OpsviewConfigError::InvalidCalculateRate),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let servicecheck = ServiceCheck::default();
        assert_eq!(servicecheck.name, "");
    }

    #[test]
    fn test_minimal() {
        let servicecheck = ServiceCheck::minimal("test");
        assert_eq!(servicecheck.unwrap().name, "test");
    }
}
