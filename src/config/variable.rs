use super::ServiceCheckRef;
use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Represents a [Variable](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/variables/variables/index.html) in Opsview.
///
/// Variables are used to store information that can be used in other parts of the Opsview
/// configuration. For example, a variable can be used to store the IP address of a [`super::Host`], which can
/// then be used in a [`super::ServiceCheck`] command.
///
/// This struct defines the structure for a variable entity as used in Opsview. [In the
/// API](https://docs.itrsgroup.com/docs/opsview/current/rest-api/config/api-config-attribute/index.html),
/// variables are known as attributes for legacy reasons.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Variable {
    // Required fields ---------------------------------------------------------------------------//
    /// The name of the `Variable`.
    pub name: String,

    // Optional fields ---------------------------------------------------------------------------//
    /// Optional argument 1 for the `Variable`.
    ///
    /// Default: `Some(String::new())`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg1: Option<String>,

    /// Optional argument 2 for the `Variable`.
    ///
    /// Default: `Some(String::new())`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg2: Option<String>,

    /// Optional argument 3 for the `Variable`.
    ///
    /// Default: `Some(String::new())`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg3: Option<String>,

    /// Optional argument 4 for the `Variable`.
    ///
    /// Default: `Some(String::new())`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg4: Option<String>,

    /// Optional label for the `arg1` field.
    ///
    /// Default: `Some(String::new())`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label1: Option<String>,

    /// Optional label for the `arg2` field.
    ///
    /// Default: `Some(String::new())`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label2: Option<String>,

    /// Optional label for the `arg3` field.
    ///
    /// Default: `Some(String::new())`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label3: Option<String>,

    /// Optional label for the `arg4` field.
    ///
    /// Default: `Some(String::new())`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label4: Option<String>,

    /// Boolean indicating whether `arg1` should be secured.
    ///
    /// Default: `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub secured1: Option<bool>,

    /// Boolean indicating whether `arg2` should be secured.
    ///
    /// Default: `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub secured2: Option<bool>,

    /// Boolean indicating whether `arg3` should be secured.
    /// Default: `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub secured3: Option<bool>,

    /// Boolean indicating whether `arg4` should be secured.
    ///
    /// Default: `Some(false)`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub secured4: Option<bool>,

    /// Optional value for the `Variable`, acting as a default value.
    /// Default: `Some(String::new())`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    // Read-only fields --------------------------------------------------------------------------//
    /// The unique identifier of the `Variable`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub id: Option<u64>,

    /// A reference string unique to this `Variable`.
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    pub ref_: Option<String>,

    /// [`ConfigRefMap`] of `ServiceCheckRef` objects associated with this variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servicechecks: Option<ConfigRefMap<ServiceCheckRef>>,

    /// A boolean indicating whether the `Variable` is uncommitted.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    pub uncommitted: Option<bool>,
}

impl Default for Variable {
    fn default() -> Self {
        Self {
            name: String::new(),
            arg1: Some(String::new()),
            arg2: Some(String::new()),
            arg3: Some(String::new()),
            arg4: Some(String::new()),
            label1: Some(String::new()),
            label2: Some(String::new()),
            label3: Some(String::new()),
            label4: Some(String::new()),
            secured1: Some(false),
            secured2: Some(false),
            secured3: Some(false),
            secured4: Some(false),
            value: Some(String::new()),
            id: None,
            ref_: None,
            servicechecks: None,
            uncommitted: None,
        }
    }
}

/// Enables the creation of a [`Variable`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for Variable {}

impl ConfigObject for Variable {
    type Builder = VariableBuilder;

    /// Returns a builder for constructing a [`Variable`] object.
    ///
    /// # Returns
    /// A [`VariableBuilder`] object.
    fn builder() -> Self::Builder {
        VariableBuilder::new()
    }

    /// Provides the configuration path for a [`Variable`] object within the Opsview system.
    ///
    /// # Returns
    /// A string representing the API path where variables are configured.
    fn config_path() -> Option<String> {
        Some("/config/attribute".to_string())
    }

    /// Returns a minimal `Variable` object with only the name set.
    ///
    /// # Arguments
    /// * `name` - Name of the [`Variable`].
    ///
    /// # Returns
    /// A minimal `Variable` object with only the name set, and the rest of the fields in their
    /// default states.
    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Ok(Self {
            name: validate_and_trim_variable_name(name)?,
            ..Default::default()
        })
    }

    /// Returns the unique name of the [`Variable`] object.
    ///
    /// This name is used to identify the `Variable` when building the `HashMap` for an
    /// [`ConfigObjectMap`].
    ///
    /// # Returns
    /// A string representing the unique name of the `Variable`.
    fn unique_name(&self) -> String {
        match self.id {
            Some(id) => format!("{}_{}", self.name, id),
            None => self.name.clone(),
        }
    }
}

impl Persistent for Variable {
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
        Some(VARIABLE_NAME_REGEX_STR.to_string())
    }

    fn validated_name(&self, name: &str) -> Result<String, OpsviewConfigError> {
        validate_and_trim_variable_name(name)
    }

    fn set_name(&mut self, new_name: &str) -> Result<String, OpsviewConfigError> {
        self.name = self.validated_name(new_name)?;
        Ok(self.name.clone())
    }

    fn clear_readonly(&mut self) {
        self.id = None;
        self.ref_ = None;
        self.servicechecks = None;
        self.uncommitted = None;
    }
}

impl PersistentMap for ConfigObjectMap<Variable> {
    fn config_path() -> Option<String> {
        Some("/config/attribute".to_string())
    }
}

/// Builder for [`Variable`] objects, used to simplify the creation of new instances.
///
/// # Example
/// ```rust
/// use opsview::config::Variable;
/// use opsview::prelude::*;
///
/// let variable = Variable::builder()
///   .name("MY_VARIABLE")
///   .value("my_value")
///   .build()
///   .unwrap();
///
/// assert_eq!(variable.name, "MY_VARIABLE".to_string());
/// assert_eq!(variable.value, Some("my_value".to_string()));
/// ```
#[derive(Clone, Debug)]
pub struct VariableBuilder {
    // Required fields ---------------------------------------------------------------------------//
    name: Option<String>,
    // Optional fields ---------------------------------------------------------------------------//
    arg1: Option<String>,
    arg2: Option<String>,
    arg3: Option<String>,
    arg4: Option<String>,
    label1: Option<String>,
    label2: Option<String>,
    label3: Option<String>,
    label4: Option<String>,
    secured1: Option<bool>,
    secured2: Option<bool>,
    secured3: Option<bool>,
    secured4: Option<bool>,
    value: Option<String>,
}

impl Default for VariableBuilder {
    fn default() -> Self {
        Self {
            name: None,
            arg1: Some(String::new()),
            arg2: Some(String::new()),
            arg3: Some(String::new()),
            arg4: Some(String::new()),
            label1: Some(String::new()),
            label2: Some(String::new()),
            label3: Some(String::new()),
            label4: Some(String::new()),
            secured1: Some(false),
            secured2: Some(false),
            secured3: Some(false),
            secured4: Some(false),
            value: Some(String::new()),
        }
    }
}

impl Builder for VariableBuilder {
    type ConfigObject = Variable;

    /// Creates a new [`VariableBuilder`] instance used to construct a [`Variable`] object.
    ///
    /// # Returns
    /// A `VariableBuilder` instance.
    fn new() -> Self {
        Self::default()
    }

    /// Sets the `name` field.
    ///
    /// # Arguments
    /// * `name` - The name of the `Variable`.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Builds a new [`Variable`] instance using the [`VariableBuilder`].
    ///
    /// # Returns
    /// A `Variable` instance.
    ///
    /// # Errors
    /// * `name` not set or invalid.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;

        check_arg_label(&self.arg1, &self.label1, 1)?;
        check_arg_label(&self.arg2, &self.label2, 2)?;
        check_arg_label(&self.arg3, &self.label3, 3)?;
        check_arg_label(&self.arg4, &self.label4, 4)?;

        check_secured_arg_label(&self.secured1, &self.arg1, &self.label1, 1)?;
        check_secured_arg_label(&self.secured2, &self.arg2, &self.label2, 2)?;
        check_secured_arg_label(&self.secured3, &self.arg3, &self.label3, 3)?;
        check_secured_arg_label(&self.secured4, &self.arg4, &self.label4, 4)?;

        let validated_value = validate_opt_string(self.value, validate_variable_value)?;

        Ok(Variable {
            name: validate_and_trim_variable_name(&name)?,
            arg1: self.arg1,
            arg2: self.arg2,
            arg3: self.arg3,
            arg4: self.arg4,
            label1: self.label1,
            label2: self.label2,
            label3: self.label3,
            label4: self.label4,
            secured1: self.secured1,
            secured2: self.secured2,
            secured3: self.secured3,
            secured4: self.secured4,
            value: validated_value,
            id: None,
            ref_: None,
            servicechecks: None,
            uncommitted: None,
        })
    }
}

impl VariableBuilder {
    /// Sets the `arg1` field.
    ///
    /// # Arguments
    /// * `arg1` - The arg1 field.
    pub fn arg1(mut self, arg1: &str) -> Self {
        self.arg1 = Some(arg1.to_string());
        self
    }

    /// Sets the `arg2` field.
    ///
    /// # Arguments
    /// * `arg2` - The arg2 field.
    pub fn arg2(mut self, arg2: &str) -> Self {
        self.arg2 = Some(arg2.to_string());
        self
    }

    /// Sets the `arg3` field.
    ///
    /// # Arguments
    /// * `arg3` - The arg3 field.
    pub fn arg3(mut self, arg3: &str) -> Self {
        self.arg3 = Some(arg3.to_string());
        self
    }

    /// Sets the `arg4` field.
    ///
    /// # Arguments
    /// * `arg4` - The arg4 field.
    pub fn arg4(mut self, arg4: &str) -> Self {
        self.arg4 = Some(arg4.to_string());
        self
    }

    /// Clears the `arg1` field.
    pub fn clear_arg1(mut self) -> Self {
        self.arg1 = None;
        self
    }

    /// Clears the `arg2` field.
    pub fn clear_arg2(mut self) -> Self {
        self.arg2 = None;
        self
    }

    /// Clears the `arg3` field.
    pub fn clear_arg3(mut self) -> Self {
        self.arg3 = None;
        self
    }

    /// Clears the `arg4` field.
    pub fn clear_arg4(mut self) -> Self {
        self.arg4 = None;
        self
    }

    /// Clears the `label1` field.
    pub fn clear_label1(mut self) -> Self {
        self.label1 = None;
        self
    }

    /// Clears the `label2` field.
    pub fn clear_label2(mut self) -> Self {
        self.label2 = None;
        self
    }

    /// Clears the `label3` field.
    pub fn clear_label3(mut self) -> Self {
        self.label3 = None;
        self
    }

    /// Clears the `label4` field.
    pub fn clear_label4(mut self) -> Self {
        self.label4 = None;
        self
    }

    /// Clears the `name` field.
    pub fn clear_name(mut self) -> Self {
        self.name = None;
        self
    }

    /// Clears the `secured1` field.
    pub fn clear_secured1(mut self) -> Self {
        self.secured1 = None;
        self
    }

    /// Clears the `secured2` field.
    pub fn clear_secured2(mut self) -> Self {
        self.secured2 = None;
        self
    }

    /// Clears the `secured3` field.
    pub fn clear_secured3(mut self) -> Self {
        self.secured3 = None;
        self
    }

    /// Clears the `secured4` field.
    pub fn clear_secured4(mut self) -> Self {
        self.secured4 = None;
        self
    }

    /// Clears the `value` field.
    pub fn clear_value(mut self) -> Self {
        self.value = None;
        self
    }

    /// Sets the `label1` field.
    ///
    /// # Arguments
    /// * `label1` - A label for `arg1`.
    pub fn label1(mut self, label1: &str) -> Self {
        self.label1 = Some(label1.to_string());
        self
    }

    /// Sets the `label2` field.
    ///
    /// # Arguments
    /// * `label2` - A label for `arg2`.
    pub fn label2(mut self, label2: &str) -> Self {
        self.label2 = Some(label2.to_string());
        self
    }

    /// Sets the `label3` field.
    ///
    /// # Arguments
    /// * `label3` - A label for `arg3`.
    pub fn label3(mut self, label3: &str) -> Self {
        self.label3 = Some(label3.to_string());
        self
    }

    /// Sets the `label4` field.
    ///
    /// # Arguments
    /// * `label4` - A label for `arg4`.
    pub fn label4(mut self, label4: &str) -> Self {
        self.label4 = Some(label4.to_string());
        self
    }

    /// Sets the `secured1` field.
    ///
    /// # Arguments
    /// * `secured1` - A boolean indicating whether or not `arg1` should be secured.
    pub fn secured1(mut self, secured1: bool) -> Self {
        self.secured1 = Some(secured1);
        self
    }

    /// Sets the `secured2` field.
    ///
    /// # Arguments
    /// * `secured2` - A boolean indicating whether or not `arg2` should be secured.
    pub fn secured2(mut self, secured2: bool) -> Self {
        self.secured1 = Some(secured2);
        self
    }

    /// Sets the `secured3` field.
    ///
    /// # Arguments
    /// * `secured3` - A boolean indicating whether or not `arg3` should be secured.
    pub fn secured3(mut self, secured3: bool) -> Self {
        self.secured3 = Some(secured3);
        self
    }

    /// Sets the `secured4` field.
    ///
    /// # Arguments
    /// * `secured4` - A boolean indicating whether or not `arg4` should be secured.
    pub fn secured4(mut self, secured4: bool) -> Self {
        self.secured4 = Some(secured4);
        self
    }

    /// Sets the `value` field.
    ///
    /// # Arguments
    /// * `value` - The value for the `Variable`, acting as a default value.
    pub fn value(mut self, value: &str) -> Self {
        self.value = Some(value.to_string());
        self
    }
}

// Check for arg and label consistency
fn check_arg_label(
    arg: &Option<String>,
    label: &Option<String>,
    arg_num: u8,
) -> Result<(), OpsviewConfigError> {
    if let Some(arg_val) = arg {
        validate_arg_string(arg_val)?;
    }

    if let Some(label_val) = label {
        validate_and_trim_label_string(label_val)?;
    }

    if let Some(arg_val) = arg {
        if !arg_val.is_empty() && (label.is_none() || label.as_ref().unwrap().is_empty()) {
            return Err(OpsviewConfigError::MissingArgLabel(arg_num));
        }
    }
    Ok(())
}

// Check for secured, arg, and label consistency
fn check_secured_arg_label(
    secured: &Option<bool>,
    arg: &Option<String>,
    label: &Option<String>,
    arg_num: u8,
) -> Result<(), OpsviewConfigError> {
    if matches!(secured, Some(false) | None) {
        return Ok(());
    }

    if matches!(arg.as_deref(), None | Some("")) {
        return Err(OpsviewConfigError::MissingSecuredArg(arg_num));
    }

    if matches!(label.as_deref(), None | Some("")) {
        return Err(OpsviewConfigError::MissingSecuredLabel(arg_num));
    }

    Ok(())
}

/// A reference version of [`Variable`] that is used when passing or retrieving a [`Variable`]
/// object as part of another non-[`super::Host`] object. For use with a [`super::Host`] object, see
/// [`super::HostVariableRef`].
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct VariableRef {
    name: String,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
}

/// Enables the creation of a [`VariableRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for VariableRef {}

impl ConfigRef for VariableRef {
    type FullObject = Variable;

    /// Returns the reference string of the [`VariableRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`VariableRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`VariableRef`] object.
    ///
    /// This name is used to identify the `VariableRef` when building the `HashMap` for an
    /// [`ConfigRefMap`].
    ///
    /// # Returns
    /// A string representing the unique name of the `VariableRef`.
    fn unique_name(&self) -> String {
        match &self.ref_ {
            Some(ref_) => ref_.clone(),
            None => self.name.clone(),
        }
    }
}

impl From<Variable> for VariableRef {
    /// Returns a [`VariableRef`] object from a [`Variable`] object.
    ///
    /// # Arguments
    /// * `full_object` - A [`Variable`] object.
    ///
    /// # Returns
    /// A [`VariableRef`] object.
    fn from(full_object: Variable) -> Self {
        Self {
            name: full_object.name.clone(),
            ref_: full_object.ref_.clone(),
        }
    }
}

impl From<Arc<Variable>> for VariableRef {
    fn from(item: Arc<Variable>) -> Self {
        let var: Variable = Arc::try_unwrap(item).unwrap_or_else(|arc| (*arc).clone());
        VariableRef::from(var)
    }
}

impl From<&ConfigObjectMap<Variable>> for ConfigRefMap<VariableRef> {
    fn from(variables: &ConfigObjectMap<Variable>) -> Self {
        ref_map_from(variables)
    }
}

/// A reference version of [`Variable`] that is used when passing or retrieving a [`Variable`]
/// object as part of another non-[`super::Host`] object. For use with a [`super::Host`] object, see
/// [`super::HostVariableRef`].
///
/// This variant is used by [`super::NotificationMethod`] and [`super::Contact`].
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct VariableValueRef {
    name: String,
    value: String,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
}

/// Enables the creation of a [`VariableValueRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for VariableValueRef {}

impl ConfigRef for VariableValueRef {
    type FullObject = Variable;

    /// Returns the reference string of the [`VariableValueRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`VariableValueRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`VariableValueRef`] object.
    ///
    /// This name is used to identify the `VariableValueRef` when building the `HashMap` for an
    /// [`ConfigRefMap`].
    ///
    /// # Returns
    /// A string representing the unique name of the `VariableValueRef`.
    fn unique_name(&self) -> String {
        match &self.ref_ {
            Some(ref_) => ref_.clone(),
            None => self.name.clone(),
        }
    }
}

impl From<Variable> for VariableValueRef {
    /// Returns a [`VariableValueRef`] object from a [`Variable`] object.
    ///
    /// # Arguments
    /// * `full_object` - A [`Variable`] object.
    ///
    /// # Returns
    /// A [`VariableValueRef`] object.
    fn from(full_object: Variable) -> Self {
        Self {
            name: full_object.name.clone(),
            value: match &full_object.value {
                Some(val) => val.clone(),
                None => "".to_string(),
            },
            ref_: full_object.ref_.clone(),
        }
    }
}

impl From<Arc<Variable>> for VariableValueRef {
    fn from(item: Arc<Variable>) -> Self {
        let var: Variable = Arc::try_unwrap(item).unwrap_or_else(|arc| (*arc).clone());
        VariableValueRef::from(var)
    }
}

impl From<&ConfigObjectMap<Variable>> for ConfigRefMap<VariableValueRef> {
    fn from(variables: &ConfigObjectMap<Variable>) -> Self {
        ref_map_from(variables)
    }
}

impl VariableValueRef {
    /// Returns the value of the `VariableValueRef`.
    pub fn value(&self) -> String {
        self.value.clone()
    }
}

/// A reference version of [`Variable`] that is used when passing or retrieving a
/// [`Variable`] object as part of a [`super::Host`] object.
#[allow(missing_docs)]
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct HostVariableRef {
    pub name: String,
    #[serde(
        rename = "ref",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_readonly",
        default
    )]
    ref_: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub arg1: Option<String>,
    pub arg2: Option<String>,
    pub arg3: Option<String>,
    pub arg4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted_arg1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted_arg2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted_arg3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted_arg4: Option<String>,
}

/// Enables the creation of a [`HostVariableRef`] instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for HostVariableRef {}

impl ConfigRef for HostVariableRef {
    type FullObject = Variable;

    /// Returns the reference string of the [`HostVariableRef`] object.
    fn ref_(&self) -> Option<String> {
        self.ref_.clone()
    }

    /// Returns the name of the [`HostVariableRef`] object.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the unique name of the [`HostVariableRef`] object.
    ///
    /// This name is used to identify the `HostVariableRef` when building the `HashMap` for an
    /// [`ConfigRefMap`].
    ///
    /// # Returns
    /// A string representing the unique name of the `HostVariableRef`.
    fn unique_name(&self) -> String {
        match self.id {
            Some(id) => format!("{}_{}", self.name, id),
            None => self.name.clone(),
        }
    }
}

impl From<Variable> for HostVariableRef {
    fn from(full_object: Variable) -> Self {
        Self {
            name: full_object.name.clone(),
            ref_: full_object.ref_.clone(),
            id: full_object.id,
            value: full_object.value.clone(),
            arg1: match full_object.secured1 {
                Some(false) => full_object.arg1.clone(),
                _ => None,
            },
            arg2: match full_object.secured2 {
                Some(false) => full_object.arg2.clone(),
                _ => None,
            },
            arg3: match full_object.secured3 {
                Some(false) => full_object.arg3.clone(),
                _ => None,
            },
            arg4: match full_object.secured4 {
                Some(false) => full_object.arg4.clone(),
                _ => None,
            },
            encrypted_arg1: match full_object.secured1 {
                Some(true) => full_object.arg1.clone(),
                _ => None,
            },
            encrypted_arg2: match full_object.secured2 {
                Some(true) => full_object.arg2.clone(),
                _ => None,
            },
            encrypted_arg3: match full_object.secured3 {
                Some(true) => full_object.arg3.clone(),
                _ => None,
            },
            encrypted_arg4: match full_object.secured4 {
                Some(true) => full_object.arg4.clone(),
                _ => None,
            },
        }
    }
}

impl From<Arc<Variable>> for HostVariableRef {
    fn from(item: Arc<Variable>) -> Self {
        let var: Variable = Arc::try_unwrap(item).unwrap_or_else(|arc| (*arc).clone());
        HostVariableRef::from(var)
    }
}

impl From<Arc<HostVariableRef>> for HostVariableRef {
    fn from(item: Arc<HostVariableRef>) -> Self {
        Arc::try_unwrap(item).unwrap_or_else(|arc| (*arc).clone())
    }
}

impl From<&ConfigObjectMap<Variable>> for ConfigRefMap<HostVariableRef> {
    fn from(variables: &ConfigObjectMap<Variable>) -> Self {
        ref_map_from(variables)
    }
}

impl HostVariableRef {
    /// Returns the unique identifier of the `HostVariableRef`.
    pub fn id(&self) -> Option<u64> {
        self.id
    }
}

impl ConfigRefMap<HostVariableRef> {
    /// Check if the map contains at least 1 `HostVariableRef` with the given name in the `name`
    /// field.
    ///
    /// # Arguments
    /// * `name` - The name of the variable to check for.
    ///
    /// # Returns
    /// A boolean indicating whether or not the map contains at least 1 `HostVariableRef` with the
    /// given name in the `name` field.
    pub fn contains_name(&self, name: &str) -> bool {
        self.values().filter(|v| v.name == name).count() > 0
    }

    /// Adds a `HostVariableRef` to the map using a given name instead of the `unique_name` method.
    pub fn add_named(&mut self, name: &str, object: HostVariableRef) {
        self.objects.insert(name.to_string(), Arc::new(object));
    }

    /// Removes all `HostVariableRef` where the name field of the object matches the given name.
    ///
    /// # Arguments
    /// * `name` - The name of the variable or variables to remove.
    ///
    /// # Returns
    /// An `Option<ConfigRefMap<HostVariableRef>>` representing the variables if found, or `None`
    /// if not found.
    pub fn remove_named(&mut self, name: &str) -> Option<ConfigRefMap<HostVariableRef>> {
        let mut removed = ConfigRefMap::<HostVariableRef>::new();
        let mut keys_to_remove = Vec::new();

        for (key, value) in self.iter() {
            if value.name() == name {
                removed.add_named(name, value.clone().into());
                keys_to_remove.push(key.clone());
            }
        }

        for key in keys_to_remove {
            self.remove(&key);
        }

        if removed.is_empty() {
            None
        } else {
            Some(removed)
        }
    }

    /// Returns a `Vec` of `HostVariableRef` objects where the name field of the object matches the
    /// given name.
    pub fn filter_by_name(&self, name: &str) -> Option<Vec<HostVariableRef>> {
        let mut matches = Vec::new();

        for v in self.values().filter(|v| v.name == name) {
            matches.push(v.clone().into());
        }

        if matches.is_empty() {
            None
        } else {
            Some(matches)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let variable = Variable::default();

        assert!(variable.name.is_empty());
    }

    #[test]
    fn test_minimal() {
        let variable = Variable::minimal("MY_VARIABLE");

        if variable.is_err() {
            println!("variable: {:#?}", variable);
        }

        assert_eq!(variable.unwrap().name, "MY_VARIABLE".to_string());
    }

    #[test]
    fn test_build_with_only_name() {
        let variable = Variable::builder().name("MY_VARIABLE").build();

        if variable.is_err() {
            println!("variable: {:#?}", variable);
        }

        assert!(variable.is_ok());
    }

    #[test]
    fn test_build_with_valid_data() {
        let builder = VariableBuilder::new()
            .name("VALID_VARIABLE")
            .arg1("arg1_value")
            .label1("label1_value")
            .secured1(true)
            .arg2("arg2_value")
            .label2("label2_value")
            .secured2(false);

        assert!(builder.build().is_ok());
    }

    #[test]
    fn test_build_fails_without_name() {
        let builder = VariableBuilder::new();
        assert!(builder.build().is_err());
    }

    #[test]
    fn test_build_fails_with_arg_but_no_label() {
        let builder = VariableBuilder::new().name("VARIABLE").arg1("arg1_value");

        assert!(builder.build().is_err());
    }

    #[test]
    fn test_build_fails_with_empty_label_for_non_empty_arg() {
        let builder = VariableBuilder::new()
            .name("VARIABLE")
            .arg1("arg1_value")
            .label1("");

        assert!(builder.build().is_err());
    }

    #[test]
    fn test_build_fails_with_secured_but_empty_arg_and_label() {
        let builder = VariableBuilder::new().name("VARIABLE").secured1(true);

        assert!(builder.build().is_err());
    }

    #[test]
    fn test_build_fails_with_secured_but_no_arg() {
        let builder = VariableBuilder::new()
            .name("VARIABLE")
            .label1("label1_value")
            .secured1(true);

        assert!(builder.build().is_err());
    }

    #[test]
    fn test_build_fails_with_secured_but_empty_label() {
        let builder = VariableBuilder::new()
            .name("VARIABLE")
            .arg1("arg1_value")
            .secured1(true);

        assert!(builder.build().is_err());
    }
}
