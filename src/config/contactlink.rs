use super::FontAwesomeIcon;
use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};

/// Represents a contact link in Opsview.
///
/// Contact links are displayed as [My Links](https://docs.itrsgroup.com/docs/opsview/6.8.9/getting-started/user-interface/my-profile/index.html#my-links) in the Opsview UI. They are used to provide
/// quick access to frequently used pages in the Opsview UI that the user may want to save.
///
/// There is no API endpoint for managing contact links, so this struct is only used as a
/// component of other entities.
///
/// This struct defines the structure for a contact link entity as used in Opsview.
///
/// # Example
/// ```rust
/// use opsview::config::{ContactLink, FontAwesomeIcon};
/// use opsview::prelude::*;
///
/// let contact_link = ContactLink::builder()
///     .name("My Contact Link")
///     .fontawesome_icon(FontAwesomeIcon::Sitemap)
///     .url("/monitoring/#!/allproblems?cid=mxUQoVgjyu273hJ8")
///     .build()
///     .unwrap();
///
/// assert_eq!(contact_link.name, "My Contact Link".to_string());
/// ```
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ContactLink {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the contact link. This is used as a unique identifier per user, but is not
    /// unique across all users.
    pub name: String,

    // Optional fields ---------------------------------------------------------------------------//
    // TODO: Add validation of this field.
    /// URL associated with this contact link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// [`FontAwesomeIcon`] associated with this contact link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fontawesome_icon: Option<FontAwesomeIcon>,

    // Read-only fields --------------------------------------------------------------------------//
    /// The unique identifier of the contact link.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,
}

/// Enables the creation of a `ContactLink` instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for ContactLink {}

impl ConfigObject for ContactLink {
    type Builder = ContactLinkBuilder;

    /// Returns a builder for constructing a `ContactLink` object.
    fn builder() -> Self::Builder {
        ContactLinkBuilder::new()
    }

    /// Provides the configuration path for a `ContactLink` object within the Opsview system.
    ///
    /// # Returns
    /// `None` since `ContactLink` does not have a dedicated configuration path in the API.
    fn config_path() -> Option<String> {
        None
    }

    /// Returns the unique name of the `ContactLink` object.
    ///
    /// This name is used to identify the `ContactLink` when building the `HashMap` for a
    /// `ConfigObjectMap`. Since the name is not unique across all users, the last 5 characters
    /// of the URL are appended to the name to ensure uniqueness. If the URL is less than 5
    /// characters, the entire URL is appended to the name, regardless of length.
    ///
    /// # Returns
    /// A string representing the unique name of the `ContactLink` entity.
    fn unique_name(&self) -> String {
        if self.url.as_ref().is_some_and(|u| u.len() < 5) {
            return format!("{}_{}", self.name, &self.url.as_ref().unwrap());
        } else if self.url.as_ref().is_some() {
            return format!(
                "{}_{}",
                self.name,
                &self.url.as_ref().unwrap()[self.url.as_ref().unwrap().len() - 5..]
            );
        }

        self.name.clone()
    }

    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            name: validate_and_trim_contactlink_name(name)?,
            ..Default::default()
        })
    }
}

/// Builder for creating instances of `ContactLink`.
///
/// Provides a fluent interface for constructing an `ContactLink` object with optional fields.
#[derive(Clone, Debug, Default)]
pub struct ContactLinkBuilder {
    name: Option<String>,
    url: Option<String>,
    fontawesome_icon: Option<FontAwesomeIcon>,
}

impl Builder for ContactLinkBuilder {
    type ConfigObject = ContactLink;

    /// Creates a new instance of `ContactLinkBuilder` with default values.
    ///
    /// Initializes a new builder for creating a `ContactLink` object with all fields in their
    /// default state.
    ///
    /// # Returns
    /// A `ContactLinkBuilder` object.
    fn new() -> Self {
        Self::default()
    }

    /// Sets the `name` field.
    ///
    /// # Arguments
    /// * `name` - The name of the contact link.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Builds a new instance of [`ContactLink`] using the configured values.
    ///
    /// # Returns
    /// A new instance of `ContactLink`.
    ///
    /// # Errors
    /// Returns an `OpsviewConfigError` if the `name` field is not set or not valid.
    /// Returns an `OpsviewConfigError` if the `url` field is not set.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;
        let url = require_field(&self.url, "url")?;

        Ok(ContactLink {
            name: validate_and_trim_contactlink_name(&name)?,
            url: Some(url),
            fontawesome_icon: self.fontawesome_icon,
            id: None,
        })
    }
}

impl ContactLinkBuilder {
    /// Clears the `name` field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Clears the `url` field.
    pub fn clear_url(mut self) -> Self {
        self.url = None;
        self
    }

    /// Clears the `fontawesome_icon` field.
    pub fn clear_fontawesome_icon(mut self) -> Self {
        self.fontawesome_icon = None;
        self
    }

    /// Sets the `url` field.
    ///
    /// # Arguments
    /// * `url` - The URL associated with the contact link.
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    /// Sets the `fontawesome_icon` field.
    ///
    /// # Arguments
    /// * `fontawesome_icon` - The [`FontAwesomeIcon`] associated with the contact link.
    pub fn fontawesome_icon(mut self, fontawesome_icon: FontAwesomeIcon) -> Self {
        self.fontawesome_icon = Some(fontawesome_icon);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let contactlink = ContactLink::default();

        assert!(contactlink.name.is_empty());
    }

    #[test]
    fn test_minimal() {
        let contactlink = ContactLink::minimal("My Contact Link")
            .expect("Failed to create a minimal ContactLink");

        assert_eq!(contactlink.name, "My Contact Link".to_string());
    }

    #[test]
    fn test_minimal_fails_with_invalid_name() {
        let contactlink = ContactLink::minimal("My Contact Link 1");
        assert!(contactlink.is_ok());

        let contatlink2 = ContactLink::minimal("My Contact Link 1!");
        assert!(contatlink2.is_err());

        let contatlink3 = ContactLink::minimal("My Contact Link 1#");
        assert!(contatlink3.is_err());

        let contatlink4 = ContactLink::minimal("My Contact Link 1$");
        assert!(contatlink4.is_err());

        let contatlink5 = ContactLink::minimal("My Contact Link 1%");
        assert!(contatlink5.is_err());

        let contatlink6 = ContactLink::minimal("My Contact Link 1&");
        assert!(contatlink6.is_err());
    }

    #[test]
    fn test_build_fails_with_invalid_name() {
        let valid_url = "/monitoring/#!/allproblems?cid=mxUQoVgjyu273hJ8".to_string();

        let contactlink = ContactLink::builder()
            .name("My Contact Link 1")
            .url(&valid_url)
            .build();
        assert!(contactlink.is_ok());

        let contatlink2 = ContactLink::builder()
            .name("My Contact Link 1!")
            .url(&valid_url)
            .build();
        assert!(contatlink2.is_err());

        let contatlink3 = ContactLink::builder()
            .name("My Contact Link 1#")
            .url(&valid_url)
            .build();
        assert!(contatlink3.is_err());

        let contatlink4 = ContactLink::builder()
            .name("My Contact Link 1$")
            .url(&valid_url)
            .build();
        assert!(contatlink4.is_err());

        let contatlink5 = ContactLink::builder()
            .name("My Contact Link 1%")
            .url(&valid_url)
            .build();
        assert!(contatlink5.is_err());

        let contatlink6 = ContactLink::builder()
            .name("My Contact Link 1&")
            .url(&valid_url)
            .build();
        assert!(contatlink6.is_err());
    }
}
