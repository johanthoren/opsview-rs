use super::ServiceCheck;
use crate::config::{Host, HostRef, Role, RoleRef, ServiceCheckRef};
use crate::{prelude::*, util::*};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::str::FromStr;
use std::sync::Arc;

/// Represents a [Hashtag](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/hashtags/hashtags/index.html#Heading-overview) entity in Opsview.
///
/// The `Hashtag` struct defines the structure for a hashtag used within the Opsview monitoring
/// system. Hashtags in Opsview are used to categorize and filter monitoring data, such as hosts and
/// service checks. It also allows administrators to restrict access for some users or roles to one
/// or more Hashtags.
///
/// For legacy reasons, Hashtags are known as keywords in the Opsview API, but in this library they
/// are known as Hashtags since that's what they are called in the modern Opsview UI.
///
/// # Example
/// ```rust
/// use opsview::config::Hashtag;
/// use opsview::prelude::*;
///
/// let hashtag = Hashtag::builder()
///   .name("MyHashtag")
///   .description("My Hashtag Description")
///   .build()
///   .unwrap();
///
///   assert_eq!(hashtag.name, "MyHashtag".to_string());
///   assert_eq!(hashtag.description.unwrap(), "My Hashtag Description".to_string());
/// ```
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Hashtag {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the hashtag, serving as a unique identifier.
    pub name: String,

    // Optional fields ---------------------------------------------------------------------------//
    /// A boolean indicating whether the `Hashtag` applies to all [`Host`]s.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub all_hosts: Option<bool>,

    /// A boolean indicating whether the `Hashtag` applies to all [`ServiceCheck`]s.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub all_servicechecks: Option<bool>,

    /// A boolean indicating whether to calculate hard states for the `Hashtag`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub calculate_hard_states: Option<bool>,

    /// A description of the `Hashtag`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A boolean indicating whether the `Hashtag` is enabled in the [Hashtag Summary](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/hashtags/hashtag-views/index.html) view.
    /// Default: Some(true)
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub enabled: Option<bool>,

    /// A boolean indicating whether to exclude handled items from the `Hashtag`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub exclude_handled: Option<bool>,

    /// [`ConfigRefMap`] of [`HostRef`] objects associated with the `Hashtag`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: Option<ConfigRefMap<HostRef>>,

    /// A boolean indicating whether the `Hashtag` is public.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub public: Option<bool>,

    /// [`ConfigRefMap`] of [`RoleRef`] objects associated with the `Hashtag`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<ConfigRefMap<RoleRef>>,

    /// [`ConfigRefMap`] of [`ServiceCheckRef`] objects associated with the `Hashtag`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servicechecks: Option<ConfigRefMap<ServiceCheckRef>>,

    /// A boolean indicating whether to show contextual menus for the `Hashtag`.
    /// Default: Some(true)
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub show_contextual_menus: Option<bool>,

    /// A `HashtagStyle` variant representing the visual style of the `Hashtag`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_hashtag_style",
        default
    )]
    pub style: Option<HashtagStyle>,

    // Read-only fields --------------------------------------------------------------------------//
    /// The unique identifier of the `Hashtag`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,

    /// A reference string unique to this `Hashtag`.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    /// A boolean indicating whether the `Hashtag` is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

impl Default for Hashtag {
    /// Returns a default [`Hashtag`] object.
    fn default() -> Self {
        Hashtag {
            name: "".to_string(),
            all_hosts: None,
            all_servicechecks: None,
            calculate_hard_states: None,
            description: None,
            enabled: Some(true),
            exclude_handled: None,
            hosts: None,
            public: None,
            roles: None,
            servicechecks: None,
            show_contextual_menus: Some(true),
            style: None,
            id: None,
            ref_: None,
            uncommitted: None,
        }
    }
}

/// Enables the creation of a [`Hashtag`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for Hashtag {}

impl ConfigObject for Hashtag {
    type Builder = HashtagBuilder;

    /// Returns a builder for constructing a [`Hashtag`] object.
    ///
    /// # Returns
    /// A [`HashtagBuilder`] object.
    fn builder() -> Self::Builder {
        HashtagBuilder::new()
    }

    /// Provides the configuration path for a [`Hashtag`] object within the Opsview system.
    ///
    /// # Returns
    /// A string representing the API path where hashtags are configured.
    fn config_path() -> Option<String> {
        Some("/config/keyword".to_string())
    }

    /// Returns a minimal `Hashtag` object with only the name set.
    ///
    /// # Arguments
    /// * `name` - Name of the [`Hashtag`].
    ///
    /// # Returns
    /// A Result containing a minimal `Hashtag` object with only the name set, and the rest of the fields in their
    /// default states.
    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            name: validate_and_trim_hashtag_name(name)?,
            ..Default::default()
        })
    }

    /// Returns the unique name of the [`Hashtag`] object.
    ///
    /// This name is used to identify the `Hashtag` when building the `HashMap` for an
    /// [`ConfigObjectMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl Persistent for Hashtag {
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
        Some(HASHTAG_NAME_REGEX_STR.to_string())
    }

    fn validated_name(&self, name: &str) -> Result<String, OpsviewConfigError> {
        validate_and_trim_hashtag_name(name)
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

impl PersistentMap for ConfigObjectMap<Hashtag> {
    fn config_path() -> Option<String> {
        Some("/config/keyword".to_string())
    }
}

/// Builder for creating instances of [`Hashtag`].
///
/// Provides a fluent interface for constructing a `Hashtag` object with optional fields.
#[derive(Clone, Debug)]
pub struct HashtagBuilder {
    name: Option<String>,
    all_hosts: Option<bool>,
    all_servicechecks: Option<bool>,
    calculate_hard_states: Option<bool>,
    description: Option<String>,
    enabled: Option<bool>,
    exclude_handled: Option<bool>,
    hosts: Option<ConfigRefMap<HostRef>>,
    public: Option<bool>,
    roles: Option<ConfigRefMap<RoleRef>>,
    servicechecks: Option<ConfigRefMap<ServiceCheckRef>>,
    show_contextual_menus: Option<bool>,
    style: Option<HashtagStyle>,
}

impl Default for HashtagBuilder {
    /// Creates a new instance of [`HashtagBuilder`] with default values.
    ///
    /// Initializes a new builder for creating a [`Hashtag`] object with all fields in their default
    /// state.
    fn default() -> Self {
        HashtagBuilder {
            name: None,
            all_hosts: None,
            all_servicechecks: None,
            calculate_hard_states: None,
            description: None,
            enabled: Some(true),
            exclude_handled: None,
            hosts: None,
            public: None,
            roles: None,
            servicechecks: None,
            show_contextual_menus: Some(true),
            style: None,
        }
    }
}

impl Builder for HashtagBuilder {
    type ConfigObject = Hashtag;

    /// Creates a new instance of [`HashtagBuilder`] with default values.
    ///
    /// Initializes a new builder for creating a [`Hashtag`] object with all fields in their default
    /// state.
    fn new() -> Self {
        HashtagBuilder::default()
    }

    /// Sets the `name` field.
    ///
    /// # Arguments
    /// * `name` - Name of the `Hashtag`.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Builds the [`Hashtag`] object with the specified properties.
    ///
    /// Constructs a new `Hashtag` object based on the current state of the builder.
    /// Returns an error if the required field `name` is not set.
    ///
    /// # Returns
    /// A `Result` containing the constructed `Hashtag` object or an error if the object
    /// could not be built due to the absence of the required `name` field.
    ///
    /// # Errors
    /// * `OpsviewConfigError::RequiredFieldEmpty` - If the `name` field is not set.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;

        let validated_description =
            validate_opt_string(self.description, validate_and_trim_description)?;

        Ok(Hashtag {
            name: validate_and_trim_hashtag_name(&name)?,
            all_hosts: self.all_hosts,
            all_servicechecks: self.all_servicechecks,
            calculate_hard_states: self.calculate_hard_states,
            description: validated_description,
            enabled: self.enabled,
            exclude_handled: self.exclude_handled,
            hosts: self.hosts,
            public: self.public,
            roles: self.roles,
            servicechecks: self.servicechecks,
            show_contextual_menus: self.show_contextual_menus,
            style: self.style,
            id: None,
            ref_: None,
            uncommitted: None,
        })
    }
}

impl HashtagBuilder {
    /// Sets the `all_hosts` field.
    ///
    /// # Arguments
    /// * `all_hosts` - Boolean indicating if the `Hashtag` applies to all [`Host`]s.
    pub fn all_hosts(mut self, all_hosts: bool) -> Self {
        self.all_hosts = Some(all_hosts);
        self
    }

    /// Sets the `all_servicechecks` field.
    ///
    /// # Arguments
    /// * `all_servicechecks` - Boolean indicating if the `Hashtag` applies to all [`ServiceCheck`]s.
    pub fn all_servicechecks(mut self, all_servicechecks: bool) -> Self {
        self.all_servicechecks = Some(all_servicechecks);
        self
    }

    /// Sets the `calculate_hard_states` field.
    ///
    /// # Arguments
    /// * `calculate_hard_states` - Boolean indicating if hard states are to be calculated.
    pub fn calculate_hard_states(mut self, calculate_hard_states: bool) -> Self {
        self.calculate_hard_states = Some(calculate_hard_states);
        self
    }

    /// Clears the `all_hosts` field.
    pub fn clear_all_hosts(mut self) -> Self {
        self.all_hosts = None;
        self
    }

    /// Clears the `all_servicechecks` field.
    pub fn clear_all_servicechecks(mut self) -> Self {
        self.all_servicechecks = None;
        self
    }

    /// Clears the `calculate_hard_states` field.
    pub fn clear_calculate_hard_states(mut self) -> Self {
        self.calculate_hard_states = None;
        self
    }

    /// Clears the `description` field.
    pub fn clear_description(mut self) -> Self {
        self.description = None;
        self
    }

    /// Clears the `enabled` field.
    pub fn clear_enabled(mut self) -> Self {
        self.enabled = None;
        self
    }

    /// Clears the `exclude_handled` field.
    pub fn clear_exclude_handled(mut self) -> Self {
        self.exclude_handled = None;
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

    /// Clears the `public` field.
    pub fn clear_public(mut self) -> Self {
        self.public = None;
        self
    }

    /// Clears the `roles` field.
    pub fn clear_roles(mut self) -> Self {
        self.roles = None;
        self
    }

    /// Clears the `servicechecks` field.
    pub fn clear_servicechecks(mut self) -> Self {
        self.servicechecks = None;
        self
    }

    /// Clears the `show_contextual_menus` field.
    pub fn clear_show_contextual_menus(mut self) -> Self {
        self.show_contextual_menus = None;
        self
    }

    /// Clears the `style` field.
    pub fn clear_style(mut self) -> Self {
        self.style = None;
        self
    }

    /// Sets the `description` field.
    ///
    /// # Arguments
    /// * `description` - Description of the `Hashtag`.
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Sets the `enabled` field.
    ///
    /// # Arguments
    /// * `enabled` - Boolean indicating if the `Hashtag` is enabled in [the summary view](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/hashtags/hashtag-views/index.html#list-view).
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    /// Sets the `exclude_handled` field.
    ///
    /// # Arguments
    /// * `exclude_handled` - Boolean indicating if handled items should be excluded.
    pub fn exclude_handled(mut self, exclude_handled: bool) -> Self {
        self.exclude_handled = Some(exclude_handled);
        self
    }

    /// Sets the `hosts` field.
    ///
    /// # Arguments
    /// * `hosts` - A reference to a [`ConfigObjectMap`] of [`Host`] objects associated with the `Hashtag`.
    pub fn hosts(mut self, hosts: &ConfigObjectMap<Host>) -> Self {
        self.hosts = Some(hosts.into());
        self
    }

    /// Sets the `public` field.
    ///
    /// # Arguments
    /// * `public` - Boolean indicating if the `Hashtag` is [public](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/hashtags/hashtag-views/index.html#public-hashtags-view).
    pub fn public(mut self, public: bool) -> Self {
        self.public = Some(public);
        self
    }

    /// Sets the `roles` field.
    ///
    /// # Arguments
    /// * `roles` - A reference to a [`ConfigObjectMap`] of [`Role`] objects associated with the `Hashtag`.
    pub fn roles(mut self, roles: &ConfigObjectMap<Role>) -> Self {
        self.roles = Some(roles.into());
        self
    }

    /// Sets the `servicechecks` field.
    ///
    /// # Arguments
    /// * `servicechecks` - A reference to a [`ConfigObjectMap`] of [`ServiceCheck`] objects associated with the `Hashtag`.
    pub fn servicechecks(mut self, servicechecks: &ConfigObjectMap<ServiceCheck>) -> Self {
        self.servicechecks = Some(servicechecks.into());
        self
    }

    /// Sets the `show_contextual_menus` field.
    ///
    /// # Arguments
    /// * `show_contextual_menus` - Boolean indicating if contextual menus should be shown.
    pub fn show_contextual_menus(mut self, show_contextual_menus: bool) -> Self {
        self.show_contextual_menus = Some(show_contextual_menus);
        self
    }

    /// Sets the `style` field.
    ///
    /// # Arguments
    /// * `style` - `HashtagStyle` variant representing the visual style of the `Hashtag`.
    pub fn style(mut self, style: HashtagStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// A reference version of [`Hashtag`] that is used when passing or retrieving a
/// [`Hashtag`] object as part of another object.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct HashtagRef {
    name: String,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
}

/// Enables the creation of a [`HashtagRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for HashtagRef {}

impl ConfigRef for HashtagRef {
    type FullObject = Hashtag;

    /// Returns the reference string of the [`HashtagRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`HashtagRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`HashtagRef`] object.
    ///
    /// This name is used to identify the `HashtagRef` when building the `HashMap` for a
    /// [`ConfigRefMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl From<Hashtag> for HashtagRef {
    fn from(full_object: Hashtag) -> Self {
        Self {
            name: full_object.name.clone(),
            ref_: full_object.ref_.clone(),
        }
    }
}

impl From<Arc<Hashtag>> for HashtagRef {
    fn from(item: Arc<Hashtag>) -> Self {
        let hashtag: Hashtag = Arc::try_unwrap(item).unwrap_or_else(|arc| (*arc).clone());
        HashtagRef::from(hashtag)
    }
}

impl From<&ConfigObjectMap<Hashtag>> for ConfigRefMap<HashtagRef> {
    fn from(hashtags: &ConfigObjectMap<Hashtag>) -> Self {
        ref_map_from(hashtags)
    }
}

/// Display style for the hashtag detail view.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum HashtagStyle {
    GroupByHost,
    GroupByService,
    HostSummary,
    ErrorsAndHostCells,
    Performance,
}

impl FromStr for HashtagStyle {
    type Err = OpsviewConfigError;
    fn from_str(s: &str) -> Result<Self, OpsviewConfigError> {
        match s {
            "group_by_host" => Ok(HashtagStyle::GroupByHost),
            "group_by_service" => Ok(HashtagStyle::GroupByService),
            "host_summary" => Ok(HashtagStyle::HostSummary),
            "errors_and_host_cells" => Ok(HashtagStyle::ErrorsAndHostCells),
            "performance" => Ok(HashtagStyle::Performance),
            _ => Err(OpsviewConfigError::InvalidHashtagStyle(s.to_string())),
        }
    }
}

fn deserialize_hashtag_style<'de, D>(deserializer: D) -> Result<Option<HashtagStyle>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        Value::Null => Ok(None),
        Value::String(ref s) if s == "null" => Ok(None),
        _ => {
            let maybe_style = HashtagStyle::deserialize(value)
                .map(Some)
                .map_err(serde::de::Error::custom);

            maybe_style
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let hashtag = Hashtag::default();

        assert_eq!(hashtag.name, "".to_string());
    }

    #[test]
    fn test_minimal() {
        let hashtag = Hashtag::minimal("MyHashtag");

        assert_eq!(hashtag.unwrap().name, "MyHashtag".to_string());

        let hashtag_2 = Hashtag::minimal("My Hashtag");

        assert!(hashtag_2.is_err());
    }

    #[test]
    fn test_build() {
        let hashtag = Hashtag::builder()
            .name("MyHashtag")
            .description("My Hashtag Description")
            .build()
            .unwrap();

        assert_eq!(hashtag.name, "MyHashtag".to_string());

        assert_eq!(
            hashtag.description.unwrap(),
            "My Hashtag Description".to_string()
        );

        let hashtag_2 = Hashtag::builder()
            .name("My Hashtag")
            .description("My Hashtag Description")
            .build();

        assert!(hashtag_2.is_err());
    }
}
