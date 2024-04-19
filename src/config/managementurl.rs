use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};

/// Represents a [ManagementURL](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/service-checks-and-host/host-templates/index.html#management-urls-tab) in Opsview.
///
/// There is no API endpoint for managing management urls, so this struct is only used as a
/// component of other entities.
///
/// This struct defines the structure for a management url entity as used in Opsview.
///
/// # Example
/// ```rust
/// use opsview::config::ManagementURL;
/// use opsview::prelude::*;
///
/// let management_url = ManagementURL::builder()
///     .name("My Management URL")
///     .url("https://www.example.com")
///     .build()
///     .unwrap();
///
/// assert_eq!(management_url.name, "My Management URL".to_string());
/// ```
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ManagementURL {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the `ManagementURL`. This is used as a unique identifier per user, but is not
    /// unique across all users.
    pub name: String,

    // Semi-optional fields ----------------------------------------------------------------------//
    /// URL associated with this `ManagementURL`.
    pub url: Option<String>,

    // Read-only fields --------------------------------------------------------------------------//
    /// The unique identifier of the `ManagementURL`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,
}

/// Enables the creation of a `ManagementURL` instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for ManagementURL {}

impl ConfigObject for ManagementURL {
    type Builder = ManagementURLBuilder;

    /// Returns a builder for constructing a [`ManagementURL`] object.
    fn builder() -> Self::Builder {
        ManagementURLBuilder::new()
    }

    /// Returns the unique name of the [`ManagementURL`] object.
    ///
    /// This name is used to identify the `ManagementURL` when building the `HashMap` for a
    /// [`ConfigObjectMap`]. Since the name is not unique across all users, the last 5 characters
    /// of the URL are appended to the name to ensure uniqueness. If the URL is less than 5
    /// characters, the entire URL is appended to the name, regardless of length.
    ///
    /// # Returns
    /// A string representing the unique name of the `ManagementURL` entity.
    fn unique_name(&self) -> String {
        if self.url.as_ref().is_some_and(|u| u.len() < 5) {
            return format!("{}{}", self.name, &self.url.as_ref().unwrap());
        } else if self.url.as_ref().is_some() {
            return format!(
                "{}{}",
                self.name,
                &self.url.as_ref().unwrap()[self.url.as_ref().unwrap().len() - 5..]
            );
        }

        self.name.clone()
    }

    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            name: name.to_string(),
            ..Default::default()
        })
    }
}

/// Builder for creating instances of [`ManagementURL`].
///
/// Provides a fluent interface for constructing a `ManagementURL` object with optional fields.
#[derive(Clone, Debug, Default)]
pub struct ManagementURLBuilder {
    name: Option<String>,
    url: Option<String>,
}

impl Builder for ManagementURLBuilder {
    type ConfigObject = ManagementURL;

    /// Creates a new instance of [`ManagementURLBuilder`] with default values.
    ///
    /// Initializes a new builder for creating a [`ManagementURL`] object with all fields in their
    /// default state.
    ///
    /// # Returns
    /// A `ManagementURLBuilder` object.
    fn new() -> Self {
        Self::default()
    }

    /// Sets the `name` field.
    ///
    /// # Arguments
    /// * `name` - The name of the management url.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Builds a new instance of [`ManagementURL`] using the configured values.
    ///
    /// # Returns
    /// A new instance of `ManagementURL`.
    ///
    /// # Errors
    /// Returns an `Error` if the `name` field is not set.
    /// Returns an `Error` if the `url` field is not set.
    fn build(self) -> Result<ManagementURL, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;
        let url = require_field(&self.url, "url")?;

        Ok(ManagementURL {
            name: validate_and_trim_managementurl_name(&name)?,
            url: Some(validate_uri(&url)?),
            id: None,
        })
    }
}

impl ManagementURLBuilder {
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

    /// Sets the `url` field.
    ///
    /// # Arguments
    /// * `url` - The URL associated with the management url.
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let url = ManagementURL::default();

        assert!(url.name.is_empty());
    }

    #[test]
    fn test_minimal() {
        let url = ManagementURL::minimal("My Management URL");

        assert_eq!(url.unwrap().name, "My Management URL".to_string());
    }
}
