use super::{Role, RoleRef};
use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Represents a [Tenancy](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/users-and-roles/multi-tenancy/index.html) entity in Opsview.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Tenancy {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the `Tenancy`.
    pub name: String,

    // Semi-optional fields ----------------------------------------------------------------------//
    /// Primary role of the `Tenancy`.
    pub primary_role: Option<RoleRef>,

    // Optional fields ---------------------------------------------------------------------------//
    /// Optional description of the `Tenancy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    // Read-only fields --------------------------------------------------------------------------//
    /// The unique identifier of the `Tenancy`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,

    /// A reference string unique to this Tenancy.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    /// Optional priority of the `Tenancy`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub priority: Option<u64>,

    /// A boolean indicating whether the `Tenancy` is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

/// Enables the creation of a [`Tenancy`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for Tenancy {}

impl ConfigObject for Tenancy {
    type Builder = TenancyBuilder;

    /// Returns a builder for constructing a [`Tenancy`] object.
    ///
    /// # Returns
    /// A [`TenancyBuilder`] object.
    fn builder() -> Self::Builder {
        TenancyBuilder::new()
    }

    /// Provides the configuration path for a [`Tenancy`] object within the Opsview system.
    ///
    /// # Returns
    /// A string representing the API path where Tenancy are configured.
    fn config_path() -> Option<String> {
        Some("/config/tenancy".to_string())
    }

    /// Returns the unique name of the [`Tenancy`] object.
    ///
    /// This name is used to identify the `Tenancy` when building the `HashMap` for a
    /// [`ConfigObjectMap`].
    ///
    /// # Returns
    /// A string representing the unique name of the `Tenancy` entity.
    fn unique_name(&self) -> String {
        self.name.clone()
    }

    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            name: validate_and_trim_tenancy_name(name)?,
            ..Default::default()
        })
    }
}

impl Persistent for Tenancy {
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
        Some(TENANCY_NAME_REGEX_STR.to_string())
    }

    fn validated_name(&self, name: &str) -> Result<String, OpsviewConfigError> {
        validate_and_trim_tenancy_name(name)
    }

    fn set_name(&mut self, new_name: &str) -> Result<String, OpsviewConfigError> {
        self.name = self.validated_name(new_name)?;
        Ok(self.name.clone())
    }

    fn clear_readonly(&mut self) {
        self.id = None;
        self.priority = None;
        self.ref_ = None;
        self.uncommitted = None;
    }
}

impl PersistentMap for ConfigObjectMap<Tenancy> {
    fn config_path() -> Option<String> {
        Some("/config/tenancy".to_string())
    }
}

/// Builder for [`Tenancy`] objects, used to simplify the creation of new instances.
///
/// # Example
/// ```rust
/// use opsview::config::{Role, Tenancy};
/// use opsview::prelude::*;
///
/// let my_role = Role::minimal("My Role").unwrap();
///    
/// let tenancy = Tenancy::builder()
///    .name("My Tenancy")
///    .primary_role(my_role)
///    .build()
///    .unwrap();
///
/// assert_eq!(tenancy.name, "My Tenancy".to_string());
/// ```
#[derive(Clone, Debug, Default)]
pub struct TenancyBuilder {
    // Required fields ---------------------------------------------------------------------------//
    name: Option<String>,
    primary_role: Option<RoleRef>,
    // Optional fields ---------------------------------------------------------------------------//
    description: Option<String>,
}

impl Builder for TenancyBuilder {
    type ConfigObject = Tenancy;

    /// Creates a new [`TenancyBuilder`] instance with default values.
    ///
    /// # Returns
    /// A `TenancyBuilder` instance.
    fn new() -> Self {
        TenancyBuilder::default()
    }

    /// Sets the name field.
    ///
    /// # Arguments
    /// * `name` - The name of the `Tenancy`.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Consumes the builder and returns a [`Tenancy`] object.
    ///
    /// # Returns
    /// A `Tenancy` object.
    ///
    /// # Errors
    /// If the `name` field is not set or invalid, an `Error` will be returned.
    /// If the `primary_role` field is not set, an `Error` will be returned.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;
        let primary_role = require_field(&self.primary_role, "primary_role")?;
        let validated_description =
            validate_opt_string(self.description, validate_and_trim_description)?;

        Ok(Tenancy {
            name: validate_and_trim_tenancy_name(&name)?,
            primary_role: Some(primary_role),
            description: validated_description,
            id: None,
            priority: None,
            ref_: None,
            uncommitted: None,
        })
    }
}

impl TenancyBuilder {
    /// Clears the name field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Clears the description field.
    pub fn clear_description(mut self) -> Self {
        self.description = None;
        self
    }

    /// Clears the primary_role field.
    pub fn clear_primary_role(mut self) -> Self {
        self.primary_role = None;
        self
    }

    /// Sets the description field.
    ///
    /// # Arguments
    /// * `description` - The description of the `Tenancy`.
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Sets the primary_role field.
    ///
    /// # Arguments
    /// * `primary_role` - The primary [`Role`] of the `Tenancy`.
    pub fn primary_role(mut self, primary_role: Role) -> Self {
        self.primary_role = Some(RoleRef::from(primary_role));
        self
    }
}

/// A reference version of [`Tenancy`] that is used when passing or retrieving a
/// [`Tenancy`] object as part of another object.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct TenancyRef {
    name: String,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
}

/// Enables the creation of a [`TenancyRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for TenancyRef {}

impl ConfigRef for TenancyRef {
    type FullObject = Tenancy;

    /// Returns the reference string of the [`TenancyRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`TenancyRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`TenancyRef`] object.
    ///
    /// This name is used to identify the `TenancyRef` when building the `HashMap` for a
    /// [`ConfigRefMap`].
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

impl From<Tenancy> for TenancyRef {
    /// Creates a [`TenancyRef`] object from a [`Tenancy`] object.
    ///
    /// # Arguments
    /// * `tenancy` - A [`Tenancy`] object.
    ///
    /// # Returns
    /// A [`TenancyRef`] object.
    fn from(tenancy: Tenancy) -> Self {
        Self {
            name: tenancy.name.clone(),
            ref_: tenancy.ref_.clone(),
        }
    }
}

impl From<Arc<Tenancy>> for TenancyRef {
    fn from(item: Arc<Tenancy>) -> Self {
        let cmd: Tenancy = Arc::try_unwrap(item).unwrap_or_else(|arc| (*arc).clone());
        TenancyRef::from(cmd)
    }
}

impl From<&ConfigObjectMap<Tenancy>> for ConfigRefMap<TenancyRef> {
    fn from(tenancies: &ConfigObjectMap<Tenancy>) -> Self {
        ref_map_from(tenancies)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tenancy_default() {
        let tenancy = Tenancy::default();

        assert_eq!(tenancy.name, "".to_string());
        assert_eq!(tenancy.description, None);
        assert_eq!(tenancy.id, None);
        assert_eq!(tenancy.priority, None);
        assert_eq!(tenancy.ref_, None);
        assert_eq!(tenancy.primary_role, None);
    }

    #[test]
    fn test_tenancy_minimal() {
        let tenancy = Tenancy::minimal("My Tenancy");

        assert_eq!(tenancy.unwrap().name, "My Tenancy".to_string());
    }

    #[test]
    fn test_tenancy_unique_name() {
        let tenancy = Tenancy::builder()
            .name("My Tenancy")
            .primary_role(Role::default())
            .build()
            .unwrap();

        assert_eq!(tenancy.unique_name(), "My Tenancy".to_string());
    }

    #[test]
    fn test_tenancy_builder() {
        let tenancy = Tenancy::builder()
            .name("My Tenancy")
            .description("My Tenancy description")
            .primary_role(Role::minimal("My Role").unwrap())
            .build()
            .unwrap();

        assert_eq!(tenancy.name, "My Tenancy".to_string());
        assert_eq!(
            tenancy.description.unwrap(),
            "My Tenancy description".to_string()
        );
        assert_eq!(tenancy.id, None);
        assert_eq!(tenancy.priority, None);
        assert_eq!(tenancy.ref_, None);
        assert_eq!(tenancy.primary_role.unwrap().name(), "My Role".to_string());
    }

    #[test]
    fn test_is_valid_tenancy_name() {
        // Test valid names
        assert!(validate_and_trim_tenancy_name("ValidName123").is_ok());
        assert!(validate_and_trim_tenancy_name("Valid_Name-With.Symbols!").is_ok());
        assert!(validate_and_trim_tenancy_name("A").is_ok());
        assert!(validate_and_trim_tenancy_name("1").is_ok());
        assert!(validate_and_trim_tenancy_name("A name with spaces and symbols *&^%$#@!").is_ok());
        assert!(validate_and_trim_tenancy_name(&"a".repeat(191)).is_ok()); // Max length

        // Test invalid names
        assert!(validate_and_trim_tenancy_name("").is_err()); // Empty name
        assert!(validate_and_trim_tenancy_name(" ").is_err()); // Name with only space
        assert!(validate_and_trim_tenancy_name(&"a".repeat(192)).is_err()); // Exceeds max length
        assert!(validate_and_trim_tenancy_name("Invalid\nName").is_err()); // Contains newline
        assert!(validate_and_trim_tenancy_name("Invalid\tName").is_err()); // Contains tab
        assert!(validate_and_trim_tenancy_name("Invalid\rName").is_err()); // Contains carriage return
    }
}
