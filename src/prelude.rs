#![allow(missing_docs)]
use crate::client::OpsviewClient;
pub use crate::error::*;
pub use crate::state::*;
use serde::de::{self, DeserializeOwned};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::sync::Arc;

/// A map of objects implementing the `ConfigObject` trait.
///
/// This struct provides a container for storing and managing a collection of objects, each of which
/// implements the `ConfigObject` trait. It uses a `HashMap` to maintain a mapping between object
/// names and the objects themselves, facilitating efficient retrieval and management.
///
/// # Type Parameters
/// * `T` - The type of the objects stored in the map. Must implement the `ConfigObject` and `Clone` traits.
///
/// # Fields
/// * `objects` - A `HashMap` where keys are object names and values are `Arc<T>`.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConfigObjectMap<T: ConfigObject> {
    objects: HashMap<String, Arc<T>>,
}

/// A collection of objects implementing the `ConfigRef` trait.
///
/// This struct provides a container for storing and managing a collection of objects, each of which
/// implements the `ConfigRef` trait. It uses a `HashMap` to maintain a mapping between object
/// names and the objects themselves, facilitating efficient retrieval and management.
///
/// # Type Parameters
/// * `T` - The type of the objects stored in the collection. Must implement the `ConfigRef` and `Clone` traits.
///
/// # Fields
/// * `objects` - A `HashMap` where keys are object names and values are `Arc<T>` references to the objects.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConfigRefMap<T: ConfigRef> {
    pub objects: HashMap<String, Arc<T>>,
}

/// Trait for creating objects from a JSON string.
///
/// This trait defines a method for deserializing a JSON string into an instance of a type that
/// implements it. It is intended for use with types that represent entities in the Opsview system
/// and are typically parsed from JSON responses.
pub trait CreateFromJson: DeserializeOwned {
    /// Deserializes a JSON string into an instance of the implementing type.
    ///
    /// # Arguments
    /// * `json` - A string slice containing the JSON representation of the object.
    ///
    /// # Returns
    /// A `Result` containing either the deserialized object or an error if deserialization fails.
    fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Trait defining common behavior for Opsview builders.
///
/// This trait is implemented by types that are used to construct entities in the Opsview system.
/// It provides a method for creating a new instance of the builder, which is typically used to
/// construct an object using the builder pattern.
pub trait Builder {
    type ConfigObject;

    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError>
    where
        Self: Sized;

    /// Creates a new instance of the builder.
    fn new() -> Self
    where
        Self: Sized;

    fn name(self, name: &str) -> Self
    where
        Self: Sized;
}

/// Trait defining common behavior for Opsview objects.
///
/// This trait is implemented by types representing entities within the Opsview system. It provides
/// methods for retrieving configuration paths and unique names, which are essential for object
/// identification and management in Opsview.
pub trait ConfigObject:
    Clone
    + Debug
    + Default
    + for<'a> Deserialize<'a>
    + DeserializeOwned
    + Eq
    + PartialEq
    + Serialize
    + Sized
{
    type Builder: Builder<ConfigObject = Self>;

    /// Retrieves the API configuration path for an object of this type.
    ///
    /// # Returns
    /// An `Option<String>` representing the API path where the object is configured, or `None` if
    /// not applicable.
    fn config_path() -> Option<String> {
        None
    }

    // fn optional_columns() -> Option<Vec<String>> {
    //     None
    // }

    /// Retrieves the unique name of the object.
    ///
    /// # Returns
    /// A `String` representing the unique name of the object to use when creating the `HashMap`
    /// in an `ConfigObjectMap`.
    fn unique_name(&self) -> String;

    /// Creates a new instance of the builder for the object.
    fn builder() -> Self::Builder;

    fn minimal(name: &str) -> Result<Self, OpsviewConfigError> {
        Self::builder().name(name).build()
    }
}

/// Trait defining common behavior for Opsview objects that can be represented as a thin
/// version of itself when used by other ConfigObjects.
pub trait ConfigRef:
    Clone
    + Debug
    + Default
    + for<'a> Deserialize<'a>
    + DeserializeOwned
    + Eq
    + PartialEq
    + Serialize
    + Sized
{
    type FullObject: Persistent;
    fn name(&self) -> String;
    fn ref_(&self) -> Option<String>;
    fn unique_name(&self) -> String;
}

/// Trait defining common behavior for Opsview `Objects` that on which CRUD-operations can be
/// performed via the API.
#[allow(async_fn_in_trait)]
pub trait Persistent: ConfigObject {
    fn id(&self) -> Option<u64>;
    fn ref_(&self) -> Option<String>;
    fn name(&self) -> Option<String>;
    fn clear_readonly(&mut self);
    fn name_regex(&self) -> Option<String>;
    fn validated_name(&self, name: &str) -> Result<String, OpsviewConfigError>;
    fn set_name(&mut self, new_name: &str) -> Result<String, OpsviewConfigError>;

    /// Clones an existing object, but with a new name and with read-only fields cleared.
    fn clone_new_name(original: &Self, new_name: &str) -> Result<Self, OpsviewConfigError> {
        let mut cloned_obj = original.clone();
        cloned_obj.clear_readonly();
        cloned_obj.set_name(new_name)?;

        Ok(cloned_obj)
    }

    /// Checks whether the object exists in the Opsview Server.
    ///
    /// # Arguments
    /// * `client` - The [`OpsviewClient`] used to make the API request.
    ///
    /// # Returns
    /// A `Result` containing a boolean indicating whether the object exists or an error if the
    /// object could not be checked.
    ///
    /// # Example
    /// ```rust
    /// use opsview::client::OpsviewClient;
    /// use opsview::config::Hashtag;
    /// use opsview::prelude::*;
    ///
    /// async fn check_if_hashtag_exists(client: &OpsviewClient) -> Result<bool, OpsviewClientError> {
    ///    let hashtag = Hashtag{
    ///         name: "foo".to_string(),
    ///         ..Default::default()
    ///    };
    ///
    ///    // Will lookup the hashtag by name, since it doesn't have an id.
    ///    let exists = hashtag.exists(client).await?;
    ///
    ///    Ok(exists)
    /// }
    ///
    /// async fn check_if_hashtag_exists_by_id(client: &OpsviewClient) -> Result<bool, OpsviewClientError> {
    ///   let hashtag = Hashtag{
    ///      id: Some(1234),
    ///      ..Default::default()
    ///      };
    ///
    ///   // Will lookup the hashtag by id, since it has an id.
    ///   let exists = hashtag.exists(client).await?;
    ///
    ///   Ok(exists)
    /// }
    /// ```
    async fn exists(&self, client: &OpsviewClient) -> Result<bool, OpsviewClientError> {
        client.object_exists::<Self>(self).await
    }

    /// Retrieves the object from the Opsview Server and returns it.
    ///
    /// # Arguments
    /// * `client` - The [`OpsviewClient`] used to make the API request.
    ///
    /// # Returns
    /// A `Result` containing the retrieved object or an error if the object could not be
    /// retrieved.
    ///
    /// # Example
    /// This could be used to retrieve the already existing object from the Opsview API for the
    /// purpose of comparing it to a local object.
    /// ```rust
    /// use opsview::client::OpsviewClient;
    /// use opsview::config::Hashtag;
    /// use opsview::prelude::*;
    ///
    /// async fn get_hashtag(client: &OpsviewClient) -> Result<Hashtag, OpsviewError> {
    ///     let hashtag = Hashtag::builder()
    ///         .name("opsview")
    ///         .build()?;
    ///
    ///     let api_returned_hashtag = hashtag.fetch(client).await?;
    ///
    ///     assert_eq!(api_returned_hashtag.name, hashtag.name);
    ///     assert!(api_returned_hashtag.id.is_some());
    ///
    ///     Ok(api_returned_hashtag)
    /// }
    /// ```
    ///
    /// It could also be used to fetch the object from the Opsview API and replace the local object
    /// with the fetched object, effectively synchronizing the local object with the one in the
    /// Opsview API.
    /// ```rust
    /// use opsview::client::OpsviewClient;
    /// use opsview::config::Hashtag;
    /// use opsview::prelude::*;
    ///
    /// async fn sync_hashtag_by_name_if_existing(
    /// client: &OpsviewClient,
    /// name: &str
    /// ) -> Result<Hashtag, OpsviewError> {
    ///    let hashtag = Hashtag::builder()
    ///     .name(name)
    ///     .build()?;
    ///
    ///     assert!(hashtag.id.is_none()); // The builder doesn't set the id field.
    ///
    ///     let hashtag = hashtag.fetch(client).await?; // This will fail if the hashtag doesn't
    ///                                                 // exist in the Opsview API.
    ///
    ///     assert!(hashtag.id.is_some()); // The id field is set after the fetch, since the object
    ///                                    // that exists in the Opsview API has an id.
    ///                                    // The local object is now a copy of the object in the
    ///                                    // Opsview API and thus has the same id.
    ///
    ///     Ok(hashtag)
    /// }
    /// ```
    async fn fetch(&self, client: &OpsviewClient) -> Result<Self, OpsviewClientError> {
        client.get_object_config::<Self>(self).await
    }

    /// Removes the object from the Opsview Server.
    ///
    /// # Arguments
    /// * `client` - The [`OpsviewClient`] used to make the API request.
    ///
    /// # Returns
    /// A `Result` containing the response from the Opsview API or an error if the object could not
    /// be removed.
    ///
    /// # Example
    /// ```rust
    /// use opsview::client::OpsviewClient;
    /// use opsview::config::Hashtag;
    /// use opsview::prelude::*;
    ///
    /// async fn remove_hashtag_by_name_if_existing(
    /// client: &OpsviewClient,
    /// name: &str
    /// ) -> Result<(), OpsviewError> {
    ///    let hashtag = Hashtag::builder()
    ///        .name(name)
    ///        .build()?;
    ///
    ///    let response = hashtag.remove(client).await?;
    ///
    ///    assert_eq!(response["status"], "ok");
    ///
    ///    Ok(())
    /// }
    /// ```
    async fn remove(&self, client: &OpsviewClient) -> Result<Value, OpsviewClientError> {
        client.delete_object_config::<Self>(self).await
    }

    /// Creates the object on the Opsview Server if it doesn't already exist.
    async fn create(&self, client: &OpsviewClient) -> Result<Value, OpsviewClientError> {
        client.post_new_object_config::<Self>(self).await
    }

    /// Updates the object on the Opsview Server if it exists, creates it if it doesn't.
    async fn update(&self, client: &OpsviewClient) -> Result<Value, OpsviewClientError> {
        client.put_object_config::<Self>(self).await
    }
}

#[allow(async_fn_in_trait)]
pub trait PersistentMap: Serialize + Sized {
    fn config_path() -> Option<String> {
        None
    }

    async fn create_all(&self, client: &OpsviewClient) -> Result<Value, OpsviewClientError> {
        client.post_new_object_config_map::<Self>(self).await
    }
}

impl<T: ConfigObject> ConfigObjectMap<T> {
    /// Creates a new instance of `ConfigObjectMap` with default values.
    pub fn new() -> Self {
        ConfigObjectMap {
            objects: HashMap::new(),
        }
    }

    /// Adds an object to the collection using the object's unique name as the key.
    /// The unique name is retrieved by calling the `unique_name()` method on the object.
    pub fn add(&mut self, object: T) {
        self.objects.insert(object.unique_name(), Arc::new(object));
    }

    /// Adds a reference to an object to the collection using the object's unique name as the key.
    /// This method is used to add objects that are already wrapped in `Arc<T>`.
    ///
    /// # Arguments
    /// * `object` - An `Arc<T>` reference to the object to add.
    pub fn add_ref(&mut self, object: Arc<T>) {
        self.objects.insert(object.unique_name(), object);
    }

    /// Creates a representation of the collection as a `Vec` of `Arc<T>`.
    pub fn as_vec(&self) -> Vec<Arc<T>> {
        self.objects.values().cloned().collect()
    }

    /// Checks whether the collection contains an object with the specified name.
    ///
    /// # Arguments
    /// * `key` - The name of the object to check for.
    ///
    /// # Returns
    /// A boolean indicating whether the collection contains an object with the specified name.
    pub fn contains(&self, key: &str) -> bool {
        self.objects.contains_key(key)
    }

    /// Retrieves an optional reference to an object with the specified name.
    ///
    /// # Arguments
    /// * `key` - The name of the object to retrieve.
    ///
    /// # Returns
    /// An `Option<Arc<T>>` representing the object if found, or `None` if not found.
    pub fn get(&self, key: &str) -> Option<Arc<T>> {
        self.objects.get(key).cloned()
    }

    /// Checks whether the collection is empty.
    ///
    /// # Returns
    /// A boolean indicating whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    /// Retrieves an iterator over references to the objects in the collection.
    ///
    /// # Returns
    /// An iterator over items in the collection in arbitrary order.
    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Arc<T>> {
        self.objects.iter()
    }

    /// Retrieves an iterator over the keys in the collection.
    ///
    /// # Returns
    /// An iterator over the keys in the collection in arbitrary order.
    pub fn keys(&self) -> std::collections::hash_map::Keys<String, Arc<T>> {
        self.objects.keys()
    }

    /// Calculates the number of objects in the collection.
    ///
    /// # Returns
    /// The number of objects in the collection as a `usize`.
    pub fn len(&self) -> usize {
        self.objects.len()
    }

    /// Removes an object from the collection.
    ///
    /// # Arguments
    /// * `name` - The name of the object to remove.
    ///
    /// # Returns
    /// An `Option<Arc<T>>` representing the object if found, or `None` if not found.
    pub fn remove(&mut self, name: &str) -> Option<Arc<T>> {
        self.objects.remove(name)
    }

    /// Retrieves an iterator over references to the objects in the collection.
    ///
    /// # Returns
    /// An iterator over references to the objects in the collection.
    pub fn values(&self) -> impl Iterator<Item = &Arc<T>> {
        self.objects.values()
    }

    pub fn drain(&mut self) -> std::collections::hash_map::Drain<String, Arc<T>> {
        self.objects.drain()
    }

    pub fn extend(&mut self, other: &mut Self) {
        self.objects.extend(other.drain());
    }
}

impl<T: ConfigObject> Serialize for ConfigObjectMap<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let objects: Vec<&T> = self.objects.values().map(|arc_obj| &**arc_obj).collect();
        objects.serialize(serializer)
    }
}

impl<'de, T: ConfigObject> Deserialize<'de> for ConfigObjectMap<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec: Vec<T> = Vec::deserialize(deserializer)?;
        let mut objects = HashMap::new();

        for obj in vec {
            let name = obj.unique_name().clone();
            if objects.contains_key(&name) {
                return Err(serde::de::Error::custom(format!(
                    "Duplicate name detected: {}",
                    name
                )));
            }
            objects.insert(name, Arc::new(obj));
        }

        Ok(ConfigObjectMap { objects })
    }
}

impl<T: ConfigRef> ConfigRefMap<T> {
    /// Creates a new instance of `ConfigRefMap` with default values.
    pub fn new() -> Self {
        ConfigRefMap {
            objects: HashMap::new(),
        }
    }

    /// Adds an object to the collection using the object's unique name as the key.
    /// The unique name is retrieved by calling the `unique_name()` method on the object.
    pub fn add(&mut self, object: T) {
        self.objects.insert(object.unique_name(), Arc::new(object));
    }

    /// Adds a reference to an object to the collection using the object's unique name as the key.
    /// This method is used to add objects that are already wrapped in `Arc<T>`.
    ///
    /// # Arguments
    /// * `object` - An `Arc<T>` reference to the object to add.
    pub fn add_ref(&mut self, object: Arc<T>) {
        self.objects.insert(object.unique_name(), object);
    }

    /// Creates a representation of the collection as a `Vec` of `Arc<T>`.
    pub fn as_vec(&self) -> Vec<Arc<T>> {
        self.objects.values().cloned().collect()
    }

    /// Checks whether the collection contains an object with the specified name.
    ///
    /// # Arguments
    /// * `key` - The name of the object to check for.
    ///
    /// # Returns
    /// A boolean indicating whether the collection contains an object with the specified name.
    pub fn contains(&self, key: &str) -> bool {
        self.objects.contains_key(key)
    }

    /// Retrieves an optional reference to an object with the specified name.
    ///
    /// # Arguments
    /// * `key` - The name of the object to retrieve.
    ///
    /// # Returns
    /// An `Option<Arc<T>>` representing the object if found, or `None` if not found.
    pub fn get(&self, key: &str) -> Option<Arc<T>> {
        self.objects.get(key).cloned()
    }

    /// Checks whether the collection is empty.
    ///
    /// # Returns
    /// A boolean indicating whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    /// Retrieves an iterator over references to the objects in the collection.
    ///
    /// # Returns
    /// An iterator over items in the collection in arbitrary order.
    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Arc<T>> {
        self.objects.iter()
    }

    /// Retrieves an iterator over the keys in the collection.
    ///
    /// # Returns
    /// An iterator over the keys in the collection in arbitrary order.
    pub fn keys(&self) -> std::collections::hash_map::Keys<String, Arc<T>> {
        self.objects.keys()
    }

    /// Calculates the number of objects in the collection.
    ///
    /// # Returns
    /// The number of objects in the collection as a `usize`.
    pub fn len(&self) -> usize {
        self.objects.len()
    }

    /// Removes an object from the collection.
    ///
    /// # Arguments
    /// * `name` - The name of the object to remove.
    ///
    /// # Returns
    /// An `Option<Arc<T>>` representing the object if found, or `None` if not found.
    pub fn remove(&mut self, name: &str) -> Option<Arc<T>> {
        self.objects.remove(name)
    }

    /// Retrieves an iterator over references to the objects in the collection.
    ///
    /// # Returns
    /// An iterator over references to the objects in the collection.
    pub fn values(&self) -> impl Iterator<Item = &Arc<T>> {
        self.objects.values()
    }
}

impl<T: ConfigRef> Serialize for ConfigRefMap<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let objects: Vec<&T> = self.objects.values().map(|arc_obj| &**arc_obj).collect();
        objects.serialize(serializer)
    }
}

impl<'de, T: ConfigRef> Deserialize<'de> for ConfigRefMap<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec: Vec<T> = Vec::deserialize(deserializer)?;
        let mut objects = HashMap::new();

        for obj in vec {
            let name = obj.unique_name().clone();
            if objects.contains_key(&name) {
                return Err(serde::de::Error::custom(format!(
                    "Duplicate name detected: {}",
                    name
                )));
            }
            objects.insert(name, Arc::new(obj));
        }

        Ok(ConfigRefMap { objects })
    }
}

// Custom deserialization for readonly fields
pub fn deserialize_readonly<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    // Deserialize the field normally
    let value = Deserialize::deserialize(deserializer)?;
    Ok(Some(value))
}

/// Custom deserialization for fields that are represented as either strings or numbers in the JSON
/// data but are to be interpreted as unsigned integers.
pub fn deserialize_string_or_number_to_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(StringOrNumberVisitor)
}

struct StringOrNumberVisitor;

impl<'de> serde::de::Visitor<'de> for StringOrNumberVisitor {
    type Value = Option<u64>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string or a number, or null")
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_any(StringOrNumberValueVisitor)
            .map(Some)
    }

    fn visit_none<E>(self) -> Result<Option<u64>, E> {
        Ok(None)
    }
}

struct StringOrNumberValueVisitor;

impl<'de> serde::de::Visitor<'de> for StringOrNumberValueVisitor {
    type Value = u64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string or a number")
    }

    fn visit_u64<E>(self, value: u64) -> Result<u64, E> {
        Ok(value)
    }

    fn visit_str<E>(self, value: &str) -> Result<u64, E>
    where
        E: serde::de::Error,
    {
        value.parse().map_err(serde::de::Error::custom)
    }
}

/// Custom deserialization for fields that are represented as either strings or numbers in the JSON
/// data but are to be interpreted as boolean.
pub fn deserialize_string_or_number_to_option_bool<'de, D>(
    deserializer: D,
) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrNumberVisitor;

    impl<'de> serde::de::Visitor<'de> for StringOrNumberVisitor {
        type Value = Option<bool>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string or a number, either '0' or '1'")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match value {
                0 => Ok(Some(false)),
                1 => Ok(Some(true)),
                _ => Err(serde::de::Error::custom(format!(
                    "Invalid int value '{}' for boolean field",
                    value
                ))),
            }
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match value {
                "0" => Ok(Some(false)),
                "1" => Ok(Some(true)),
                "yes" => Ok(Some(true)),
                "no" => Ok(Some(false)),
                "true" => Ok(Some(true)),
                "false" => Ok(Some(false)),
                _ => Err(serde::de::Error::custom(format!(
                    "Invalid str value '{}' for boolean field",
                    value
                ))),
            }
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }

    deserializer.deserialize_any(StringOrNumberVisitor)
}

pub fn serialize_option_bool_as_string<S>(
    value: &Option<bool>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let str_value = match value {
        Some(true) => "1",
        Some(false) => "0",
        None => return serializer.serialize_none(),
    };
    serializer.serialize_str(str_value)
}

pub fn ref_map_from<T, U>(config_obj_map: &ConfigObjectMap<T>) -> ConfigRefMap<U>
where
    T: ConfigObject,
    U: ConfigRef<FullObject = T> + From<Arc<T>>,
{
    let mut ref_map = ConfigRefMap::new();
    for arc_object in config_obj_map.values() {
        let config_ref = U::from(Arc::clone(arc_object));
        ref_map.add(config_ref);
    }
    ref_map
}
