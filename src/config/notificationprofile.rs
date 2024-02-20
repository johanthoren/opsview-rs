use crate::{config::*, prelude::*, util::*};
use decimal_percentage::Percentage;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;

/// Represents a [notification
/// profile](https://docs.itrsgroup.com/docs/opsview/current/monitoring/notifications/notification-profiles/index.html)
/// in Opsview.
///
/// This struct defines the structure for a `NotificationProfile` entity as used in Opsview.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct NotificationProfile {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the `NotificationProfile`.
    pub name: String,

    // Optional fields ---------------------------------------------------------------------------//
    /// Optional boolean indicating whether all [`BSMComponent`]s are included in the `NotificationProfile`.
    ///
    /// Default: `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub all_business_components: Option<bool>,

    /// Optional boolean indicating whether all [`BSMService`]s are included in the `NotificationProfile`.
    ///
    /// Default: `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub all_business_services: Option<bool>,

    /// Optional boolean indicating whether all [`HostGroup`]s are included in the `NotificationProfile`.
    ///
    /// Default: `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub all_hostgroups: Option<bool>,

    /// Optional boolean indicating whether all [`Hashtag`]s are included in the `NotificationProfile`.
    ///
    /// Default: `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub all_keywords: Option<bool>,

    /// Optional boolean indicating whether all [`ServiceGroup`]s are included in the `NotificationProfile`.
    ///
    /// Default: `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub all_servicegroups: Option<bool>,

    /// Optional Percentage indicating the availability level below which [`BSMComponent`]s will be
    /// included in the `NotificationProfile`.
    ///
    /// Default: `Some(Percentage::from(99.999))`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_component_availability_below: Option<Percentage>,

    // TODO: Add validation of this field.
    /// Optional string indicating the options for [`BSMComponent`]s in the `NotificationProfile`.
    ///
    /// Default: `Some("f,i".to_string())`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_component_options: Option<String>,

    /// Optional interval in seconds indicating the renotification interval for [`BSMComponent`]s
    /// in the `NotificationProfile`.
    ///
    /// Default: `Some(1800)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub business_component_renotification_interval: Option<u64>,

    /// [`ConfigRefMap`] of [`BSMComponentRef`] objects associated with this `NotificationProfile`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_components: Option<ConfigRefMap<BSMComponentRef>>,

    /// Optional Percentage indicating the availability level below which [`BSMService`]s will be
    /// included in the `NotificationProfile`.
    ///
    /// Default: `Some(Percentage::from(99.999))`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_service_availability_below: Option<Percentage>,

    // TODO: Add validation of this field.
    /// Optional string indicating the options for [`BSMService`]s in the `NotificationProfile`.
    ///
    /// Default: `Some("o,i".to_string())`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_service_options: Option<String>,

    /// Optional interval in seconds indicating the renotification interval for [`BSMService`]s
    /// in the `NotificationProfile`.
    ///
    /// Default: `Some(1800)`.
    pub business_service_renotification_interval: Option<u64>,

    /// [`ConfigRefMap`] of [`BSMServiceRef`] objects associated with this `NotificationProfile`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_services: Option<ConfigRefMap<BSMServiceRef>>,

    /// Optional string indicating the host notification options for the `NotificationProfile`.
    ///
    /// Default: `Some("d,r,f".to_string())`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_notification_options: Option<String>,

    /// [`ConfigRefMap`] of [`HostGroupRef`] objects associated with this `NotificationProfile`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostgroups: Option<ConfigRefMap<HostGroupRef>>,

    /// Optional boolean indicating whether component notes are included.
    ///
    /// Default: `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub include_component_notes: Option<bool>,

    /// Optional boolean indicating whether service notes are included.
    ///
    /// Default: `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub include_service_notes: Option<bool>,

    /// [`ConfigRefMap`] of [`HashtagRef`] objects associated with this `NotificationProfile`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<ConfigRefMap<HashtagRef>>,

    /// Optional integer indicating the notification level for the `NotificationProfile`.
    ///
    /// It seems as this field corresponds to the count at which this profile should start receiving
    /// notifications.
    ///
    /// Default: `Some(1)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub notification_level: Option<u64>,

    /// Optional integer indicating the notification level stop for the `NotificationProfile`.
    ///
    /// It seems as this field corresponds to the count after which this profile should stop
    /// receiving notifications, with 0 representing no limit.
    ///
    /// Default: `Some(0)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub notification_level_stop: Option<u64>,

    /// Optional [`TimePeriodRef`] object indicating the notification period for the `NotificationProfile`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_period: Option<TimePeriodRef>,

    /// [`ConfigRefMap`] of [`NotificationMethodRef`] objects associated with this `NotificationProfile`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notificationmethods: Option<ConfigRefMap<NotificationMethodRef>>,

    /// Optional integer indicating the renotification interval for the `NotificationProfile`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub renotification_interval_seconds: Option<u64>,

    /// Optional [`RoleRef`] object indicating the role for the `NotificationProfile`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<RoleRef>,

    /// Optional string indicating the service notification options for the `NotificationProfile`.
    ///
    /// Default: `Some("w".to_string())`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_notification_options: Option<String>,

    /// [`ConfigRefMap`] of [`ServiceGroupRef`] objects associated with this `NotificationProfile`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servicegroups: Option<ConfigRefMap<ServiceGroupRef>>,

    // Read-only fields --------------------------------------------------------------------------//
    /// The unique identifier of the `NotificationProfile`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,

    /// A reference string unique to this `NotificationProfile`.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    /// A boolean indicating whether the `NotificationProfile` is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

impl Default for NotificationProfile {
    /// Returns a default `NotificationProfile` instance.
    fn default() -> Self {
        NotificationProfile {
            all_business_components: Some(false),
            all_business_services: Some(false),
            all_hostgroups: Some(false),
            all_keywords: Some(false),
            all_servicegroups: Some(false),
            business_component_availability_below: Some(Percentage::from(99.999)),
            business_component_options: Some("f,i".to_string()),
            business_component_renotification_interval: Some(1800),
            business_components: None,
            business_service_availability_below: Some(Percentage::from(99.999)),
            business_service_options: Some("o,i".to_string()),
            business_service_renotification_interval: Some(1800),
            business_services: None,
            host_notification_options: Some("d,r,f".to_string()),
            hostgroups: None,
            include_component_notes: Some(false),
            include_service_notes: Some(false),
            keywords: None,
            name: String::new(),
            notification_level: Some(1),
            notification_level_stop: Some(0),
            notification_period: None,
            notificationmethods: None,
            renotification_interval_seconds: None,
            role: None,
            service_notification_options: Some("w".to_string()),
            servicegroups: None,
            id: None,
            ref_: None,
            uncommitted: None,
        }
    }
}

/// Enables the creation of a [`NotificationProfile`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for NotificationProfile {}

impl ConfigObject for NotificationProfile {
    type Builder = NotificationProfileBuilder;

    /// Returns a builder for constructing a [`NotificationProfile`] object.
    ///
    /// # Returns
    /// A [`NotificationProfileBuilder`] object.
    fn builder() -> Self::Builder {
        NotificationProfileBuilder::new()
    }

    /// Provides the configuration path for a [`NotificationProfile`] object within the Opsview
    /// system.
    ///
    /// # Returns
    /// None, since [`NotificationProfile`] objects are not stored in the Opsview configuration
    /// except as part of other objects.
    fn config_path() -> Option<String> {
        None
    }

    /// Returns the unique name of the [`NotificationProfile`] object.
    ///
    /// This name is used to identify the `NotificationProfile` when building the `HashMap`
    /// for an [`ConfigObjectMap`].
    ///
    /// # Returns
    /// A string representing the unique name of the `NotificationProfile`.
    fn unique_name(&self) -> String {
        self.name.clone()
    }

    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            name: validate_and_trim_notificationprofile_name(name)?,
            ..NotificationProfile::default()
        })
    }
}

impl Persistent for NotificationProfile {
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
        Some(NOTIFICATIONPROFILE_NAME_REGEX_STR.to_string())
    }

    fn validated_name(&self, name: &str) -> Result<String, OpsviewConfigError> {
        validate_and_trim_notificationprofile_name(name)
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

impl PersistentMap for ConfigObjectMap<NotificationProfile> {
    fn config_path() -> Option<String> {
        None
    }
}

/// Builder for [`NotificationProfile`] objects.
///
/// This builder supports creating a `NotificationProfile` object in a declarative manner.
/// It is typically used when creating a new shared notification profile in the Opsview system.
///
/// # Example
/// ```rust
/// use opsview::config::NotificationProfile;
/// use opsview::prelude::*;
///
/// let shared_notification_profile = NotificationProfile::builder()
///    .name("My Shared Notification Profile")
///    .build()
///    .unwrap();
///
/// assert_eq!(shared_notification_profile.name, "My Shared Notification Profile".to_string());
/// ```
#[derive(Clone, Debug)]
pub struct NotificationProfileBuilder {
    all_business_components: Option<bool>,
    all_business_services: Option<bool>,
    all_hostgroups: Option<bool>,
    all_keywords: Option<bool>,
    all_servicegroups: Option<bool>,
    business_component_availability_below: Option<Percentage>,
    business_component_options: Option<String>,
    business_component_renotification_interval: Option<u64>,
    business_components: Option<ConfigRefMap<BSMComponentRef>>,
    business_service_availability_below: Option<Percentage>,
    business_service_options: Option<String>,
    business_service_renotification_interval: Option<u64>,
    business_services: Option<ConfigRefMap<BSMServiceRef>>,
    host_notification_options: Option<String>,
    hostgroups: Option<ConfigRefMap<HostGroupRef>>,
    include_component_notes: Option<bool>,
    include_service_notes: Option<bool>,
    keywords: Option<ConfigRefMap<HashtagRef>>,
    name: Option<String>,
    notification_level: Option<u64>,
    notification_level_stop: Option<u64>,
    notification_period: Option<TimePeriodRef>,
    notificationmethods: Option<ConfigRefMap<NotificationMethodRef>>,
    renotification_interval_seconds: Option<u64>,
    role: Option<RoleRef>,
    service_notification_options: Option<String>,
    servicegroups: Option<ConfigRefMap<ServiceGroupRef>>,
}

impl Default for NotificationProfileBuilder {
    /// Creates a new [`NotificationProfileBuilder`] instance with default values.
    ///
    /// Initializes a new builder for creating a [`NotificationProfile`] object with all fields
    /// in their default state.
    fn default() -> Self {
        NotificationProfileBuilder {
            all_business_components: Some(false),
            all_business_services: Some(false),
            all_hostgroups: Some(false),
            all_keywords: Some(false),
            all_servicegroups: Some(false),
            business_component_availability_below: Some(Percentage::from(99.999)),
            business_component_options: Some("f,i".to_string()),
            business_component_renotification_interval: Some(1800),
            business_components: None,
            business_service_availability_below: Some(Percentage::from(99.999)),
            business_service_options: Some("o,i".to_string()),
            business_service_renotification_interval: Some(1800),
            business_services: None,
            host_notification_options: Some("d,r,f".to_string()),
            hostgroups: None,
            include_component_notes: Some(false),
            include_service_notes: Some(false),
            keywords: None,
            name: None,
            notification_level: Some(1),
            notification_level_stop: Some(0),
            notification_period: None,
            notificationmethods: None,
            renotification_interval_seconds: None,
            role: None,
            service_notification_options: Some("w".to_string()),
            servicegroups: None,
        }
    }
}

impl Builder for NotificationProfileBuilder {
    type ConfigObject = NotificationProfile;

    /// Creates a new [`NotificationProfileBuilder`] instance used to construct a
    /// [`NotificationProfile`] object.
    ///
    /// # Returns
    /// A `NotificationProfileBuilder` instance.
    fn new() -> Self {
        NotificationProfileBuilder::default()
    }

    /// Sets the name field.
    ///
    /// # Arguments
    /// * `name` - Name of the `NotificationProfile`.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Consumes the builder and returns a [`NotificationProfile`] object.
    ///
    /// # Returns
    /// A `NotificationProfile` object constructed from the builder's configuration.
    ///
    /// # Errors
    /// This method will return an error if the name field is not set.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;

        self.bsm_component_fields_are_valid()?;
        self.bsm_service_fields_are_valid()?;

        let validated_host_notification_options = validate_opt_string(
            self.host_notification_options,
            validate_and_trim_host_notification_options,
        )?;

        let validated_service_notification_options = validate_opt_string(
            self.service_notification_options,
            validate_and_trim_servicecheck_notification_options,
        )?;

        Ok(NotificationProfile {
            name: validate_and_trim_notificationprofile_name(&name)?,
            all_business_components: self.all_business_components,
            all_business_services: self.all_business_services,
            all_hostgroups: self.all_hostgroups,
            all_keywords: self.all_keywords,
            all_servicegroups: self.all_servicegroups,
            business_component_availability_below: self.business_component_availability_below,
            business_component_options: self.business_component_options,
            business_component_renotification_interval: self
                .business_component_renotification_interval,
            business_components: self.business_components,
            business_service_availability_below: self.business_service_availability_below,
            business_service_options: self.business_service_options,
            business_service_renotification_interval: self.business_service_renotification_interval,
            business_services: self.business_services,
            host_notification_options: validated_host_notification_options,
            hostgroups: self.hostgroups,
            include_component_notes: self.include_component_notes,
            include_service_notes: self.include_service_notes,
            keywords: self.keywords,
            notification_level: self.notification_level,
            notification_level_stop: self.notification_level_stop,
            notification_period: self.notification_period,
            notificationmethods: self.notificationmethods,
            renotification_interval_seconds: self.renotification_interval_seconds,
            role: self.role,
            service_notification_options: validated_service_notification_options,
            servicegroups: self.servicegroups,
            id: None,
            ref_: None,
            uncommitted: None,
        })
    }
}

impl NotificationProfileBuilder {
    /// Sets the all_business_components field.
    ///
    /// # Arguments
    /// * `all_business_components` - Boolean indicating whether all [`BSMComponent`]s are included in the `NotificationProfile`.
    pub fn all_business_components(mut self, all_business_components: bool) -> Self {
        self.all_business_components = Some(all_business_components);
        self
    }

    /// Sets the all_keywords field. Alias for `all_keywords()`.
    ///
    /// # Arguments
    /// * `all_hashtags` - Boolean indicating whether all [`Hashtag`]s are included in the `NotificationProfile`.
    pub fn all_hashtags(mut self, all_hashtags: bool) -> Self {
        self.all_keywords = Some(all_hashtags);
        self
    }

    /// Sets the all_business_services field.
    ///
    /// # Arguments
    /// * `all_business_services` - Boolean indicating whether all [`BSMService`]s are included in the `NotificationProfile`.
    pub fn all_business_services(mut self, all_business_services: bool) -> Self {
        self.all_business_services = Some(all_business_services);
        self
    }

    /// Sets the all_hostgroups field.
    ///
    /// # Arguments
    /// * `all_hostgroups` - Boolean indicating whether all [`HostGroup`]s are included in the `NotificationProfile`.
    pub fn all_hostgroups(mut self, all_hostgroups: bool) -> Self {
        self.all_hostgroups = Some(all_hostgroups);
        self
    }

    /// Sets the all_keywords field.
    ///
    /// # Arguments
    /// * `all_keywords` - Boolean indicating whether all [`Hashtag`]s are included in the `NotificationProfile`.
    pub fn all_keywords(mut self, all_keywords: bool) -> Self {
        self.all_keywords = Some(all_keywords);
        self
    }

    /// Sets the all_servicegroups field.
    ///
    /// # Arguments
    /// * `all_servicegroups` - Boolean indicating whether all [`ServiceGroup`]s are included in the `NotificationProfile`.
    pub fn all_servicegroups(mut self, all_servicegroups: bool) -> Self {
        self.all_servicegroups = Some(all_servicegroups);
        self
    }

    /// Sets the business_component_availability_below field using a String slice.
    ///
    /// # Arguments
    /// * `business_component_availability_below` - &str indicating the availability level below which [`BSMComponent`]s will be included in the `NotificationProfile`.
    pub fn business_component_availability_below_from_str(
        mut self,
        business_component_availability_below: &str,
    ) -> Result<Self, OpsviewConfigError> {
        let p = Percentage::from_str(business_component_availability_below);
        match p {
            Ok(p) => {
                self.business_component_availability_below = Some(p);
                Ok(self)
            }
            Err(e) => Err(OpsviewConfigError::InvalidPercentage(
                business_component_availability_below.to_string(),
                e.to_string(),
            )),
        }
    }

    /// Sets the business_component_availability_below field.
    ///
    /// # Arguments
    /// * `business_component_availability_below` - Percentage indicating the availability level below which [`BSMComponent`]s will be included in the `NotificationProfile`.
    pub fn business_component_availability_below(
        mut self,
        business_component_availability_below: Percentage,
    ) -> Self {
        self.business_component_availability_below = Some(business_component_availability_below);
        self
    }

    /// Sets the business_component_options field.
    ///
    /// # Arguments
    /// * `business_component_options` - String indicating the options for [`BSMComponent`]s in the `NotificationProfile`.
    pub fn business_component_options(mut self, business_component_options: &str) -> Self {
        self.business_component_options = Some(business_component_options.to_string());
        self
    }

    /// Sets the business_component_renotification_interval field.
    ///
    /// # Arguments
    /// * `business_component_renotification_interval` - Integer indicating the renotification interval for [`BSMComponent`]s in the `NotificationProfile`.
    pub fn business_component_renotification_interval(
        mut self,
        business_component_renotification_interval: u64,
    ) -> Self {
        self.business_component_renotification_interval =
            Some(business_component_renotification_interval);
        self
    }

    /// Sets the business_components field.
    ///
    /// # Arguments
    /// * `business_components` - A reference to a [`ConfigObjectMap`] of [`BSMComponent`] objects associated with this `NotificationProfile`
    pub fn business_components(
        mut self,
        business_components: &ConfigObjectMap<BSMComponent>,
    ) -> Self {
        self.business_components = Some(business_components.into());
        self
    }

    /// Sets the business_service_availability_below field using a String slice.
    ///
    /// # Arguments
    /// * `business_service_availability_below` - &str indicating the availability level below which [`BSMService`]s will be included in the `NotificationProfile`.
    pub fn business_service_availability_below_from_str(
        mut self,
        business_service_availability_below: &str,
    ) -> Result<Self, OpsviewConfigError> {
        let p = Percentage::from_str(business_service_availability_below);
        match p {
            Ok(p) => {
                self.business_service_availability_below = Some(p);
                Ok(self)
            }
            Err(e) => Err(OpsviewConfigError::InvalidPercentage(
                business_service_availability_below.to_string(),
                e.to_string(),
            )),
        }
    }

    /// Sets the business_service_availability_below field.
    ///
    /// # Arguments
    /// * `business_service_availability_below` - Percentage indicating the availability level below which [`BSMService`]s will be included in the `NotificationProfile`.
    pub fn business_service_availability_below(
        mut self,
        business_service_availability_below: Percentage,
    ) -> Self {
        self.business_service_availability_below = Some(business_service_availability_below);
        self
    }

    /// Sets the business_service_options field.
    ///
    /// # Arguments
    /// * `business_service_options` - String indicating the options for [`BSMService`]s in the `NotificationProfile`.
    pub fn business_service_options(mut self, business_service_options: &str) -> Self {
        self.business_service_options = Some(business_service_options.to_string());
        self
    }

    /// Sets the business_service_renotification_interval field.
    ///
    /// # Arguments
    /// * `business_service_renotification_interval` - Integer indicating the renotification interval for [`BSMService`]s in the `NotificationProfile`.
    pub fn business_service_renotification_interval(
        mut self,
        business_service_renotification_interval: u64,
    ) -> Self {
        self.business_service_renotification_interval =
            Some(business_service_renotification_interval);
        self
    }

    /// Sets the business_services field.
    ///
    /// # Arguments
    /// * `business_services` - A reference to a [`ConfigObjectMap`] of `BSMService` objects associated with this `NotificationProfile`.
    pub fn business_services(mut self, business_services: &ConfigObjectMap<BSMService>) -> Self {
        self.business_services = Some(business_services.into());
        self
    }

    /// Sets the host_notification_options field.
    ///
    /// # Arguments
    /// * `host_notification_options` - String indicating the host notification options for the `NotificationProfile`.
    pub fn clear_all_business_components(mut self) -> Self {
        self.all_business_components = None;
        self
    }

    /// Clears the all_business_services field.
    pub fn clear_all_business_services(mut self) -> Self {
        self.all_business_services = None;
        self
    }

    /// Clears the all_hostgroups field.
    pub fn clear_all_hostgroups(mut self) -> Self {
        self.all_hostgroups = None;
        self
    }

    /// Clears the all_keywords field.
    pub fn clear_all_keywords(mut self) -> Self {
        self.all_keywords = None;
        self
    }

    /// Clears the all_servicegroups field.
    pub fn clear_all_servicegroups(mut self) -> Self {
        self.all_servicegroups = None;
        self
    }

    /// Clears the business_component_availability_below field.
    pub fn clear_business_component_availability_below(mut self) -> Self {
        self.business_component_availability_below = None;
        self
    }

    /// Clears the business_component_options field.
    pub fn clear_business_component_options(mut self) -> Self {
        self.business_component_options = None;
        self
    }

    /// Clears the business_component_renotification_interval field.
    pub fn clear_business_component_renotification_interval(mut self) -> Self {
        self.business_component_renotification_interval = None;
        self
    }

    /// Clears the business_components field.
    pub fn clear_business_components(mut self) -> Self {
        self.business_components = None;
        self
    }

    /// Clears the business_service_availability_below field.
    pub fn clear_business_service_availability_below(mut self) -> Self {
        self.business_service_availability_below = None;
        self
    }

    /// Clears the business_service_options field.
    pub fn clear_business_service_options(mut self) -> Self {
        self.business_service_options = None;
        self
    }

    /// Clears the business_service_renotification_interval field.
    pub fn clear_business_service_renotification_interval(mut self) -> Self {
        self.business_service_renotification_interval = None;
        self
    }

    /// Clears the business_services field.
    pub fn clear_business_services(mut self) -> Self {
        self.business_services = None;
        self
    }

    /// Clears the keywords field.
    pub fn clear_hashtags(mut self) -> Self {
        self.keywords = None;
        self
    }

    /// Clears the host_notification_options field.
    pub fn clear_host_notification_options(mut self) -> Self {
        self.host_notification_options = None;
        self
    }

    /// Clears the hostgroups field.
    pub fn clear_hostgroups(mut self) -> Self {
        self.hostgroups = None;
        self
    }

    /// Clears the include_component_notes field.
    pub fn clear_include_component_notes(mut self) -> Self {
        self.include_component_notes = None;
        self
    }

    /// Clears the include_service_notes field.
    pub fn clear_include_service_notes(mut self) -> Self {
        self.include_service_notes = None;
        self
    }

    /// Clears the keywords field.
    pub fn clear_keywords(mut self) -> Self {
        self.keywords = None;
        self
    }

    /// Clears the name field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Clears the notification_level field.
    pub fn clear_notification_level(mut self) -> Self {
        self.notification_level = None;
        self
    }

    /// Clears the notification_level_stop field.
    pub fn clear_notification_level_stop(mut self) -> Self {
        self.notification_level_stop = None;
        self
    }

    /// Clears the notification_period field.
    pub fn clear_notification_period(mut self) -> Self {
        self.notification_period = None;
        self
    }

    /// Clears the notificationmethods field.
    pub fn clear_notificationmethods(mut self) -> Self {
        self.notificationmethods = None;
        self
    }

    /// Clears the renotification_interval_seconds field.
    pub fn clear_renotification_interval_seconds(mut self) -> Self {
        self.renotification_interval_seconds = None;
        self
    }

    /// Clears the role field.
    pub fn clear_role(mut self) -> Self {
        self.role = None;
        self
    }

    /// Clears the service_notification_options field.
    pub fn clear_service_notification_options(mut self) -> Self {
        self.service_notification_options = None;
        self
    }

    /// Clears the servicegroups field.
    pub fn clear_servicegroups(mut self) -> Self {
        self.servicegroups = None;
        self
    }

    /// Sets the `keywords` field, an alias for `keywords`.
    ///
    /// # Arguments
    /// * `hashtags` - A reference to a [`ConfigObjectMap`] of `Hashtag` objects using `NotificationProfile`.
    pub fn hashtags(mut self, hashtags: &ConfigObjectMap<Hashtag>) -> Self {
        self.keywords = Some(hashtags.into());
        self
    }

    /// Sets the host_notification_options field.
    ///
    /// # Arguments
    /// * `host_notification_options` - String indicating the host notification options for the `NotificationProfile`.
    pub fn host_notification_options(mut self, host_notification_options: &str) -> Self {
        self.host_notification_options = Some(host_notification_options.to_string());
        self
    }

    /// Sets the hostgroups field.
    ///
    /// # Arguments
    /// * `hostgroups` - A reference to a [`ConfigObjectMap`] of `HostGroup` objects associated with this `NotificationProfile`.
    pub fn hostgroups(mut self, hostgroups: &ConfigObjectMap<HostGroup>) -> Self {
        self.hostgroups = Some(hostgroups.into());
        self
    }

    /// Sets the include_component_notes field.
    ///
    /// # Arguments
    /// * `include_component_notes` - Boolean indicating whether component notes are included in the `NotificationProfile`.
    pub fn include_component_notes(mut self, include_component_notes: bool) -> Self {
        self.include_component_notes = Some(include_component_notes);
        self
    }

    /// Sets the include_service_notes field.
    ///
    /// # Arguments
    /// * `include_service_notes` - Boolean indicating whether service notes are included in the `NotificationProfile`.
    pub fn include_service_notes(mut self, include_service_notes: bool) -> Self {
        self.include_service_notes = Some(include_service_notes);
        self
    }

    /// Sets the keywords field.
    ///
    /// # Arguments
    /// * `keywords` - A reference to a [`ConfigObjectMap`] of `Hashtag` objects using this `NotificationProfile`.
    pub fn keywords(mut self, keywords: &ConfigObjectMap<Hashtag>) -> Self {
        self.keywords = Some(keywords.into());
        self
    }

    /// Sets the notification_level field.
    ///
    /// # Arguments
    /// * `notification_level` - Integer indicating the notification level for the `NotificationProfile`.
    pub fn notification_level(mut self, notification_level: u64) -> Self {
        self.notification_level = Some(notification_level);
        self
    }

    /// Sets the notification_level_stop field.
    ///
    /// # Arguments
    /// * `notification_level_stop` - Integer indicating the notification level stop for the `NotificationProfile`.
    pub fn notification_level_stop(mut self, notification_level_stop: u64) -> Self {
        self.notification_level_stop = Some(notification_level_stop);
        self
    }

    /// Sets the notification_period field.
    ///
    /// # Arguments
    /// * `notification_period` - A [`TimePeriod`] object indicating the notification period for the `NotificationProfile`.
    pub fn notification_period(mut self, notification_period: TimePeriod) -> Self {
        self.notification_period = Some(TimePeriodRef::from(notification_period));
        self
    }

    /// Sets the notificationmethods field.
    ///
    /// # Arguments
    /// * `notificationmethods` - A reference to a [`ConfigObjectMap`] of `NotificationMethod` objects associated with this `NotificationProfile`.
    pub fn notificationmethods(
        mut self,
        notificationmethods: &ConfigObjectMap<NotificationMethod>,
    ) -> Self {
        self.notificationmethods = Some(notificationmethods.into());
        self
    }

    /// Sets the renotification_interval_seconds field.
    ///
    /// # Arguments
    /// * `renotification_interval_seconds` - Integer indicating the renotification interval for the `NotificationProfile`.
    pub fn renotification_interval_seconds(mut self, renotification_interval_seconds: u64) -> Self {
        self.renotification_interval_seconds = Some(renotification_interval_seconds);
        self
    }

    /// Sets the role field.
    ///
    /// # Arguments
    /// * `role` - The [`Role`] object indicating the role for the `NotificationProfile`.
    pub fn role(mut self, role: Role) -> Self {
        self.role = Some(RoleRef::from(role));
        self
    }

    /// Sets the service_notification_options field.
    ///
    /// # Arguments
    /// * `service_notification_options` - String indicating the service notification options for the `NotificationProfile`.
    pub fn service_notification_options(mut self, service_notification_options: &str) -> Self {
        self.service_notification_options = Some(service_notification_options.to_string());
        self
    }

    /// Sets the servicegroups field.
    ///
    /// # Arguments
    /// * `servicegroups` - A reference to a [`ConfigObjectMap`] of [`ServiceGroup`] objects associated with this `NotificationProfile`.
    pub fn servicegroups(mut self, servicegroups: &ConfigObjectMap<ServiceGroup>) -> Self {
        self.servicegroups = Some(servicegroups.into());
        self
    }
}

/// Extends the [`NotificationProfileBuilder`] object with validation methods that are shared with
/// [`SharedNotificationProfileBuilder`](super::SharedNotificationProfileBuilder).
pub trait NotificationProfileBuilderExt {
    /// Validates the fields related to [`BSMComponent`]s in the Profile:
    /// - `all_business_components`
    /// - `business_component_options`
    /// - `business_component_availability_below`
    /// - `business_component_renotification_interval`
    /// - `business_components`
    fn bsm_component_fields_are_valid(&self) -> Result<(), OpsviewConfigError> {
        self.bsm_component_selection_is_valid()?;
        self.bsm_component_availablity_below_is_valid()?;
        self.bsm_component_options_are_valid()
    }

    /// Asserts that the `Percentage` used for business_component_availability_below is between 0
    /// and 100.
    fn bsm_component_availablity_below_is_valid(&self) -> Result<(), OpsviewConfigError>;

    /// Validates the `bsm_component_options` field. The following values are valid:
    /// - `r` - Notify on recovery
    /// - `f` - Notify on failure
    /// - `i` - Notify on impact
    /// - `a` - Notify on availability dropping below a certain threshold`
    /// - `n` - Include notes in notification
    ///
    /// # Regex
    /// `'^[rfian](,[rfia])*$'`
    fn bsm_component_options_are_valid(&self) -> Result<(), OpsviewConfigError>;

    /// Verifies that the `all_business_components` and `business_components` fields are not in
    /// conflict.
    fn bsm_component_selection_is_valid(&self) -> Result<(), OpsviewConfigError>;

    /// Validates the fields related to [`BSMService`]s in the Profile:
    /// - `all_business_services`
    /// - `business_service_options`
    /// - `business_service_availability_below`
    /// - `business_service_renotification_interval`
    /// - `business_services`
    fn bsm_service_fields_are_valid(&self) -> Result<(), OpsviewConfigError> {
        self.bsm_service_selection_is_valid()?;
        self.bsm_service_availablity_below_is_valid()?;
        self.bsm_service_options_are_valid()
    }

    /// Asserts that the `Percentage` used for business_service_availability_below is between 0
    /// and 100.
    fn bsm_service_availablity_below_is_valid(&self) -> Result<(), OpsviewConfigError>;

    /// Validates the `bsm_service_options` field. The following values are valid:
    /// - `r` - Notify on recovery
    /// - `o` - Notify on offline
    /// - `i` - Notify on impact
    /// - `a` - Notify on availability dropping below a certain threshold`
    /// - `n` - Include notes in notification
    ///
    /// # Regex
    /// `'^[roian](,[roia])*$'
    fn bsm_service_options_are_valid(&self) -> Result<(), OpsviewConfigError>;

    /// Verifies that the `all_business_services` and `business_services` fields are not in
    /// conflict.
    fn bsm_service_selection_is_valid(&self) -> Result<(), OpsviewConfigError>;
}

impl NotificationProfileBuilderExt for NotificationProfileBuilder {
    fn bsm_component_availablity_below_is_valid(&self) -> Result<(), OpsviewConfigError> {
        match self.business_component_availability_below {
            None => Ok(()),
            Some(p) => percentage_between_0_and_100(p),
        }
    }

    fn bsm_component_options_are_valid(&self) -> Result<(), OpsviewConfigError> {
        match self.business_component_options {
            None => Ok(()),
            Some(ref opts) => {
                match validate_and_trim_notificationprofile_bsm_component_options(opts) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }
        }
    }

    fn bsm_component_selection_is_valid(&self) -> Result<(), OpsviewConfigError> {
        if self.all_business_components.is_some_and(|x| x)
            && self
                .business_components
                .as_ref()
                .is_some_and(|x| !x.is_empty())
        {
            return Err(OpsviewConfigError::ConflictingOptions(
                "all_business_components can't be Some(true) if business_components is also Some and not empty".to_string(),
            ));
        }
        Ok(())
    }

    fn bsm_service_availablity_below_is_valid(&self) -> Result<(), OpsviewConfigError> {
        match self.business_service_availability_below {
            None => Ok(()),
            Some(p) => percentage_between_0_and_100(p),
        }
    }

    fn bsm_service_options_are_valid(&self) -> Result<(), OpsviewConfigError> {
        match self.business_service_options {
            None => Ok(()),
            Some(ref opts) => match validate_and_trim_notificationprofile_bsm_service_options(opts)
            {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
        }
    }

    fn bsm_service_selection_is_valid(&self) -> Result<(), OpsviewConfigError> {
        if self.all_business_services.is_some_and(|x| x)
            && self
                .business_services
                .as_ref()
                .is_some_and(|x| !x.is_empty())
        {
            return Err(OpsviewConfigError::ConflictingOptions(
                "all_business_services can't be Some(true) if business_services is also Some and not empty".to_string(),
            ));
        }
        Ok(())
    }
}

/// A reference version of [`NotificationProfile`] that is used when passing or retrieving a
/// [`NotificationProfile`] object as part of another object.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct NotificationProfileRef {
    name: String,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
}

/// Enables the creation of a [`NotificationProfileRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for NotificationProfileRef {}

impl ConfigRef for NotificationProfileRef {
    type FullObject = NotificationProfile;

    /// Returns the reference string of the [`NotificationProfileRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`NotificationProfileRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`NotificationProfileRef`] object.
    ///
    /// This name is used to identify the `NotificationProfileRef` when building the `HashMap` for a
    /// [`ConfigRefMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl From<NotificationProfile> for NotificationProfileRef {
    /// Creates a [`NotificationProfileRef`] object from a [`NotificationProfile`] object.
    ///
    /// # Arguments
    /// * `notification_profile` - A [`NotificationProfile`] object.
    ///
    /// # Returns
    /// A [`NotificationProfileRef`] object.
    fn from(notification_profile: NotificationProfile) -> Self {
        NotificationProfileRef {
            name: notification_profile.name.clone(),
            ref_: notification_profile.ref_.clone(),
        }
    }
}

impl From<Arc<NotificationProfile>> for NotificationProfileRef {
    fn from(item: Arc<NotificationProfile>) -> Self {
        let cmd: NotificationProfile = Arc::try_unwrap(item).unwrap_or_else(|arc| (*arc).clone());
        NotificationProfileRef::from(cmd)
    }
}

impl From<&ConfigObjectMap<NotificationProfile>> for ConfigRefMap<NotificationProfileRef> {
    fn from(profiles: &ConfigObjectMap<NotificationProfile>) -> Self {
        ref_map_from(profiles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let profile = NotificationProfile::default();

        assert!(profile.name.is_empty());
    }

    #[test]
    fn test_minimal() {
        let profile = NotificationProfile::minimal("My Shared Notification Profile");

        assert_eq!(
            profile.unwrap().name,
            "My Shared Notification Profile".to_string()
        );
    }
}
