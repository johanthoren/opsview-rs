use crate::{config::*, prelude::*, util::*};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;

/// Represents a [Role](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/users-and-roles/roles/index.html#Heading-overview) in Opsview.
///
/// Roles are used to define a set of permissions for users within the Opsview system.
///
/// This struct defines the structure for a role entity as used in Opsview.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Role {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the `Role`.
    pub name: String,

    // Optional fields ---------------------------------------------------------------------------//
    /// [`ConfigRefMap`] of [`HostGroupRef`] objects that the `Role` has access to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_hostgroups: Option<ConfigRefMap<HostGroupRef>>,

    /// [`ConfigRefMap`] of [`HashtagRef`] objects that the `Role` has access to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_keywords: Option<ConfigRefMap<HashtagRef>>,

    /// [`ConfigRefMap`] of [`ServiceGroupRef`] objects that the `Role` has access to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_servicegroups: Option<ConfigRefMap<ServiceGroupRef>>,

    /// [`HashSet`] of [`Access`] objects that the `Role` has access to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accesses: Option<HashSet<Access>>,

    /// A boolean indicating whether the `Role` has access to all [`super::BSMComponent`]s.
    /// Default: `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub all_bsm_components: Option<bool>,

    /// A boolean indicating whether the `Role` has access to all BSM edit operations.
    /// `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub all_bsm_edit: Option<bool>,

    /// A boolean indicating whether the `Role` has access to all BSM view operations.
    /// `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub all_bsm_view: Option<bool>,

    /// A boolean indicating whether the `Role` has access to all [`HostGroup`]s.
    /// `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub all_hostgroups: Option<bool>,

    /// A boolean indicating whether the `Role` has access to all [`Hashtag`]s.
    /// `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub all_keywords: Option<bool>,

    /// A boolean indicating whether the `Role` has access to all monitoring servers.
    /// `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub all_monitoringservers: Option<bool>,

    /// A boolean indicating whether the `Role` has access to all remotely managed clusters.
    /// `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub all_remotely_managed_clusters: Option<bool>,

    /// A boolean indicating whether the `Role` has access to all [`ServiceGroup`]s.
    /// `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub all_servicegroups: Option<bool>,

    /// [`ConfigRefMap`] of [`ContactRef`] objects associated with this role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<ConfigRefMap<ContactRef>>,

    /// A description of the `Role`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// [`ConfigRefMap`] of [`HostGroupRef`] objects associated with this role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostgroups: Option<ConfigRefMap<HostGroupRef>>,

    /// [`ConfigObjectMap`] of [`MonitoringServer`] objects associated with this role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoringservers: Option<ConfigObjectMap<MonitoringServer>>,

    /// [`ConfigRefMap`] of remotely managed [`MonitoringClusterRef`] objects associated with this role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remotely_managed_clusters: Option<ConfigRefMap<MonitoringClusterRef>>,

    /// The [`TenancyRef`] of the `Role`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<TenancyRef>,

    // Read-only fields --------------------------------------------------------------------------//
    /// The unique identifier of the `Role`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,

    /// A reference string unique to this Role.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    /// A boolean indicating whether the `Role` is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

impl Default for Role {
    fn default() -> Self {
        Role {
            name: String::new(),
            access_hostgroups: None,
            access_keywords: None,
            access_servicegroups: None,
            accesses: None,
            all_bsm_components: Some(false),
            all_bsm_edit: Some(false),
            all_bsm_view: Some(false),
            all_hostgroups: Some(false),
            all_keywords: Some(false),
            all_monitoringservers: Some(false),
            all_remotely_managed_clusters: Some(false),
            all_servicegroups: Some(false),
            contacts: None,
            description: None,
            hostgroups: None,
            id: None,
            monitoringservers: None,
            ref_: None,
            remotely_managed_clusters: None,
            tenancy: None,
            uncommitted: None,
        }
    }
}

/// Enables the creation of a [`Role`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for Role {}

impl ConfigObject for Role {
    type Builder = RoleBuilder;

    /// Returns a builder for constructing a [`Role`] object.
    ///
    /// # Returns
    /// A [`RoleBuilder`] object.
    fn builder() -> Self::Builder {
        RoleBuilder::new()
    }

    /// Provides the configuration path for a [`Role`] object within the Opsview system.
    ///
    /// # Returns
    /// A string representing the API path where roles are configured.
    fn config_path() -> Option<String> {
        Some("/config/role".to_string())
    }

    /// Returns the unique name of the [`Role`] object.
    ///
    /// This name is used to identify the `Role` when building the `HashMap` for an
    /// [`ConfigObjectMap`].
    ///
    /// # Returns
    /// A string representing the unique name of the `Role`.
    fn unique_name(&self) -> String {
        if self.tenancy.is_some() {
            format!("{}:{}", self.name, self.tenancy.as_ref().unwrap().name())
        } else {
            self.name.clone()
        }
    }
}

impl Persistent for Role {
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
        Some(ROLE_NAME_REGEX_STR.to_string())
    }

    fn validated_name(&self, name: &str) -> Result<String, OpsviewConfigError> {
        validate_and_trim_role_name(name)
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

impl PersistentMap for ConfigObjectMap<Role> {
    fn config_path() -> Option<String> {
        Some("/config/role".to_string())
    }
}
/// Builder for creating instances of [`Role`].
///
/// Provides a fluent interface for constructing a `Role` object, allowing for
/// customizable construction and ensuring that the created object conforms to required parameters
/// and defaults.
///
/// # Example
/// ```rust
/// use opsview::config::Role;
/// use opsview::prelude::*;
///
/// let role = Role::builder()
///  .name("My Role")
///  .build()
///  .unwrap();
///
/// assert_eq!(role.name, "My Role".to_string());
/// ```
#[derive(Clone, Debug)]
pub struct RoleBuilder {
    // Required fields ---------------------------------------------------------------------------//
    name: Option<String>,
    // Optional fields ---------------------------------------------------------------------------//
    access_hostgroups: Option<ConfigRefMap<HostGroupRef>>,
    access_keywords: Option<ConfigRefMap<HashtagRef>>,
    access_servicegroups: Option<ConfigRefMap<ServiceGroupRef>>,
    accesses: Option<HashSet<Access>>,
    all_bsm_components: Option<bool>,
    all_bsm_edit: Option<bool>,
    all_bsm_view: Option<bool>,
    all_hostgroups: Option<bool>,
    all_keywords: Option<bool>,
    all_monitoringservers: Option<bool>,
    all_remotely_managed_clusters: Option<bool>,
    all_servicegroups: Option<bool>,
    contacts: Option<ConfigRefMap<ContactRef>>,
    description: Option<String>,
    hostgroups: Option<ConfigRefMap<HostGroupRef>>,
    monitoringservers: Option<ConfigObjectMap<MonitoringServer>>,
    remotely_managed_clusters: Option<ConfigRefMap<MonitoringClusterRef>>,
    tenancy: Option<TenancyRef>,
}

impl Default for RoleBuilder {
    fn default() -> Self {
        RoleBuilder {
            // Required fields -------------------------------------------------------------------//
            name: None,
            // Optional fields -------------------------------------------------------------------//
            access_hostgroups: None,
            access_keywords: None,
            access_servicegroups: None,
            accesses: None,
            all_bsm_components: Some(false),
            all_bsm_edit: Some(false),
            all_bsm_view: Some(false),
            all_hostgroups: Some(false),
            all_keywords: Some(false),
            all_monitoringservers: Some(false),
            all_remotely_managed_clusters: Some(false),
            all_servicegroups: Some(false),
            contacts: None,
            description: None,
            hostgroups: None,
            monitoringservers: None,
            remotely_managed_clusters: None,
            tenancy: None,
        }
    }
}

impl Builder for RoleBuilder {
    type ConfigObject = Role;

    /// Creates a new instance of [`RoleBuilder`] with default values.
    ///
    /// Initializes a new builder for creating a [`Role`] object with all fields in their default
    /// state.
    ///
    /// # Returns
    /// A new instance of `RoleBuilder`.
    fn new() -> Self {
        RoleBuilder::default()
    }

    /// Sets the name field.
    ///
    /// # Arguments
    /// * `name` - Name of the `Role`.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Consumes the builder and returns a [`Role`] object.
    ///
    /// # Returns
    /// A `Role` object with the values specified by the builder.
    ///
    /// # Errors
    /// Returns an error if the required `name` field is not set.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;

        let validated_description =
            validate_opt_string(self.description, validate_and_trim_description)?;

        Ok(Role {
            name: validate_and_trim_role_name(&name)?,
            access_hostgroups: self.access_hostgroups,
            access_keywords: self.access_keywords,
            access_servicegroups: self.access_servicegroups,
            accesses: self.accesses,
            all_bsm_components: self.all_bsm_components,
            all_bsm_edit: self.all_bsm_edit,
            all_bsm_view: self.all_bsm_view,
            all_hostgroups: self.all_hostgroups,
            all_keywords: self.all_keywords,
            all_monitoringservers: self.all_monitoringservers,
            all_remotely_managed_clusters: self.all_remotely_managed_clusters,
            all_servicegroups: self.all_servicegroups,
            contacts: self.contacts,
            description: validated_description,
            hostgroups: self.hostgroups,
            monitoringservers: self.monitoringservers,
            remotely_managed_clusters: self.remotely_managed_clusters,
            tenancy: self.tenancy,
            id: None,
            ref_: None,
            uncommitted: None,
        })
    }
}

impl RoleBuilder {
    /// Sets the access_hostgroups field.
    ///
    /// # Arguments
    /// * `access_hostgroups` - [`ConfigObjectMap`] of [`HostGroup`] objects that the `Role` has access to.
    pub fn access_hostgroups(mut self, access_hostgroups: &ConfigObjectMap<HostGroup>) -> Self {
        self.access_hostgroups = Some(access_hostgroups.into());
        self
    }

    /// Sets the access_keywords field.
    ///
    /// # Arguments
    /// * `access_keywords` - A reference to a [`ConfigObjectMap`] of [`Hashtag`] objects that the `Role` has access to.
    pub fn access_keywords(mut self, access_keywords: &ConfigObjectMap<Hashtag>) -> Self {
        self.access_keywords = Some(access_keywords.into());
        self
    }

    /// Sets the access_servicegroups field.
    ///
    /// # Arguments
    /// * `access_servicegroups` - A reference to a [`ConfigObjectMap`] of [`ServiceGroup`] objects that the `Role` has access to.
    pub fn access_servicegroups(
        mut self,
        access_servicegroups: &ConfigObjectMap<ServiceGroup>,
    ) -> Self {
        self.access_servicegroups = Some(access_servicegroups.into());
        self
    }

    /// Sets the accesses field.
    ///
    /// # Arguments
    /// * `accesses` - `HashSet` of [`Access`] objects that the `Role` has access to.
    pub fn accesses(mut self, accesses: HashSet<Access>) -> Self {
        self.accesses = Some(accesses);
        self
    }

    /// Add a single [`Access`] to the accesses field.
    ///
    /// Will create a new [`HashSet`] if `accesses` is `None`.
    ///
    /// # Arguments
    /// * `access` - [`Access`] object to add to the accesses field.
    pub fn add_access(mut self, access: Access) -> Self {
        if let Some(ref mut accesses) = self.accesses {
            accesses.insert(access);
        } else {
            let mut accesses = HashSet::new();
            accesses.insert(access);
            self.accesses = Some(accesses);
        }
        self
    }

    /// Sets the all_bsm_components field.
    ///
    /// # Arguments
    /// * `all_bsm_components` - Boolean indicating whether the `Role` has access to all [`super::BSMComponent`]s.
    pub fn all_bsm_components(mut self, all_bsm_components: bool) -> Self {
        self.all_bsm_components = Some(all_bsm_components);
        self
    }

    /// Sets the all_bsm_edit field.
    ///
    /// # Arguments
    /// * `all_bsm_edit` - Boolean indicating whether the `Role` has access to all BSM edit operations.
    pub fn all_bsm_edit(mut self, all_bsm_edit: bool) -> Self {
        self.all_bsm_edit = Some(all_bsm_edit);
        self
    }

    /// Sets the all_bsm_view field.
    ///
    /// # Arguments
    /// * `all_bsm_view` - Boolean indicating whether the `Role` has access to all BSM view operations.
    pub fn all_bsm_view(mut self, all_bsm_view: bool) -> Self {
        self.all_bsm_view = Some(all_bsm_view);
        self
    }

    /// Sets the all_hostgroups field.
    ///
    /// # Arguments
    /// * `all_hostgroups` - Boolean indicating whether the `Role` has access to all [`HostGroup`]s.
    pub fn all_hostgroups(mut self, all_hostgroups: bool) -> Self {
        self.all_hostgroups = Some(all_hostgroups);
        self
    }

    /// Sets the all_keywords field.
    ///
    /// # Arguments
    /// * `all_keywords` - Boolean indicating whether the `Role` has access to all [`Hashtag`]s.
    pub fn all_keywords(mut self, all_keywords: bool) -> Self {
        self.all_keywords = Some(all_keywords);
        self
    }

    /// Sets the all_monitoringservers field.
    ///
    /// # Arguments
    /// * `all_monitoringservers` - Boolean indicating whether the `Role` has access to all monitoring servers.
    pub fn all_monitoringservers(mut self, all_monitoringservers: bool) -> Self {
        self.all_monitoringservers = Some(all_monitoringservers);
        self
    }

    /// Sets the all_remotely_managed_clusters field.
    ///
    /// # Arguments
    /// * `all_remotely_managed_clusters` - Boolean indicating whether the `Role` has access to all remotely managed clusters.
    pub fn all_remotely_managed_clusters(mut self, all_remotely_managed_clusters: bool) -> Self {
        self.all_remotely_managed_clusters = Some(all_remotely_managed_clusters);
        self
    }

    /// Sets the all_servicegroups field.
    ///
    /// # Arguments
    /// * `all_servicegroups` - Boolean indicating whether the `Role` has access to all [`ServiceGroup`]s.
    pub fn all_servicegroups(mut self, all_servicegroups: bool) -> Self {
        self.all_servicegroups = Some(all_servicegroups);
        self
    }

    /// Clears the access_hostgroups field.
    pub fn clear_access_hostgroups(mut self) -> Self {
        self.access_hostgroups = None;
        self
    }

    /// Clears the access_keywords field.
    pub fn clear_access_keywords(mut self) -> Self {
        self.access_keywords = None;
        self
    }

    /// Clears the access_servicegroups field.
    pub fn clear_access_servicegroups(mut self) -> Self {
        self.access_servicegroups = None;
        self
    }

    /// Clears the accesses field.
    pub fn clear_accesses(mut self) -> Self {
        self.accesses = None;
        self
    }

    /// Clears the all_bsm_components field.
    pub fn clear_all_bsm_components(mut self) -> Self {
        self.all_bsm_components = None;
        self
    }

    /// Clears the all_bsm_edit field.
    pub fn clear_all_bsm_edit(mut self) -> Self {
        self.all_bsm_edit = None;
        self
    }

    /// Clears the all_bsm_view field.
    pub fn clear_all_bsm_view(mut self) -> Self {
        self.all_bsm_view = None;
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

    /// Clears the all_monitoringservers field.
    pub fn clear_all_monitoringservers(mut self) -> Self {
        self.all_monitoringservers = None;
        self
    }

    /// Clears the all_remotely_managed_clusters field.
    pub fn clear_all_remotely_managed_clusters(mut self) -> Self {
        self.all_remotely_managed_clusters = None;
        self
    }

    /// Clears the all_servicegroups field.
    pub fn clear_all_servicegroups(mut self) -> Self {
        self.all_servicegroups = None;
        self
    }

    /// Clears the contacts field.
    pub fn clear_contacts(mut self) -> Self {
        self.contacts = None;
        self
    }

    /// Clears the description field.
    pub fn clear_description(mut self) -> Self {
        self.description = None;
        self
    }

    /// Clears the hostgroups field.
    pub fn clear_hostgroups(mut self) -> Self {
        self.hostgroups = None;
        self
    }

    /// Clears the monitoringservers field.
    pub fn clear_monitoringservers(mut self) -> Self {
        self.monitoringservers = None;
        self
    }

    /// Clears the name field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Clears the remotely_managed_clusters field.
    pub fn clear_remotely_managed_clusters(mut self) -> Self {
        self.remotely_managed_clusters = None;
        self
    }

    /// Clears the tenancy field.
    pub fn clear_tenancy(mut self) -> Self {
        self.tenancy = None;
        self
    }

    /// Sets the contacts field.
    ///
    /// # Arguments
    /// * `contacts` - A reference to a [`ConfigObjectMap`] of [`Contact`] objects associated with this role.
    pub fn contacts(mut self, contacts: &ConfigObjectMap<Contact>) -> Self {
        self.contacts = Some(contacts.into());
        self
    }

    /// Sets the description field.
    ///
    /// # Arguments
    /// * `description` - Description of the `Role`.
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Sets the hostgroups field.
    ///
    /// # Arguments
    /// * `hostgroups` - A reference to a [`ConfigObjectMap`] of [`HostGroup`] objects associated with this role.
    pub fn hostgroups(mut self, hostgroups: &ConfigObjectMap<HostGroup>) -> Self {
        self.hostgroups = Some(hostgroups.into());
        self
    }

    /// Sets the monitoringservers field.
    ///
    /// # Arguments
    /// * `monitoringservers` - A [`ConfigObjectMap`] of [`MonitoringServer`] objects associated with this role.
    pub fn monitoringservers(
        mut self,
        monitoringservers: ConfigObjectMap<MonitoringServer>,
    ) -> Self {
        self.monitoringservers = Some(monitoringservers);
        self
    }

    /// Sets the remotely_managed_clusters field.
    ///
    /// # Arguments
    /// * `remotely_managed_clusters` - A reference to a [`ConfigObjectMap`] of [`MonitoringCluster`] objects associated with this role.
    pub fn remotely_managed_clusters(
        mut self,
        remotely_managed_clusters: &ConfigObjectMap<MonitoringCluster>,
    ) -> Self {
        self.remotely_managed_clusters = Some(remotely_managed_clusters.into());
        self
    }

    /// Sets the tenancy field.
    ///
    /// # Arguments
    /// * `tenancy` - The [`Tenancy`] that the `Role` belongs to.
    pub fn tenancy(mut self, tenancy: Tenancy) -> Self {
        self.tenancy = Some(TenancyRef::from(tenancy));
        self
    }
}

/// A reference version of [`Role`] that is used when passing or retrieving a
/// [`Role`] object as part of another object.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct RoleRef {
    name: String,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
}

/// Enables the creation of a [`RoleRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for RoleRef {}

impl ConfigRef for RoleRef {
    type FullObject = Role;

    /// Returns the reference string of the [`RoleRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`RoleRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`RoleRef`] object.
    ///
    /// This name is used to identify the `RoleRef` when building the `HashMap` for a
    /// [`ConfigRefMap`].
    fn unique_name(&self) -> String {
        if self.ref_.is_some() {
            self.ref_.clone().unwrap()
        } else {
            self.name.clone()
        }
    }
}

impl From<Role> for RoleRef {
    fn from(full_object: Role) -> Self {
        Self {
            name: full_object.name.clone(),
            ref_: full_object.ref_.clone(),
        }
    }
}

impl From<Arc<Role>> for RoleRef {
    fn from(item: Arc<Role>) -> Self {
        let cmd: Role = Arc::try_unwrap(item).unwrap_or_else(|arc| (*arc).clone());
        RoleRef::from(cmd)
    }
}

impl From<&ConfigObjectMap<Role>> for ConfigRefMap<RoleRef> {
    fn from(roles: &ConfigObjectMap<Role>) -> Self {
        ref_map_from(roles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let role = Role::default();

        assert!(role.name.is_empty());
    }

    #[test]
    fn test_minimal() {
        let role = Role::minimal("my-role");

        assert_eq!(role.unwrap().name, "my-role".to_string());
    }

    #[test]
    fn test_add_access() {
        let role = Role::builder()
            .name("my-role")
            .add_access(Access::ViewAll(Some("/rest/config/access/4".to_string())))
            .add_access(Access::AdminAccess(None))
            .build()
            .unwrap();

        assert_eq!(role.accesses.unwrap().len(), 2);
    }

    #[test]
    fn test_validated_role_name() {
        // Test valid names
        assert!(validate_and_trim_role_name("Role 1").is_ok());
        assert!(validate_and_trim_role_name("Role-Name_2").is_ok());
        assert!(validate_and_trim_role_name("My, Role Name").is_ok());
        assert!(validate_and_trim_role_name("A").is_ok()); // Minimum valid case
        assert!(validate_and_trim_role_name(&"a".repeat(128)).is_ok());

        // Testd names wdith characters not allowed by the regex
        assert!(validate_and_trim_role_name("Role@Name").is_err()); // '@' not allowed
        assert!(validate_and_trim_role_name("Role\nName").is_err()); // Newline not allowed
        assert!(validate_and_trim_role_name("Role\tName").is_err()); // Tab not allowed
        assert!(validate_and_trim_role_name("").is_err()); // Empty string (handled separately)
        assert!(validate_and_trim_role_name(&"a".repeat(129)).is_err());
    }
}
