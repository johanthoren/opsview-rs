use super::{HostRef, ServiceCheckRef, TimeZone};
use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Represents a [Time Period](https://docs.itrsgroup.com/docs/opsview/current/configuration/time-periods/time-periods/index.html) in Opsview.
///
/// Time periods are used within Opsview Monitor for one of two purposes; determining when a
/// [`super::Host`] or [`super::ServiceCheck`] is being actively monitored, and determining when
/// notifications should be sent.
///
/// For example, if a Host only needs to be monitored during office hours, then an administrator can
/// create a Time Period called ‘Working Hours’ that is 9am-5pm, Monday to Friday, and then ‘apply’
/// this `TimePeriod` to the Host via a field called the ‘Check Period’, i.e. ‘What *period *of time
/// should I actively *check *this Host’. Service Checks can be configured to have a fixed Time
/// Period or to inherit the check period from the Host. This will apply to all Service Checks
/// whether the Service Checks are applied individually via the ‘Service Checks’ tab or in bulk via
/// the addition of a Host template.
///
/// There are seven days within the `TimePeriod`, Sunday through to Saturday. In each day, the hours
/// can be defined in an ‘HH:MM’ format, and comma-separated for multiple ranges. For example,
/// ‘00:00-24:00’ means ‘all day’, ‘09:00-17:00’ means ‘9am to 5pm’, ‘00:00-09:00,17:00-23:59’ means
/// ’not 9-5pm’, and so forth.
///
/// An important point to note is the hours defined do not go over the midnight boundary, for
/// example “22:00-02:00” is not valid - instead use ‘22:00-23:59’ on the first day, and
/// ‘00:00-02:00’ on the following day.
///
/// This struct represents the structure of a time period entity as used in the [Opsview
/// API](https://docs.itrsgroup.com/docs/opsview/current/rest-api/config/api-config-time-period/index.html).
///
/// # Example
/// ```rust
/// use opsview::config::TimePeriod;
/// use opsview::prelude::*;
///
/// let time_period = TimePeriod::builder()
///    .name("MyTimePeriod")
///    .alias("My Time Period Alias")
///    .monday("00:00-09:00,17:00-24:00")
///    .tuesday("00:00-09:00,17:00-24:00")
///    .wednesday("00:00-09:00,17:00-24:00")
///    .thursday("00:00-09:00,17:00-24:00")
///    .friday("00:00-09:00,17:00-24:00")
///    .saturday("00:00-09:00,17:00-24:00")
///    .sunday("00:00-09:00,17:00-24:00")
///    .build()
///    .unwrap();
///
/// assert_eq!(time_period.name, "MyTimePeriod".to_string());
/// assert_eq!(time_period.alias, Some("My Time Period Alias".to_string()));
/// assert_eq!(time_period.monday, Some("00:00-09:00,17:00-24:00".to_string()));
/// ```
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct TimePeriod {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the `TimePeriod`.
    pub name: String,

    // Optional fields ---------------------------------------------------------------------------//
    /// Optional alias for the `TimePeriod`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    /// Optional string representing the `TimePeriod` for Friday.
    ///
    /// Example: `"00:00-09:00,17:00-24:00"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friday: Option<String>,

    /// Optional string representing the `TimePeriod` for Monday.
    ///
    /// Example: `"00:00-09:00,17:00-24:00"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monday: Option<String>,

    /// Optional string representing the `TimePeriod` for Saturday.
    ///
    /// Example: `"00:00-09:00,17:00-24:00"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturday: Option<String>,

    /// Optional string representing the `TimePeriod` for Sunday.
    ///
    /// Example: `"00:00-09:00,17:00-24:00"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunday: Option<String>,

    /// Optional string representing the `TimePeriod` for Thursday.
    ///
    /// Example: `"00:00-09:00,17:00-24:00"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thursday: Option<String>,

    /// Optional string representing the `TimePeriod` for Tuesday.
    ///
    /// Example: `"00:00-09:00,17:00-24:00"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuesday: Option<String>,

    /// Optional string representing the `TimePeriod` for Wednesday.
    ///
    /// Example: `"00:00-09:00,17:00-24:00"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wednesday: Option<String>,

    /// Optional [`TimeZone`] in which the `TimePeriod` is defined.
    ///
    /// Default: `Some(TimeZone{ name: "SYSTEM".to_string(), ref_: "/rest/config/timezone/1".to_string() })`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<TimeZone>,

    // Read-only fields --------------------------------------------------------------------------//
    /// [`ConfigRefMap`] of [`HostRef`] objects associated with this `TimePeriod` for their checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_check_periods: Option<ConfigRefMap<HostRef>>,

    /// [`ConfigRefMap`] of [`HostRef`] objects associated with this `TimePeriod` for their notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_notification_periods: Option<ConfigRefMap<HostRef>>,

    /// The unique identifier of the `TimePeriod`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,

    // TODO: This is not in the API documentation, but is returned in the JSON response.
    //       It is not clear what this field is used for.
    /// Optional boolean indicating whether the `TimePeriod` is locked.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub object_locked: Option<bool>,

    /// A reference string unique to this `TimePeriod`.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    /// [`ConfigRefMap`] of [`ServiceCheckRef`] objects associated with this `TimePeriod` for their checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servicecheck_check_periods: Option<ConfigRefMap<ServiceCheckRef>>,

    /// [`ConfigRefMap`] of [`ServiceCheckRef`] objects associated with this `TimePeriod` for their notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servicecheck_notification_periods: Option<ConfigRefMap<ServiceCheckRef>>,

    /// A boolean indicating whether the `TimePeriod` is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

impl Default for TimePeriod {
    fn default() -> Self {
        let tz = TimeZone::builder()
            .name("SYSTEM")
            .ref_("/rest/config/timezone/1")
            .build()
            .unwrap();

        TimePeriod {
            name: String::new(),
            alias: None,
            friday: None,
            host_check_periods: None,
            host_notification_periods: None,
            id: None,
            monday: None,
            object_locked: None,
            ref_: None,
            saturday: None,
            servicecheck_check_periods: None,
            servicecheck_notification_periods: None,
            sunday: None,
            thursday: None,
            tuesday: None,
            uncommitted: None,
            wednesday: None,
            zone: Some(tz),
        }
    }
}

/// Enables the creation of a [`TimePeriod`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for TimePeriod {}

impl ConfigObject for TimePeriod {
    type Builder = TimePeriodBuilder;

    /// Returns a builder for constructing a [`TimePeriod`] object.
    ///
    /// # Returns
    /// A [`TimePeriodBuilder`] object.
    fn builder() -> Self::Builder {
        TimePeriodBuilder::new()
    }

    /// Provides the configuration path for a [`TimePeriod`] object within the Opsview system.
    ///
    /// # Returns
    /// A string representing the API path where time periods are configured.
    fn config_path() -> Option<String> {
        Some("/config/timeperiod".to_string())
    }

    /// Returns the unique name of the [`TimePeriod`] object.
    ///
    /// This name is used to identify the `TimePeriod` when building the `HashMap` for an
    /// `ConfigObjectMap`.
    ///
    /// # Returns
    /// A string representing the unique name of the `TimePeriod`.
    fn unique_name(&self) -> String {
        self.name.clone()
    }

    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            name: validate_and_trim_timeperiod_name(name)?,
            ..Default::default()
        })
    }
}

impl Persistent for TimePeriod {
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
        Some(TIMEPERIOD_NAME_REGEX_STR.to_string())
    }

    fn validated_name(&self, name: &str) -> Result<String, OpsviewConfigError> {
        validate_and_trim_timeperiod_name(name)
    }

    fn set_name(&mut self, new_name: &str) -> Result<String, OpsviewConfigError> {
        self.name = self.validated_name(new_name)?;
        Ok(self.name.clone())
    }

    fn clear_readonly(&mut self) {
        self.host_check_periods = None;
        self.host_notification_periods = None;
        self.id = None;
        self.object_locked = None;
        self.ref_ = None;
        self.servicecheck_check_periods = None;
        self.servicecheck_notification_periods = None;
        self.uncommitted = None;
    }
}

impl PersistentMap for ConfigObjectMap<TimePeriod> {
    fn config_path() -> Option<String> {
        Some("/config/timeperiod".to_string())
    }
}

/// Builder for [`TimePeriod`] objects.
///
/// Provides a fluent interface for constructing a `TimePeriod` object with optional fields.
#[derive(Clone, Debug, Default)]
pub struct TimePeriodBuilder {
    alias: Option<String>,
    friday: Option<String>,
    monday: Option<String>,
    name: Option<String>,
    saturday: Option<String>,
    sunday: Option<String>,
    thursday: Option<String>,
    tuesday: Option<String>,
    wednesday: Option<String>,
    zone: Option<TimeZone>,
}

impl Builder for TimePeriodBuilder {
    type ConfigObject = TimePeriod;

    /// Creates a new [`TimePeriodBuilder`] instance used to construct a [`TimePeriod`] object.
    ///
    /// # Returns
    /// A `TimePeriodBuilder` instance.
    fn new() -> Self {
        Self::default()
    }

    /// Sets the name field for the `TimePeriod`.
    ///
    /// # Arguments
    /// * `name` - The name for the `TimePeriod`.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Consumes the builder and returns a [`TimePeriod`] object.
    ///
    /// # Returns
    /// A `TimePeriod` object with the configured settings.
    ///
    /// # Errors
    /// Returns a `OpsviewConfigError` if the name field is not set.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;
        let validated_alias = validate_opt_string(self.alias, validate_and_trim_timeperiod_alias)?;
        let validated_monday =
            validate_opt_string(self.monday, validate_and_trim_timeperiod_weekday)?;
        let validated_tuesday =
            validate_opt_string(self.tuesday, validate_and_trim_timeperiod_weekday)?;
        let validated_wednesday =
            validate_opt_string(self.wednesday, validate_and_trim_timeperiod_weekday)?;
        let validated_thursday =
            validate_opt_string(self.thursday, validate_and_trim_timeperiod_weekday)?;
        let validated_friday =
            validate_opt_string(self.friday, validate_and_trim_timeperiod_weekday)?;
        let validated_saturday =
            validate_opt_string(self.saturday, validate_and_trim_timeperiod_weekday)?;
        let validated_sunday =
            validate_opt_string(self.sunday, validate_and_trim_timeperiod_weekday)?;

        Ok(TimePeriod {
            name: validate_and_trim_timeperiod_name(&name)?,
            alias: validated_alias,
            friday: validated_friday,
            monday: validated_monday,
            saturday: validated_saturday,
            sunday: validated_sunday,
            thursday: validated_thursday,
            tuesday: validated_tuesday,
            wednesday: validated_wednesday,
            zone: self.zone,
            host_check_periods: None,
            host_notification_periods: None,
            id: None,
            object_locked: None,
            ref_: None,
            servicecheck_check_periods: None,
            servicecheck_notification_periods: None,
            uncommitted: None,
        })
    }
}

impl TimePeriodBuilder {
    /// Sets the alias field for the `TimePeriod`.
    ///
    /// # Arguments
    /// * `alias` - The alias for the `TimePeriod`.
    pub fn alias(mut self, alias: &str) -> Self {
        self.alias = Some(alias.to_string());
        self
    }

    /// Clears the alias field for the `TimePeriod`.
    pub fn clear_alias(mut self) -> Self {
        self.alias = None;
        self
    }

    /// Clears the friday field for the `TimePeriod`.
    pub fn clear_friday(mut self) -> Self {
        self.friday = None;
        self
    }

    /// Clears the monday field for the `TimePeriod`.
    pub fn clear_monday(mut self) -> Self {
        self.monday = None;
        self
    }

    /// Clears the name field for the `TimePeriod`.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Clears the saturday field for the `TimePeriod`.
    pub fn clear_saturday(mut self) -> Self {
        self.saturday = None;
        self
    }

    /// Clears the sunday field for the `TimePeriod`.
    pub fn clear_sunday(mut self) -> Self {
        self.sunday = None;
        self
    }

    /// Clears the thursday field for the `TimePeriod`.
    pub fn clear_thursday(mut self) -> Self {
        self.thursday = None;
        self
    }

    /// Clears the tuesday field for the `TimePeriod`.
    pub fn clear_tuesday(mut self) -> Self {
        self.tuesday = None;
        self
    }

    /// Clears the wednesday field for the `TimePeriod`.
    pub fn clear_wednesday(mut self) -> Self {
        self.wednesday = None;
        self
    }

    /// Clears the zone field for the `TimePeriod`.
    pub fn clear_zone(mut self) -> Self {
        self.zone = None;
        self
    }

    /// Sets the friday field for the `TimePeriod`.
    ///
    /// # Arguments
    /// * `friday` - The friday field for the `TimePeriod`.
    ///
    /// Example: `"00:00-09:00,17:00-24:00"`
    pub fn friday(mut self, friday: &str) -> Self {
        self.friday = Some(friday.to_string());
        self
    }

    /// Sets the monday field for the `TimePeriod`.
    ///
    /// # Arguments
    /// * `monday` - The monday field for the `TimePeriod`.
    ///
    /// Example: `"00:00-09:00,17:00-24:00"`
    pub fn monday(mut self, monday: &str) -> Self {
        self.monday = Some(monday.to_string());
        self
    }

    /// Sets the saturday field for the `TimePeriod`.
    ///
    /// # Arguments
    /// * `saturday` - The saturday field for the `TimePeriod`.
    ///
    /// Example: `"00:00-09:00,17:00-24:00"`
    pub fn saturday(mut self, saturday: &str) -> Self {
        self.saturday = Some(saturday.to_string());
        self
    }

    /// Sets the sunday field for the `TimePeriod`.
    ///
    /// # Arguments
    /// * `sunday` - The sunday field for the `TimePeriod`.
    ///
    /// Example: `"00:00-09:00,17:00-24:00"`
    pub fn sunday(mut self, sunday: &str) -> Self {
        self.sunday = Some(sunday.to_string());
        self
    }

    /// Sets the thursday field for the `TimePeriod`.
    ///
    /// # Arguments
    /// * `thursday` - The thursday field for the `TimePeriod`.
    ///
    /// Example: `"00:00-09:00,17:00-24:00"`
    pub fn thursday(mut self, thursday: &str) -> Self {
        self.thursday = Some(thursday.to_string());
        self
    }

    /// Sets the tuesday field for the `TimePeriod`.
    ///
    /// # Arguments
    /// * `tuesday` - The tuesday field for the `TimePeriod`.
    ///
    /// Example: `"00:00-09:00,17:00-24:00"`
    pub fn tuesday(mut self, tuesday: &str) -> Self {
        self.tuesday = Some(tuesday.to_string());
        self
    }

    /// Sets the wednesday field for the `TimePeriod`.
    ///
    /// # Arguments
    /// * `wednesday` - The wednesday field for the `TimePeriod`.
    ///
    /// Example: `"00:00-09:00,17:00-24:00"`
    pub fn wednesday(mut self, wednesday: &str) -> Self {
        self.wednesday = Some(wednesday.to_string());
        self
    }

    /// Sets the zone field for the `TimePeriod`.
    ///
    /// # Arguments
    /// * `zone` - The [`TimeZone`] in which the `TimePeriod` is defined.
    pub fn zone(mut self, zone: TimeZone) -> Self {
        self.zone = Some(zone);
        self
    }
}

/// A reference version of [`TimePeriod`] that is used when passing or retrieving a
/// [`TimePeriod`] object as part of another object.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct TimePeriodRef {
    name: String,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
}

/// Enables the creation of a [`TimePeriodRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for TimePeriodRef {}

impl ConfigRef for TimePeriodRef {
    type FullObject = TimePeriod;

    /// Returns the reference string of the [`TimePeriodRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`TimePeriodRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`TimePeriodRef`] object.
    ///
    /// This name is used to identify the `TimePeriodRef` when building the `HashMap` for a
    /// [`ConfigRefMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl From<TimePeriod> for TimePeriodRef {
    /// Creates a [`TimePeriodRef`] object from a [`TimePeriod`] object.
    ///
    /// # Arguments
    /// * `timeperiod` - The [`TimePeriod`] object from which to create the [`TimePeriodRef`] object.
    ///
    /// # Returns
    /// A [`TimePeriodRef`] object.
    fn from(timeperiod: TimePeriod) -> Self {
        TimePeriodRef {
            name: timeperiod.name.clone(),
            ref_: timeperiod.ref_.clone(),
        }
    }
}

impl From<Arc<TimePeriod>> for TimePeriodRef {
    /// Creates a [`TimePeriodRef`] object from an [`Arc`] wrapped [`TimePeriod`] object.
    ///
    /// # Arguments
    /// * `timeperiod` - The [`Arc`] wrapped [`TimePeriod`] object from which to create the [`TimePeriodRef`] object.
    ///
    /// # Returns
    /// A [`TimePeriodRef`] object.
    fn from(timeperiod: Arc<TimePeriod>) -> Self {
        TimePeriodRef {
            name: timeperiod.name.clone(),
            ref_: timeperiod.ref_.clone(),
        }
    }
}

impl From<&ConfigObjectMap<TimePeriod>> for ConfigRefMap<TimePeriodRef> {
    fn from(timeperiods: &ConfigObjectMap<TimePeriod>) -> Self {
        ref_map_from(timeperiods)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeperiod_default() {
        let tz = TimeZone::builder()
            .name("SYSTEM")
            .ref_("/rest/config/timezone/1")
            .build()
            .unwrap();

        let timeperiod = TimePeriod::default();

        assert_eq!(timeperiod.name, String::new());
        assert_eq!(timeperiod.alias, None);
        assert_eq!(timeperiod.friday, None);
        assert_eq!(timeperiod.host_check_periods, None);
        assert_eq!(timeperiod.host_notification_periods, None);
        assert_eq!(timeperiod.id, None);
        assert_eq!(timeperiod.monday, None);
        assert_eq!(timeperiod.object_locked, None);
        assert_eq!(timeperiod.ref_, None);
        assert_eq!(timeperiod.saturday, None);
        assert_eq!(timeperiod.servicecheck_check_periods, None);
        assert_eq!(timeperiod.servicecheck_notification_periods, None);
        assert_eq!(timeperiod.sunday, None);
        assert_eq!(timeperiod.thursday, None);
        assert_eq!(timeperiod.tuesday, None);
        assert_eq!(timeperiod.uncommitted, None);
        assert_eq!(timeperiod.wednesday, None);
        assert_eq!(timeperiod.zone, Some(tz));
    }

    #[test]
    fn test_timeperiod_minimal() {
        let timeperiod = TimePeriod::minimal("MyTimePeriod");

        assert_eq!(timeperiod.unwrap().name, "MyTimePeriod".to_string());
    }

    #[test]
    fn test_is_valid_timeperiod_name() {
        // Valid names
        assert!(validate_and_trim_timeperiod_name("24x7").is_ok());

        // Invalid names
        assert!(validate_and_trim_timeperiod_name("My Time Period").is_err());
    }

    #[test]
    fn test_valid_timeperiods() {
        let valid_strings = [
            "00:00-09:00,17:00-24:00",
            "00:00-24:00",
            "00:00-09:00,10:00-11:00,12:00-24:00",
        ];

        for s in valid_strings {
            println!("Testing timeperiod string '{}'", s);
            let tp = TimePeriod::builder()
                .name("foo")
                .monday(s)
                .tuesday(s)
                .wednesday(s)
                .thursday(s)
                .friday(s)
                .saturday(s)
                .sunday(s)
                .build();

            assert!(tp.is_ok());
        }
    }

    #[test]
    fn test_invalid_timeperiods() {
        let invalid_strings = ["foo", "00:00-22:00 23:00-24:00", "10:00-11:00Z", ""];

        for s in invalid_strings {
            println!("Testing timeperiod string '{}'", s);
            let tp = TimePeriod::builder()
                .name("foo")
                .monday(s)
                .tuesday(s)
                .wednesday(s)
                .thursday(s)
                .friday(s)
                .saturday(s)
                .sunday(s)
                .build();

            assert!(tp.is_err());
        }
    }
}
