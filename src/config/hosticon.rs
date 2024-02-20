use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};

/// Represents a [HostIcon](https://docs.itrsgroup.com/docs/opsview/6.8.9/administration/customization/index.html#Heading-managing-host-icons) in Opsview.
///
/// Host icons are used to differentiate between [`super::Host`]s of different kinds in Opsview. This struct
/// defines the structure for a host icon entity as used in Opsview.
///
/// # Example
/// ```rust
/// use opsview::config::HostIcon;
/// use opsview::prelude::*;
///
/// let host_icon = HostIcon::builder()
///    .name("LOGO - Some Brand")
///    .img_prefix("/images/logos/somebrand")
///    .build()
///    .unwrap();
///
/// assert_eq!(host_icon.name, "LOGO - Some Brand");
/// ```
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct HostIcon {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the `HostIcon`.
    pub name: String,

    // Semi-optional fields ----------------------------------------------------------------------//
    /// The prefix for the `HostIcon` image.
    ///
    /// # Example
    /// * `/images/logos/somebrand` for host icons in the folder `/images/logos/somebrand`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img_prefix: Option<String>,

    // Read-only fields --------------------------------------------------------------------------//
    /// A reference string unique to this `HostIcon`.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,
}

/// Enables the creation of a [`HostIcon`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for HostIcon {}

impl ConfigObject for HostIcon {
    type Builder = HostIconBuilder;

    /// Returns a builder for constructing a [`HostIcon`] object.
    ///
    /// # Returns
    /// A [`HostIconBuilder`] object.
    fn builder() -> Self::Builder {
        HostIconBuilder::new()
    }

    /// Provides the configuration path for a [`HostIcon`] object within the Opsview system.
    ///
    /// # Returns
    /// A string representing the API path where host icons are configured.
    fn config_path() -> Option<String> {
        Some("/config/hosticons".to_string())
    }

    /// Returns the unique name of the [`HostIcon`] object.
    /// This name is used to identify the `HostIcon` when building the `HashMap` for an
    /// [`ConfigObjectMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }

    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            name: validate_and_trim_hosticon_name(name)?,
            ..Default::default()
        })
    }
}

impl Persistent for HostIcon {
    /// Returns the unique identifier. Since `HostIcon` objects lack this field, `None` is returned.
    fn id(&self) -> Option<u64> {
        None
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
        Some(INLINE_FREE_TEXT_REGEX_STR.to_string())
    }

    fn validated_name(&self, name: &str) -> Result<String, OpsviewConfigError> {
        validate_and_trim_hosticon_name(name)
    }

    fn set_name(&mut self, new_name: &str) -> Result<String, OpsviewConfigError> {
        self.name = self.validated_name(new_name)?;
        Ok(self.name.clone())
    }

    fn clear_readonly(&mut self) {
        self.ref_ = None;
    }
}

impl PersistentMap for ConfigObjectMap<HostIcon> {
    fn config_path() -> Option<String> {
        Some("/config/hosticons".to_string())
    }
}

/// Builder for creating instances of [`HostIcon`].
///
/// Provides a fluent interface for constructing a `HostIcon` object with optional fields.
#[derive(Clone, Debug, Default)]
pub struct HostIconBuilder {
    img_prefix: Option<String>,
    name: Option<String>,
}

impl Builder for HostIconBuilder {
    type ConfigObject = HostIcon;

    /// Creates a new instance of [`HostIconBuilder`] with default values.
    ///
    /// Initializes a new builder for creating a [`HostIcon`] object with all fields in their default
    /// state.
    fn new() -> Self {
        HostIconBuilder::default()
    }

    /// Sets the name field.
    ///
    /// # Arguments
    /// * `name` - The name of the `HostIcon`.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Builds the [`HostIcon`] object with the specified properties.
    ///
    /// Constructs a new `HostIcon` object based on the current state of the builder.
    /// Returns an error if the required fields `name` or `img_prefix` are not set.
    ///
    /// # Returns
    /// A `Result` containing the constructed `HostIcon` object or an error if the object
    /// could not be built due to the absence of the required fields `name` and `img_prefix`.
    ///
    /// # Errors
    /// * `name not set` - The required `name` field was not set.
    /// * `img_prefix not set` - The required `img_prefix` field was not set.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;
        let img_prefix = require_field(&self.img_prefix, "img_prefix")?;

        Ok(HostIcon {
            name: validate_and_trim_hosticon_name(&name)?,
            // From what I can tell, the name and the img_prefix share the same regex validation.
            img_prefix: Some(validate_and_trim_hosticon_name(&img_prefix)?),
            ref_: None,
        })
    }
}

impl HostIconBuilder {
    /// Clears the img_prefix field.
    pub fn clear_img_prefix(mut self) -> Self {
        self.img_prefix = None;
        self
    }

    /// Clears the name field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Sets the img_prefix field.
    ///
    /// # Arguments
    /// * `img_prefix` - The prefix for the `HostIcon` image.
    ///
    /// # Example
    /// * `/images/logos/somebrand` for host icons in the folder `/images/logos/somebrand`.
    pub fn img_prefix(mut self, img_prefix: &str) -> Self {
        self.img_prefix = Some(img_prefix.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let icon = HostIcon::default();

        assert!(icon.name.is_empty());
    }

    #[test]
    fn test_minimal() {
        let icon = HostIcon::minimal("My `HostIcon`");

        assert_eq!(icon.unwrap().name, "My `HostIcon`".to_string());
    }
    #[test]
    fn test_is_valid_host_icon() {
        // Test valid host icon names
        assert!(validate_and_trim_hosticon_name("valid_icon_name").is_ok());
        assert!(validate_and_trim_hosticon_name("icon_123").is_ok());
        assert!(validate_and_trim_hosticon_name("foobar#£$%&/()=?^").is_ok());
        assert!(validate_and_trim_hosticon_name(&"i".repeat(128)).is_ok()); // Max length

        // Test invalid host icon names
        assert!(validate_and_trim_hosticon_name("").is_err()); // Empty name
        assert!(validate_and_trim_hosticon_name(&"i".repeat(129)).is_err()); // Exceeds max length
                                                                             // Additional tests for other specific invalid scenarios can be added
    }
}
