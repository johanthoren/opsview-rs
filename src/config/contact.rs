use crate::{config::*, prelude::*, util::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Represents a [Contact](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/users-and-roles/users/index.html) entity in Opsview.
///
/// The `Contact` struct defines a contact within the Opsview monitoring system.
/// Contacts are used to specify individuals responsible for specific hosts or services and for
/// logging in to the system.
///
/// # Example
/// ```rust
/// use opsview::config::{Contact, Role};
/// use opsview::prelude::*;
///
/// let role = Role::minimal("My New Role")
///     .expect("Failed to create a minimal Role with the name 'My New Role'");
///   
/// let contact = Contact::builder()
///   .name("my_contact")
///   .fullname("My Contact")
///   .role(role)
///   .build()
///   .unwrap();
///
///   assert_eq!(contact.fullname.unwrap(), "My Contact".to_string());
/// ```
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Contact {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the Contact, serving as a unique identifier.
    pub name: String,

    // Semi-optional fields ----------------------------------------------------------------------//
    // Required when building a new object, but not always present from the API, so optional for
    // serializing purposes.
    // TODO: Add validation of this field.
    /// The full name of the Contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fullname: Option<String>,

    /// The role assigned to the Contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<RoleRef>,

    // Optional-fields ---------------------------------------------------------------------------//
    // TODO: Add validation of this field.
    /// Description of the Contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A boolean indicating whether tips are enabled for this Contact.
    /// Default: Some(true)
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub enable_tips: Option<bool>,

    /// A unique identifier of the Contact's homepage.
    /// Default: Some(10)
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub homepage_id: Option<u64>,

    // TODO: Add validation of this field.
    /// Language preference for the Contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// A [`ConfigObjectMap`] of [`ContactLink`] objects associated with the Contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mylinks: Option<ConfigObjectMap<ContactLink>>,

    /// [`ConfigObjectMap`] of [`NotificationProfile`] objects associated with the Contact.
    // Not using ConfigRefMap is intentional since this is actually setting the values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notificationprofiles: Option<ConfigObjectMap<NotificationProfile>>,

    // TODO: Add validation of this field.
    /// The realm to which the Contact belongs.
    /// Default: Some("local".to_string())
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,

    /// [`ConfigRefMap`] of [`SharedNotificationProfileRef`] objects associated with the Contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharednotificationprofiles: Option<ConfigRefMap<SharedNotificationProfileRef>>,

    /// [`ConfigRefMap`] of [`VariableValueRef`] associated with the Contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<ConfigRefMap<VariableValueRef>>,

    // Read-only fields --------------------------------------------------------------------------//
    /// The unique identifier of the Contact.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,

    /// Reference string unique to this Contact.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    /// A boolean indicating whether the Contact is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

impl Default for Contact {
    /// Returns a default [`Contact`] object.
    fn default() -> Self {
        Contact {
            name: "".to_string(),
            fullname: None,
            role: None,
            description: None,
            enable_tips: Some(true),
            homepage_id: Some(10),
            language: None,
            mylinks: None,
            notificationprofiles: None,
            realm: Some("local".to_string()),
            sharednotificationprofiles: None,
            variables: None,
            id: None,
            ref_: None,
            uncommitted: None,
        }
    }
}

/// Enables the creation of a [`Contact`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for Contact {}

impl ConfigObject for Contact {
    type Builder = ContactBuilder;

    /// Returns a builder for constructing a [`Contact`] object.
    fn builder() -> Self::Builder {
        ContactBuilder::new()
    }

    /// Provides the configuration path for a [`Contact`] object within the Opsview system.
    ///
    /// # Returns
    /// A string representing the API path where contacts are configured.
    fn config_path() -> Option<String> {
        Some("/config/contact".to_string())
    }

    /// Returns a minimal `Contact` object with only the name set.
    ///
    /// # Arguments
    /// * `name` - Name of the [`Contact`].
    ///
    /// # Returns
    /// A minimal `Contact` object with only the name set, and the rest of the fields in their
    /// default states.
    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            name: validate_and_trim_contact_name(name)?,
            ..Default::default()
        })
    }

    /// Returns the unique name of the [`Contact`] object.
    ///
    /// This name is used to identify the `Contact` when building the `HashMap` for a
    /// [`ConfigObjectMap`].
    ///
    /// # Returns
    /// A string representing the unique name of the Contact.
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl Persistent for Contact {
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

    fn validated_name(&self, name: &str) -> Result<String, OpsviewConfigError> {
        validate_and_trim_contact_name(name)
    }

    fn name_regex(&self) -> Option<String> {
        Some(CONTACT_NAME_REGEX_STR.to_string())
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

impl PersistentMap for ConfigObjectMap<Contact> {
    fn config_path() -> Option<String> {
        Some("/config/contact".to_string())
    }
}

/// Builder for creating instances of [`Contact`].
///
/// Provides a fluent interface for constructing a `Contact` object with optional fields.
#[derive(Clone, Debug)]
pub struct ContactBuilder {
    name: Option<String>,
    fullname: Option<String>,
    role: Option<RoleRef>,
    description: Option<String>,
    enable_tips: Option<bool>,
    homepage_id: Option<u64>,
    language: Option<String>,
    mylinks: Option<ConfigObjectMap<ContactLink>>,
    notificationprofiles: Option<ConfigObjectMap<NotificationProfile>>,
    realm: Option<String>,
    sharednotificationprofiles: Option<ConfigRefMap<SharedNotificationProfileRef>>,
    variables: Option<ConfigRefMap<VariableValueRef>>,
}

impl Default for ContactBuilder {
    /// Creates a new `ContactBuilder` with the default values.
    fn default() -> Self {
        ContactBuilder {
            name: None,
            fullname: None,
            role: None,
            description: None,
            enable_tips: Some(true),
            homepage_id: Some(10),
            language: None,
            mylinks: None,
            notificationprofiles: None,
            realm: Some("local".to_string()),
            sharednotificationprofiles: None,
            variables: None,
        }
    }
}

impl Builder for ContactBuilder {
    type ConfigObject = Contact;

    /// Creates a new instance of [`ContactBuilder`] with default values.
    ///
    /// Initializes a new builder for creating a [`Contact`] object with all fields in their default
    /// state.
    ///
    /// # Returns
    /// A `ContactBuilder` object.
    fn new() -> Self {
        ContactBuilder::default()
    }

    /// Sets the name field.
    ///
    /// # Arguments
    /// * `name` - Name of the Contact.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Builds the [`Contact`] object with the specified properties.
    ///
    /// Constructs a new `Contact` object based on the current state of the builder.
    /// Returns an error if the required field `name` is not set.
    ///
    /// # Returns
    /// A `Result` containing the constructed `Contact` object or an error if the object
    /// could not be built due to the absence of the required fields.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;
        let fullname = require_field(&self.fullname, "fullname")?;
        let role = require_field(&self.role, "role")?;

        let validated_description =
            validate_opt_string(self.description, validate_and_trim_description)?;

        Ok(Contact {
            name: validate_and_trim_contact_name(&name)?,
            fullname: Some(fullname),
            role: Some(role),
            description: validated_description,
            enable_tips: self.enable_tips,
            homepage_id: self.homepage_id,
            language: self.language,
            mylinks: self.mylinks,
            notificationprofiles: self.notificationprofiles,
            realm: self.realm,
            sharednotificationprofiles: self.sharednotificationprofiles,
            variables: self.variables,
            id: None,
            ref_: None,
            uncommitted: None,
        })
    }
}

impl ContactBuilder {
    /// Clears the description field.
    pub fn clear_description(mut self) -> Self {
        self.description = None;
        self
    }

    /// Clears the enable_tips field.
    pub fn clear_enable_tips(mut self) -> Self {
        self.enable_tips = None;
        self
    }

    /// Clears the fullname field.
    pub fn clear_fullname(mut self) -> Self {
        self.fullname = None;
        self
    }

    /// Clears the homepage_id field.
    pub fn clear_homepage_id(mut self) -> Self {
        self.homepage_id = None;
        self
    }

    /// Clears the language field.
    pub fn clear_language(mut self) -> Self {
        self.language = None;
        self
    }

    /// Clears the mylinks field.
    pub fn clear_mylinks(mut self) -> Self {
        self.mylinks = None;
        self
    }

    /// Clears the name field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Clears the notificationprofiles field.
    pub fn clear_notificationprofiles(mut self) -> Self {
        self.notificationprofiles = None;
        self
    }

    /// Clears the realm field.
    pub fn clear_realm(mut self) -> Self {
        self.realm = None;
        self
    }

    /// Clears the role field.
    pub fn clear_role(mut self) -> Self {
        self.role = None;
        self
    }

    /// Clears the sharednotificationprofiles field.
    pub fn clear_sharednotificationprofiles(mut self) -> Self {
        self.sharednotificationprofiles = None;
        self
    }

    /// Clears the variables field.
    pub fn clear_variables(mut self) -> Self {
        self.variables = None;
        self
    }

    /// Sets the description field.
    ///
    /// # Arguments
    /// * `description` - Description of the Contact.
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Sets the enable_tips field.
    ///
    /// # Arguments
    /// * `enable_tips` - Boolean indicating whether tips are enabled for this Contact.
    pub fn enable_tips(mut self, enable_tips: bool) -> Self {
        self.enable_tips = Some(enable_tips);
        self
    }

    /// Sets the fullname field.
    ///
    /// # Arguments
    /// * `fullname` - Full name of the Contact.
    pub fn fullname(mut self, fullname: &str) -> Self {
        self.fullname = Some(fullname.to_string());
        self
    }

    /// Sets the homepage_id field.
    ///
    /// # Arguments
    /// * `homepage_id` - Unique identifier of the Contact's homepage.
    pub fn homepage_id(mut self, homepage_id: u64) -> Self {
        self.homepage_id = Some(homepage_id);
        self
    }

    /// Sets the language field.
    ///
    /// # Arguments
    /// * `language` - Language preference for the Contact.
    pub fn language(mut self, language: &str) -> Self {
        self.language = Some(language.to_string());
        self
    }

    /// Sets the mylinks field.
    ///
    /// # Arguments
    /// * `mylinks` - [`ConfigObjectMap`] of links associated with the Contact.
    pub fn mylinks(mut self, mylinks: ConfigObjectMap<ContactLink>) -> Self {
        self.mylinks = Some(mylinks);
        self
    }

    /// Sets the notificationprofiles field.
    ///
    /// This, unlike most other setter methods, does not take a reference to a `ConfigObjectMap`,
    /// because it is used to set the NotificationProfiles specific to this `Contact` and is not
    /// intended to be re-used. Therefore, it takes ownership and consumes the `ConfigObjectMap`.
    ///
    /// # Arguments
    /// * `notificationprofiles` - A [`ConfigObjectMap`] of [`NotificationProfile`] objects associated with the `Contact`.
    pub fn notificationprofiles(
        mut self,
        notificationprofiles: ConfigObjectMap<NotificationProfile>,
    ) -> Self {
        self.notificationprofiles = Some(notificationprofiles);
        self
    }

    /// Sets the realm field.
    ///
    /// # Arguments
    /// * `realm` - Realm to which the Contact belongs.
    pub fn realm(mut self, realm: &str) -> Self {
        self.realm = Some(realm.to_string());
        self
    }

    /// Sets the role field.
    ///
    /// # Arguments
    /// * `role` - The [`Role`] assigned to the Contact.
    pub fn role(mut self, role: Role) -> Self {
        self.role = Some(RoleRef::from(role));
        self
    }

    /// Sets the sharednotificationprofiles field.
    ///
    /// # Arguments
    /// * `sharednotificationprofiles` - A reference to a [`ConfigObjectMap`] of [`SharedNotificationProfile`]s associated with the Contact.
    pub fn sharednotificationprofiles(
        mut self,
        sharednotificationprofiles: &ConfigObjectMap<SharedNotificationProfile>,
    ) -> Self {
        self.sharednotificationprofiles = Some(sharednotificationprofiles.into());
        self
    }

    /// Sets the variables field.
    ///
    /// # Arguments
    /// * `variables` - A reference to a [`ConfigObjectMap`] of [`Variable`]s associated with the Contact.
    pub fn variables(mut self, variables: &ConfigObjectMap<Variable>) -> Self {
        self.variables = Some(variables.into());
        self
    }
}

/// A reference version of [`Contact`] that is used when passing or retrieving a
/// [`Contact`] object as part of another object.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct ContactRef {
    name: String,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
}

/// Enables the creation of a [`ContactRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for ContactRef {}

impl ConfigRef for ContactRef {
    type FullObject = Contact;

    /// Returns the reference string of the [`ContactRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`ContactRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`ContactRef`] object.
    ///
    /// This name is used to identify the `ContactRef` when building the `HashMap` for a
    /// [`ConfigRefMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl From<Contact> for ContactRef {
    /// Creates a [`ContactRef`] object from a [`Contact`] object.
    ///
    /// # Arguments
    /// * `contact` - A reference to a [`Contact`] object.
    ///
    /// # Returns
    /// A [`ContactRef`] object.
    fn from(contact: Contact) -> Self {
        Self {
            name: contact.name.clone(),
            ref_: contact.ref_.clone(),
        }
    }
}

impl From<Arc<Contact>> for ContactRef {
    fn from(item: Arc<Contact>) -> Self {
        let contact: Contact = Arc::try_unwrap(item).unwrap_or_else(|arc| (*arc).clone());
        ContactRef::from(contact)
    }
}

impl From<&ConfigObjectMap<Contact>> for ConfigRefMap<ContactRef> {
    fn from(contacts: &ConfigObjectMap<Contact>) -> Self {
        ref_map_from(contacts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let contact = Contact::default();

        assert!(contact.name.is_empty());
    }

    #[test]
    fn test_minimal() {
        let contact = Contact::minimal("my_contact");

        assert_eq!(contact.unwrap().name, "my_contact".to_string());
    }

    #[test]
    fn test_build_fails_on_invalid_name() {
        let contact = Contact::builder().name("foo bar").build();
        assert!(contact.is_err());

        let contact2 = Contact::builder().name(" foobar").build();
        assert!(contact2.is_err());
    }

    #[test]
    fn test_minimal_fails_on_invalid_name() {
        let names = ["foo bar", "foobar//123", "😢"];

        for name in names.iter() {
            let contact = Contact::minimal(name);
            assert!(contact.is_err());
        }
    }

    #[test]
    fn test_minimal_succeeds_on_valid_name() {
        let names = [
            "foobar",
            " foo",
            "foo ",
            "foobar123",
            "foobar_123",
            "foobar-123",
            "foobar@123",
            "foobar#123",
            "foobar.123",
        ];

        for name in names.iter() {
            let contact = Contact::minimal(name);
            assert!(contact.is_ok());
        }
    }
}
