use super::{NotificationProfileRef, SharedNotificationProfileRef, Variable, VariableValueRef};
use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Represents a [Notification Method](https://docs.itrsgroup.com/docs/opsview/6.8.9/monitoring/notifications/notification-methods/index.html) in Opsview.
///
/// Notification Methods are used to define how notifications are sent to [`super::Contact`]s in Opsview.
/// This struct defines the structure for a notification method entity as used in Opsview.
///
/// # Example
/// ```rust
/// use opsview::config::NotificationMethod;
/// use opsview::prelude::*;
///
/// let notification_method = NotificationMethod::builder()
///    .name("RSS")
///    .command("notify_by_rss")
///    .contact_variables("RSS_MAXIMUM_ITEMS,RSS_MAXIMUM_AGE,RSS_COLLAPSED")
///    .namespace("com.opsview.notificationmethods.rss")
///    .active(true)
///    .master(true)
///    .build()
///    .unwrap();
///
/// assert_eq!(notification_method.name, "RSS".to_string());
/// assert_eq!(notification_method.command, Some("notify_by_rss".to_string()));
/// assert_eq!(notification_method.active, Some(true));
/// ```
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct NotificationMethod {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the `NotificationMethod`.
    pub name: String,

    // Optional fields ---------------------------------------------------------------------------//
    /// A boolean indicating whether the `NotificationMethod` is active.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub active: Option<bool>,

    /// The command to execute for the `NotificationMethod`.
    ///
    /// Must be found at `/opt/opsview/monitoringscripts/notifications/` on the Opsview server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    // TODO: Add validation of this field.
    /// Contact variables for the `NotificationMethod`.
    ///
    /// These are the user specific variable names that are required for the `NotificationMethod` to
    /// function. Must be a comma-separated list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_variables: Option<String>,

    /// TODO: Undocumented field.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub master: Option<bool>,

    // TODO: Add validation of this field.
    /// The namespace for the `NotificationMethod`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// [`ConfigRefMap`] of [`VariableValueRef`] objects associated with this `NotificationMethod`.
    ///
    /// These would typically hold any non-user specific variables that are required for the
    /// `NotificationMethod` to function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<ConfigRefMap<VariableValueRef>>,

    // Read-only fields --------------------------------------------------------------------------//
    /// The unique identifier of the `NotificationMethod`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,

    /// [`ConfigRefMap`] of [`NotificationProfileRef`] objects associated with this `NotificationMethod`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notificationprofiles: Option<ConfigRefMap<NotificationProfileRef>>,

    /// A reference string unique to this `NotificationMethod`.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    /// [`ConfigRefMap`] of `SharedNotificationProfileRef` objects associated with this `NotificationMethod`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharednotificationprofiles: Option<ConfigRefMap<SharedNotificationProfileRef>>,

    /// A boolean indicating whether the `NotificationMethod` is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

/// Enables the creation of a [`NotificationMethod`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for NotificationMethod {}

impl ConfigObject for NotificationMethod {
    type Builder = NotificationMethodBuilder;

    /// Returns a builder for constructing a [`NotificationMethod`] object.
    ///
    /// # Returns
    /// A [`NotificationMethodBuilder`] object.
    fn builder() -> Self::Builder {
        NotificationMethodBuilder::new()
    }

    /// Provides the configuration path for a [`NotificationMethod`] object within the Opsview system.
    ///
    /// # Returns
    /// A string representing the API path where notification methods are configured.
    fn config_path() -> Option<String> {
        Some("/config/notificationmethod".to_string())
    }

    /// Returns the unique name of the [`NotificationMethod`] object.
    ///
    /// This name is used to identify the `NotificationMethod` when building the `HashMap` for an
    /// [`ConfigObjectMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl Persistent for NotificationMethod {
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
        Some(NOTIFICATIONMETHOD_NAME_REGEX_STR.to_string())
    }

    fn validated_name(&self, name: &str) -> Result<String, OpsviewConfigError> {
        validate_and_trim_notificationmethod_name(name)
    }

    fn set_name(&mut self, new_name: &str) -> Result<String, OpsviewConfigError> {
        self.name = self.validated_name(new_name)?;
        Ok(self.name.clone())
    }

    fn clear_readonly(&mut self) {
        self.id = None;
        self.notificationprofiles = None;
        self.ref_ = None;
        self.sharednotificationprofiles = None;
        self.uncommitted = None;
    }
}

impl PersistentMap for ConfigObjectMap<NotificationMethod> {
    fn config_path() -> Option<String> {
        Some("/config/notificationmethod".to_string())
    }
}
/// Builder for creating instances of [`NotificationMethod`].
///
/// Provides a fluent interface for constructing a `NotificationMethod` object with optional fields.
#[derive(Clone, Debug, Default)]
pub struct NotificationMethodBuilder {
    // Required fields ---------------------------------------------------------------------------//
    name: Option<String>,
    // Optional fields ---------------------------------------------------------------------------//
    active: Option<bool>,
    command: Option<String>,
    contact_variables: Option<String>,
    master: Option<bool>,
    namespace: Option<String>,
    variables: Option<ConfigRefMap<VariableValueRef>>,
}

impl Builder for NotificationMethodBuilder {
    type ConfigObject = NotificationMethod;

    /// Creates a new instance of [`NotificationMethodBuilder`] with default values.
    ///
    /// Initializes a new builder for creating a [`NotificationMethod`] object with all fields in their
    /// default state.
    ///
    /// # Returns
    /// A `NotificationMethodBuilder` object.
    fn new() -> Self {
        NotificationMethodBuilder::default()
    }

    /// Sets the name field.
    ///
    /// # Arguments
    /// * `name` - The name of the `NotificationMethod`.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Consumes the builder and returns a [`NotificationMethod`] object.
    ///
    /// # Returns
    /// A `NotificationMethod` object constructed from the builder's configuration.
    ///
    /// # Errors
    /// * `name not set` - The required `name` field was not set.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;
        let validated_command =
            validate_opt_string(self.command, validate_and_trim_notification_command)?;

        Ok(NotificationMethod {
            name: validate_and_trim_notificationmethod_name(&name)?,
            active: self.active,
            command: validated_command,
            contact_variables: self.contact_variables,
            master: self.master,
            namespace: self.namespace,
            variables: self.variables,
            id: None,
            notificationprofiles: None,
            ref_: None,
            sharednotificationprofiles: None,
            uncommitted: None,
        })
    }
}

impl NotificationMethodBuilder {
    /// Sets the active field.
    ///
    /// # Arguments
    /// * `active` - A boolean indicating whether the `NotificationMethod` is active.
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    /// Clears the active field.
    pub fn clear_active(mut self) -> Self {
        self.active = None;
        self
    }

    /// Clears the command field.
    pub fn clear_command(mut self) -> Self {
        self.command = None;
        self
    }

    /// Clears the contact_variables field.
    pub fn clear_contact_variables(mut self) -> Self {
        self.contact_variables = None;
        self
    }

    /// Clears the master field.
    pub fn clear_master(mut self) -> Self {
        self.master = None;
        self
    }

    /// Clears the name field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Clears the namespace field.
    pub fn clear_namespace(mut self) -> Self {
        self.namespace = None;
        self
    }

    /// Clears the variables field.
    pub fn clear_variables(mut self) -> Self {
        self.variables = None;
        self
    }

    /// Sets the command field.
    ///
    /// # Arguments
    /// * `command` - The command to execute for the `NotificationMethod`.
    pub fn command(mut self, command: &str) -> Self {
        self.command = Some(command.to_string());
        self
    }

    /// Sets the contact_variables field.
    ///
    /// # Arguments
    /// * `contact_variables` - The contact variables for the `NotificationMethod`.
    pub fn contact_variables(mut self, contact_variables: &str) -> Self {
        self.contact_variables = Some(contact_variables.to_string());
        self
    }

    /// Sets the master field.
    ///
    /// # Arguments
    /// * `master` - A boolean indicating whether the `NotificationMethod` is a master.
    pub fn master(mut self, master: bool) -> Self {
        self.master = Some(master);
        self
    }

    /// Sets the namespace field.
    ///
    /// # Arguments
    /// * `namespace` - The namespace for the `NotificationMethod`.
    pub fn namespace(mut self, namespace: &str) -> Self {
        self.namespace = Some(namespace.to_string());
        self
    }

    /// Sets the variables field.
    ///
    /// # Arguments
    /// * `variables` - A reference to a [`ConfigObjectMap`] of [`Variable`] objects associated with this `NotificationMethod`.
    pub fn variables(mut self, variables: &ConfigObjectMap<Variable>) -> Self {
        self.variables = Some(variables.into());
        self
    }
}

/// A reference version of [`NotificationMethod`] that is used when passing or retrieving a
/// [`NotificationMethod`] object as part of another object.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct NotificationMethodRef {
    name: String,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
}

/// Enables the creation of a [`NotificationMethodRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for NotificationMethodRef {}

impl ConfigRef for NotificationMethodRef {
    type FullObject = NotificationMethod;

    /// Returns the reference string of the [`NotificationMethodRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`NotificationMethodRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`NotificationMethodRef`] object.
    ///
    /// This name is used to identify the `NotificationMethodRef` when building the `HashMap` for a
    /// [`ConfigRefMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl From<NotificationMethod> for NotificationMethodRef {
    /// Creates a [`NotificationMethodRef`] object from a [`NotificationMethod`] object reference.
    ///
    /// # Arguments
    /// * `notification_method` - A [`NotificationMethod`] object.
    ///
    /// # Returns
    /// A [`NotificationMethodRef`] object.
    fn from(notification_method: NotificationMethod) -> Self {
        Self {
            name: notification_method.name.clone(),
            ref_: notification_method.ref_.clone(),
        }
    }
}

impl From<Arc<NotificationMethod>> for NotificationMethodRef {
    fn from(item: Arc<NotificationMethod>) -> Self {
        let cmd: NotificationMethod = Arc::try_unwrap(item).unwrap_or_else(|arc| (*arc).clone());
        NotificationMethodRef::from(cmd)
    }
}

impl From<&ConfigObjectMap<NotificationMethod>> for ConfigRefMap<NotificationMethodRef> {
    fn from(methods: &ConfigObjectMap<NotificationMethod>) -> Self {
        ref_map_from(methods)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let method = NotificationMethod::default();

        assert!(method.name.is_empty());
    }

    #[test]
    fn test_minimal() {
        let method = NotificationMethod::minimal("My NotificationMethod");

        assert_eq!(method.unwrap().name, "My NotificationMethod".to_string());
    }
}
