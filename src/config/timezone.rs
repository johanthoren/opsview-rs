use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};

/// Represents a Time Zone in Opsview.
///
/// These are not used directly in Opsview, or configured via the Opsview API, but are used to
/// represent the `TimeZone` of a [`super::TimePeriod`] object.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct TimeZone {
    // TODO: Add validation of this field.
    /// The name of the `TimeZone`.
    pub name: String,

    /// Unique reference string unique to this time zone.
    #[serde(rename = "ref")]
    pub ref_: String,
}

/// Enables the creation of a `TimeZone` instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for TimeZone {}

impl ConfigObject for TimeZone {
    type Builder = TimeZoneBuilder;

    /// Returns a builder for constructing a `TimeZone` object.
    ///
    /// # Returns
    /// A `TimeZoneBuilder` object.
    fn builder() -> Self::Builder {
        TimeZoneBuilder::new()
    }

    /// Provides the configuration path for a `TimeZone` object within the Opsview system.
    ///
    /// # Returns
    /// None, as time zones are not configured via the Opsview API.
    fn config_path() -> Option<String> {
        None
    }

    /// Returns the unique name of the `TimeZone` object.
    ///
    /// This name is used to identify the `TimeZone` when building the `HashMap` for an
    /// [`ConfigObjectMap`].
    ///
    /// # Returns
    /// A string representing the unique name of the `TimeZone`.
    fn unique_name(&self) -> String {
        self.ref_.clone()
    }

    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            name: name.to_string(),
            ..Default::default()
        })
    }
}

/// Builder for `TimeZone` objects, used to simplify the creation of new instances.
///
/// # Example
/// ```rust
/// use opsview::config::TimeZone;
/// use opsview::prelude::*;
///
/// let timezone = TimeZone::builder()
///    .name("SYSTEM")
///    .ref_("/rest/config/timezone/1")
///    .build()
///    .unwrap();
///
/// assert_eq!(timezone.name, "SYSTEM".to_string());
/// assert_eq!(timezone.ref_, "/rest/config/timezone/1".to_string());
/// ```
#[derive(Clone, Debug, Default)]
pub struct TimeZoneBuilder {
    name: Option<String>,
    ref_: Option<String>,
}

impl Builder for TimeZoneBuilder {
    type ConfigObject = TimeZone;

    /// Creates a new `TimeZoneBuilder` instance with default values.
    ///
    /// # Returns
    /// A TimeZoneBuilder instance.
    fn new() -> Self {
        Self::default()
    }

    /// Sets the name field.
    ///
    /// # Arguments
    /// * `name` - The name of the `TimeZone`.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Builds a new `TimeZone` instance using the `TimeZoneBuilder`.
    ///
    /// # Returns
    /// A `TimeZone` instance.
    ///
    /// # Errors
    /// If the name field is not set, an error will be returned.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;
        let ref_ = require_field(&self.ref_, "ref_")?;

        Ok(TimeZone { name, ref_ })
    }
}

impl TimeZoneBuilder {
    /// Clears the name field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Clears the ref_ field.
    pub fn clear_ref_(mut self) -> Self {
        self.ref_ = None;
        self
    }

    /// Sets the ref_ field.
    ///
    /// # Arguments
    /// * `ref_` - The reference string for the `TimeZone`.
    pub fn ref_(mut self, ref_: &str) -> Self {
        self.ref_ = Some(ref_.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_timezone_default() {
        let timezone = TimeZone::default();

        assert_eq!(timezone.name, "".to_string());
        assert_eq!(timezone.ref_, "".to_string());
    }

    #[test]
    fn test_timezone_minimal() {
        let timezone =
            TimeZone::minimal("SYSTEM").expect("Failed to create TimeZone with name 'SYSTEM'");

        assert_eq!(timezone.name, "SYSTEM".to_string());
        assert_eq!(timezone.ref_, "".to_string());
    }

    #[test]
    fn test_timezone_unique_name() {
        let timezone = TimeZone::minimal("SYSTEM");

        assert_eq!(timezone.unwrap().unique_name(), "".to_string());
    }

    #[test]
    fn test_timezone_builder() {
        let timezone = TimeZone::builder()
            .name("SYSTEM")
            .ref_("/rest/config/timezone/1")
            .build()
            .unwrap();

        assert_eq!(timezone.name, "SYSTEM".to_string());
        assert_eq!(timezone.ref_, "/rest/config/timezone/1".to_string());
    }

    #[test]
    fn test_timezone_builder_missing_name() {
        let timezone = TimeZone::builder().ref_("/rest/config/timezone/1").build();

        assert!(timezone.is_err());
        assert_eq!(
            timezone.unwrap_err().to_string(),
            "Mandatory field 'name' cannot be empty"
        );
    }

    #[test]
    fn test_timezone_builder_missing_ref() {
        let timezone = TimeZone::builder().name("SYSTEM").build();

        assert!(timezone.is_err());
        assert_eq!(
            timezone.unwrap_err().to_string(),
            "Mandatory field 'ref_' cannot be empty"
        );
    }
}
