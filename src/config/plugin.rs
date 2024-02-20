use super::{HostRef, ServiceCheckRef};
use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};

/// Represents a [Plugin](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/service-checks-and-host/active-checks/index.html#Heading-overview) in Opsview.
///
/// Plugins are used by Opsview to perform checks on hosts and services. This struct defines the
/// structure for a plugin entity as used in Opsview.
///
/// # Example
/// ```rust
/// use opsview::config::Plugin;
/// use opsview::prelude::*;
///
/// let plugin = Plugin::builder()
///   .name("My Plugin")
///   .build()
///   .unwrap();
///
///   assert_eq!(plugin.name, "My Plugin".to_string());
/// ```
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Plugin {
    // Required fields ---------------------------------------------------------------------------//
    // TODO: Add validation of this field.
    /// The name of the `Plugin`.
    pub name: String,

    // Optional fields ---------------------------------------------------------------------------//
    // TODO: Add validation of this field.
    /// A comma separated list of environment variables for the `Plugin`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub envvars: Option<String>,

    /// A integer representing the origin ID of the `Plugin`. 0 = Built-in, 1 = User uploaded.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub origin_id: Option<u64>,

    // Read-only fields --------------------------------------------------------------------------//
    /// [`ConfigRefMap`] of [`HostRef`] objects associated with this `Plugin`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostcheckcommands: Option<ConfigRefMap<HostRef>>,

    /// [`ConfigRefMap`] of [`ServiceCheckRef`] objects associated with this `Plugin`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servicechecks: Option<ConfigRefMap<ServiceCheckRef>>,

    /// A boolean indicating whether the `Plugin` is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

/// Enables the creation of a [`Plugin`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for Plugin {}

impl ConfigObject for Plugin {
    type Builder = PluginBuilder;

    /// Returns a builder for constructing a [`Plugin`] object.
    ///
    /// # Returns
    /// A [`PluginBuilder`] object.
    fn builder() -> Self::Builder {
        PluginBuilder::new()
    }

    /// Provides the configuration path for a [`Plugin`] object within the Opsview system.
    ///
    /// # Returns
    /// A string representing the API path where plugins are configured.
    fn config_path() -> Option<String> {
        Some("/config/plugin".to_string())
    }

    /// Returns the unique name of the [`Plugin`] object.
    ///
    /// This name is used to identify the `Plugin` when building the `HashMap` for an
    /// [`ConfigObjectMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

// impl Persistent for Plugin {
//     /// Returns the unique identifier, but since plugins do not have a unique identifier, this
//     /// function always returns `None`.
//     fn id(&self) -> Option<u64> {
//         None
//     }

//     /// Returns the reference string, but since plugins do not have a reference string, this
//     /// function always returns `None`.
//     fn ref_(&self) -> Option<String> {
//         None
//     }
//     /// Returns the name if it's not empty.
//     fn name(&self) -> Option<String> {
//         if self.name.is_empty() {
//             None
//         } else {
//             Some(self.name.clone())
//         }
//     }
// }

/// Builder for creating instances of [`Plugin`].
///
/// Provides a fluent interface for constructing a `Plugin` object with optional fields.
#[derive(Clone, Debug, Default)]
pub struct PluginBuilder {
    // Required fields ---------------------------------------------------------------------------//
    name: Option<String>,
    // Optional fields ---------------------------------------------------------------------------//
    envvars: Option<String>,
    origin_id: Option<u64>,
}

impl Builder for PluginBuilder {
    type ConfigObject = Plugin;

    /// Creates a new instance of [`PluginBuilder`] with default values.
    ///
    /// Initializes a new builder for creating a [`Plugin`] object with all fields in their default
    /// state.
    ///
    /// # Returns
    /// A new instance of `PluginBuilder`.
    fn new() -> Self {
        PluginBuilder::default()
    }

    /// Sets the name field.
    ///
    /// # Arguments
    /// * `name` - The name of the `Plugin`.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Consumes the builder and returns a [`Plugin`] object.
    ///
    /// # Returns
    /// A `Result` containing the constructed `Plugin` object or an error if the object
    /// could not be built due to the absence of the required `name` field or if the
    /// `origin_id` field is set but not 0 or 1.
    ///
    /// # Errors
    /// Returns an error if the name field is not set.
    /// Returns an error if the origin_id field is set but > 1.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;

        if self.origin_id.is_some() && self.origin_id.unwrap() > 1 {
            return Err(OpsviewConfigError::InvalidPluginOriginID);
        }

        Ok(Plugin {
            name,
            envvars: self.envvars,
            origin_id: self.origin_id,
            hostcheckcommands: None,
            servicechecks: None,
            uncommitted: None,
        })
    }
}

impl PluginBuilder {
    /// Clears the envvars field.
    pub fn clear_envvars(mut self) -> Self {
        self.envvars = None;
        self
    }

    /// Clears the name field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Clears the origin_id field.
    pub fn clear_origin_id(mut self) -> Self {
        self.origin_id = None;
        self
    }

    /// Alias for setting the origin_id field.
    ///
    /// # Arguments
    /// * `built_in` - A boolean indicating whether the `Plugin` is built-in.
    pub fn built_in(mut self, built_in: bool) -> Self {
        self.origin_id = Some(if built_in { 0 } else { 1 });
        self
    }

    /// Sets the envvars field.
    ///
    /// # Arguments
    /// * `envvars` - The environment variables for the `Plugin`.
    pub fn envvars(mut self, envvars: &str) -> Self {
        self.envvars = Some(envvars.to_string());
        self
    }

    /// Sets the origin_id field.
    ///
    /// # Arguments
    /// * `origin_id` - The origin ID of the `Plugin`. Must be 0 (for built-in plugins) or 1 (for user-uploaded plugins).
    pub fn origin_id(mut self, origin_id: u64) -> Self {
        self.origin_id = Some(origin_id);
        self
    }

    /// Alias for setting the origin_id field.
    ///
    /// # Arguments
    /// * `user_uploaded` - A boolean indicating whether the `Plugin` is user-uploaded.
    pub fn user_uploaded(mut self, user_uploaded: bool) -> Self {
        self.origin_id = Some(if user_uploaded { 1 } else { 0 });
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let plugin = Plugin::default();
        assert_eq!(plugin.name, "".to_string());
    }

    #[test]
    fn test_minimal() {
        let plugin = Plugin::minimal("My Plugin");
        assert_eq!(plugin.unwrap().name, "My Plugin".to_string());
    }
}
