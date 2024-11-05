//! # Opsview Client
//! Contains the [`OpsviewClient`] struct and methods for interacting with the Opsview API.
use crate::{config::*, prelude::*};
use reqwest::{self, StatusCode};
use serde_json::{json, Value};
use url::Url;

/// A builder for constructing an `OpsviewClient` instance.
pub struct OpsviewClientBuilder {
    url: Option<String>,
    username: Option<String>,
    password: Option<String>,
    ignore_cert: bool,
}

/// Alias for a list of key-value pairs used as query parameters in HTTP requests.
pub type Params = Vec<(String, String)>;

impl OpsviewClientBuilder {
    /// Sets the URL of the Opsview API.
    ///
    /// # Arguments
    /// * `url` - The URL of the Opsview API.
    ///
    /// # Returns
    /// An `OpsviewClientBuilder` instance.
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    /// Sets the username for Opsview API authentication.
    ///
    /// # Arguments
    /// * `username` - The username for Opsview API authentication.
    ///
    /// # Returns
    /// An `OpsviewClientBuilder` instance.
    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_string());
        self
    }

    /// Sets the password for Opsview API authentication.
    ///
    /// # Arguments
    /// * `password` - The password for Opsview API authentication.
    ///
    /// # Returns
    /// An `OpsviewClientBuilder` instance.
    pub fn password(mut self, password: &str) -> Self {
        self.password = Some(password.to_string());
        self
    }

    /// Sets whether to ignore certificate errors.
    ///
    /// # Arguments
    /// * `ignore_cert` - Boolean indicating whether to ignore certificate errors.
    ///
    /// # Returns
    /// An `OpsviewClientBuilder` instance.
    pub fn ignore_cert(mut self, ignore_cert: bool) -> Self {
        self.ignore_cert = ignore_cert;
        self
    }

    /// Builds a new `OpsviewClient` instance.
    ///
    /// # Returns
    /// A `Result` wrapping an `OpsviewClient` instance upon successful authentication,
    pub async fn build(self) -> Result<OpsviewClient, OpsviewClientError> {
        OpsviewClient::new(
            self.url
                .as_deref()
                .ok_or(OpsviewClientError::MissingArgument("url".to_string()))?,
            self.username
                .as_deref()
                .ok_or(OpsviewClientError::MissingArgument("username".to_string()))?,
            self.password
                .as_deref()
                .ok_or(OpsviewClientError::MissingArgument("password".to_string()))?,
            self.ignore_cert,
        )
        .await
    }
}

/// A client for interacting with the [Opsview](https://www.itrsgroup.com/infrastructure-monitoring)
/// monitoring software
/// [API](https://docs.itrsgroup.com/docs/opsview/current/rest-api/rest-api-background/api-introduction/index.html).
///
/// This struct encapsulates the HTTP client ([`reqwest::Client`]) and the base URL used for making API requests.
/// It provides a range of methods to interact with various Opsview API endpoints, facilitating
/// operations like configuration management, object retrieval, and status updates.
///
/// [`reqwest::Client`]: https://docs.rs/reqwest/latest/reqwest/struct.Client.html
///
/// # Fields
/// * `client` - An instance of [`reqwest::Client`] used for making HTTP requests.
/// * `url` - The base URL of the Opsview API.
#[derive(Debug)]
pub struct OpsviewClient {
    client: reqwest::Client,
    url: String,
}

impl OpsviewClient {
    /// Creates a new `OpsviewClientBuilder` instance with default values.
    ///
    /// This method is used to initialize a new `OpsviewClientBuilder` instance with default
    /// values for the URL, username, and password. The default values are `None` for the URL,
    /// username, and password, and `false` for the `ignore_cert` field.
    ///
    /// # Returns
    /// An `OpsviewClientBuilder` instance with default values.
    ///
    /// # Examples
    /// ```
    /// use opsview::client::OpsviewClient;
    ///
    /// async fn example() {
    ///    let client = OpsviewClient::builder()
    ///        .url("api.example.com")
    ///        .username("user")
    ///        .password("pass")
    ///        .ignore_cert(true) // Indicates that we will ignore certificate errors
    ///        .build()
    ///        .await;
    ///
    ///    assert!(client.is_ok());
    /// }
    /// ```
    pub fn builder() -> OpsviewClientBuilder {
        OpsviewClientBuilder {
            url: None,
            username: None,
            password: None,
            ignore_cert: false,
        }
    }

    /// Constructs a new `OpsviewClient`.
    ///
    /// This method creates a new instance of `OpsviewClient` by authenticating with the [Opsview
    /// API](https://docs.itrsgroup.com/docs/opsview/current/rest-api/rest-api-background/api-introduction/index.html)
    /// using the provided credentials. It initializes the internal HTTP client with necessary
    /// headers and sets the base URL for API requests.
    ///
    /// End users are encouraged to use the [`OpsviewClient::builder()`] method to create a new
    /// `OpsviewClient` instance instead of invoking this method directly.
    ///
    /// The URL is automatically prefixed with "https://" if not already present.
    ///
    /// # Arguments
    /// * `url` - The base URL of the Opsview API.
    /// * `username` - The username for Opsview API authentication.
    /// * `password` - The password for Opsview API authentication.
    ///
    /// # Returns
    /// A `Result` which is either an `OpsviewClient` upon successful authentication or an
    /// error if authentication fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, authentication is unsuccessful, or if
    /// the Opsview API returns a non-200 status code.
    ///
    /// # Examples
    /// ```
    /// use opsview::client::OpsviewClient;
    ///
    /// async fn example() {
    ///     let client = OpsviewClient::new("api.example.com", "user", "pass", false).await;
    ///
    ///     assert!(client.is_ok());
    /// }
    /// ```
    pub async fn new(
        url: &str,
        username: &str,
        password: &str,
        ignore_cert: bool,
    ) -> Result<OpsviewClient, OpsviewClientError> {
        let url_with_https = if !url.starts_with("https://") && !url.starts_with("http://") {
            format!("https://{}", url)
        } else {
            url.to_string()
        };

        let client_builder = reqwest::Client::builder();

        let client_builder = if ignore_cert {
            client_builder.danger_accept_invalid_certs(true)
        } else {
            client_builder
        };

        let client = client_builder.build()?;

        let auth_response = client
            .post(format!("{}/rest/login", &url_with_https))
            .json(&json!({"username": &username, "password": &password}))
            .send()
            .await?;

        if auth_response.status() != 200 {
            let error_message = format!(
                "Failed to authenticate with status code: {}",
                auth_response.status()
            );
            return Err(OpsviewClientError::AuthError(error_message));
        }

        let token = auth_response
            .json::<Value>()
            .await?
            .get("token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                OpsviewClientError::AuthError("Token not found in response".to_string())
            })?
            .to_string();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-Opsview-Username", username.parse()?);
        headers.insert("X-Opsview-Token", token.parse()?);
        headers.insert("Content-Type", "application/json".parse()?);

        let client_builder = reqwest::Client::builder();
        let client_builder = if ignore_cert {
            client_builder.danger_accept_invalid_certs(true)
        } else {
            client_builder
        };

        let client = client_builder.default_headers(headers).build()?;

        Ok(OpsviewClient {
            client,
            url: url_with_https,
        })
    }

    // Fundamental operations --------------------------------------------------------------------//

    /// Sends a DELETE request to a specified path in the Opsview API.
    async fn delete(&self, path: &str) -> Result<Value, OpsviewClientError> {
        let url = Url::parse(&format!("{}/rest{}", self.url, path))?;
        handle_http_response(self.client.delete(url).send().await?).await
    }

    /// Performs a GET request to a specified path in the Opsview API and returns the response.
    ///
    /// This asynchronous method sends a GET request to the Opsview API and returns the response.
    ///
    /// # Arguments
    /// * `path` - The API path to be queried.
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the JSON response from the Opsview API.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the response cannot be parsed into a
    /// JSON object.
    async fn get(&self, path: &str, params: Option<Params>) -> Result<Value, OpsviewClientError> {
        let url = Url::parse(&format!("{}/rest{}", self.url, path))?;
        if let Some(params) = params {
            handle_http_response(self.client.get(url.as_ref()).query(&params).send().await?).await
        } else {
            handle_http_response(self.client.get(url.as_ref()).send().await?).await
        }
    }

    /// Sends a POST request to the Opsview API.
    ///
    /// This asynchronous method sends a POST request to the Opsview API.
    ///
    /// # Arguments
    /// * `path` - The API path to be queried.
    /// * `data` - The JSON data to be sent in the request body.
    ///
    /// # Returns
    /// A `Result` wrapping the JSON response from the Opsview API.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the response cannot be parsed into a
    /// JSON object.
    async fn post(&self, path: &str, data: &Value) -> Result<Value, OpsviewClientError> {
        let url = Url::parse(&format!("{}/rest{}", self.url, path))?;
        handle_http_response(self.client.post(url.as_ref()).json(data).send().await?).await
    }

    /// Sends a PUT request to the Opsview API.
    ///
    /// This asynchronous method sends a PUT request to the Opsview API.
    ///
    /// # Arguments
    /// * `path` - The API path to be queried.
    /// * `data` - The JSON data to be sent in the request body.
    ///
    /// # Returns
    /// A `Result` wrapping the JSON response from the Opsview API.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the response cannot be parsed into a
    /// JSON object.
    async fn put(&self, path: &str, data: &Value) -> Result<Value, OpsviewClientError> {
        let url = Url::parse(&format!("{}/rest{}", self.url, path))?;
        handle_http_response(self.client.put(url.as_ref()).json(data).send().await?).await
    }

    // Reload management -------------------------------------------------------------------------//
    // Methods related to applying and checking for pending configuration changes.

    /// Applies pending configuration changes in the Opsview system.
    ///
    /// This asynchronous method sends a request to the Opsview API to apply any pending
    /// configuration changes. It's typically used after making a series of configuration
    /// updates to ensure the changes take effect.
    ///
    /// # Returns
    /// A `Result` wrapping a `serde_json::Value` that contains the response from the Opsview API.
    /// The response includes details like the number of audit log entries, average duration,
    /// configuration status, last updated timestamp, messages, and server status.
    ///
    /// Example response:
    /// ```json
    /// {
    ///   "auditlog_entries": "0",
    ///   "average_duration": "10",
    ///   "configuration_status": "uptodate",
    ///   "lastupdated": "1702453777",
    ///   "messages": [],
    ///   "server_status": "0"
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if the Opsview API returns an error response.
    pub async fn apply_changes(&self) -> Result<Value, OpsviewClientError> {
        self.post("/reload", &Value::Null).await
    }

    /// Checks if there are pending configuration changes in the Opsview system.
    ///
    /// This asynchronous method queries the Opsview API to determine if there are any
    /// pending configuration changes that have not yet been applied. It can be used to
    /// check the status before initiating an apply operation.
    ///
    /// # Returns
    /// A `Result` wrapping a `bool` indicating whether there are changes to apply (`true`)
    /// or not (`false`), or an error if the check operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, the response does not contain the expected
    /// field, or if the Opsview API returns an unexpected response.
    ///
    /// # Examples
    /// ```
    /// use opsview::client::OpsviewClient;
    ///
    /// async fn example() {
    ///     let client = OpsviewClient::new("api.example.com", "user", "pass", false).await.unwrap();
    ///     let has_changes = client.changes_to_apply().await.unwrap();
    /// }
    /// ```
    pub async fn changes_to_apply(&self) -> Result<bool, OpsviewClientError> {
        let field = "configuration_status";
        let response = self.get("/reload", None).await?;
        let status = response
            .get(field)
            .and_then(|v| v.as_str())
            .ok_or(OpsviewClientError::FieldNotFound(field.to_string()))?;
        match status {
            "uptodate" => Ok(false),
            "pending" => Ok(true),
            _ => {
                let error_message = format!("Unexpected response from Opsview: '{}'", status);
                Err(OpsviewClientError::AuthError(error_message))
            }
        }
    }

    /// Checks when the configuration on the API server was last updated.
    ///
    /// # Returns
    /// A unix timestamp as a `u64` representing the last configuration update.
    pub async fn last_updated(&self) -> Result<u64, OpsviewClientError> {
        let field = "lastupdated";
        let response = self.get("/reload", None).await?;
        let last_updated_str = response
            .get(field)
            .and_then(|v| v.as_str())
            .ok_or(OpsviewClientError::FieldNotFound(field.to_string()))?;
        let last_updated = last_updated_str.parse::<u64>().map_err(|_| {
            OpsviewClientError::TypeParseError(last_updated_str.to_string(), "u64".to_string())
        })?;

        Ok(last_updated)
    }

    // Lookup operations -------------------------------------------------------------------------//

    /// Checks if a specific object exists in the Opsview system based on the object ID.
    async fn object_exists_by_id(&self, path: &str, id: u64) -> Result<bool, OpsviewClientError> {
        let full_path = format!("{}/exists?id={}", path, id);
        let response = self.get(&full_path, None).await?;

        interpret_exists_response(response)
    }

    /// Checks if a specific object exists in the Opsview system based on a key-value pair.
    async fn object_exists_by_key(
        &self,
        path: &str,
        key: &str,
        value: &str,
    ) -> Result<bool, OpsviewClientError> {
        let full_path = format!("{}/exists?{}={}", path, key, value);
        let response = self.get(&full_path, None).await?;

        interpret_exists_response(response)
    }

    /// Checks if a specific object exists in the Opsview system based on either the id, if present,
    /// or the name.
    ///
    /// # Arguments
    /// * `obj` - A reference to an object which implements the `Persistent` trait.
    ///
    /// # Returns
    /// A `Result` wrapping a `bool` indicating whether the object exists (`true`) or not (`false`),
    /// or an error if the check operation fails.
    pub async fn object_exists<T: Persistent>(&self, obj: &T) -> Result<bool, OpsviewClientError> {
        let path = T::config_path().ok_or(OpsviewClientError::NoConfigPath)?;

        match (obj.id(), obj.name()) {
            (Some(id), _) => self.object_exists_by_id(&path, id).await,
            (_, Some(name)) => self.object_exists_by_key(&path, "name", &name).await,
            _ => Err(OpsviewClientError::MissingIdentifiers(
                "Cannot check if object exists: neither id, nor name are set.".to_string(),
            )),
        }
    }

    /// Deletes a specific object configuration from the Opsview system based on a key-value pair.
    async fn delete_object_config_by_key<T: Persistent>(
        &self,
        key: &str,
        value: &str,
    ) -> Result<Value, OpsviewClientError> {
        let object_id = self.get_object_id_by_key::<T>(key, value, None).await?;
        self.delete_object_config_by_id::<T>(object_id).await
    }

    /// Deletes a specific object configuration from the Opsview system based on the object ID.
    async fn delete_object_config_by_id<T: Persistent>(
        &self,
        id: u64,
    ) -> Result<Value, OpsviewClientError> {
        let path = T::config_path().ok_or(OpsviewClientError::NoConfigPath)?;
        let full_path = format!("{}/{}", path, id);
        self.delete(&full_path).await
    }

    /// Deletes a specific object configuration from the Opsview system based on a reference.
    async fn delete_object_config_by_ref(&self, ref_: &str) -> Result<Value, OpsviewClientError> {
        let path = path_from_ref(ref_)?;
        let response = self.delete(&path).await?;

        Ok(response)
    }

    /// Deletes a specific object configuration from the Opsview system based on its reference,
    /// ID, or name.
    ///
    /// This asynchronous method sends a DELETE request to the Opsview API to remove the object
    /// configuration based on the provided reference, ID, or name. It is typically used to delete
    /// an object configuration from the Opsview system.
    ///
    /// # Arguments
    /// * `obj` - A reference to an object which implements the `Persistent` trait.
    ///   The object must have at least one of the following identifiers set: ref_, id, or name.
    ///
    ///  # Returns
    ///  A `Result` wrapping a `serde_json::Value` containing the response from the Opsview API,
    ///  or an error if the delete operation fails.
    ///
    ///  # Errors
    ///  Returns an error if the HTTP request fails, if the object ID cannot be retrieved,
    ///  or if the Opsview API returns an error response.
    pub async fn delete_object_config<T: Persistent>(
        &self,
        obj: &T,
    ) -> Result<Value, OpsviewClientError> {
        match (obj.id(), obj.ref_(), obj.name()) {
            (_, Some(ref_), _) => self.delete_object_config_by_ref(&ref_).await,
            (Some(id), _, _) => self.delete_object_config_by_id::<T>(id).await,
            (_, _, Some(name)) => self.delete_object_config_by_key::<T>("name", &name).await,
            _ => Err(OpsviewClientError::MissingIdentifiers(
                "Cannot delete object: neither ref_, id, nor name are set.".to_string(),
            )),
        }
    }

    /// Fetches a specific object configuration from the Opsview system based on a reference of
    /// an existing object.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve the object
    /// configuration based on the provided reference. It is typically used to retrieve an object
    /// configuration for further processing.
    ///
    /// # Arguments
    /// * `obj` - A reference to an object which implements the `Persistent` trait.
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use opsview::client::OpsviewClient;
    /// use opsview::config::Hashtag;
    /// use opsview::prelude::*;
    ///
    /// async fn example() -> Result<(), OpsviewClientError> {
    ///     let client = OpsviewClient::builder()
    ///        .url("api.example.com")
    ///        .username("user")
    ///        .password("pass")
    ///        .build()
    ///        .await?;
    ///
    ///    let hashtag = Hashtag::builder()
    ///       .name("exampleHashtag")
    ///       .build()
    ///       .unwrap();
    ///
    ///    let hashtag = client.get_object_config(&hashtag, None).await?;
    ///
    ///    assert!(hashtag.id.is_some());
    ///
    ///    Ok(())
    /// }
    /// ```     
    pub async fn get_object_config<T: Persistent>(
        &self,
        obj: &T,
        params: Option<Params>,
    ) -> Result<T, OpsviewClientError> {
        match (T::config_path(), obj.id(), obj.ref_(), obj.name()) {
            (_, _, Some(ref_), _) => self.get_object_config_by_ref::<T>(&ref_, params).await,
            (Some(_path), Some(id), _, _) => self.get_object_config_by_id::<T>(id, params).await,
            (Some(_path), _, _, Some(name)) => {
                self.get_object_config_by_key::<T>("name", &name, params)
                    .await
            }
            _ => Err(OpsviewClientError::MissingIdentifiers(
                "Cannot fetch object: neither ref_, id, nor name are set.".to_string(),
            )),
        }
    }

    /// Gets all BSM Component configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all BSM Component
    /// configuration objects. It is typically used to retrieve a list of BSM Components for further
    /// processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `BSMComponent` objects on the
    /// Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response into JSON fails.
    pub async fn get_all_bsmcomponent_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<BSMComponent>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<BSMComponent>(params)
            .await
    }

    /// Gets all BSM Service configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all BSM Service
    /// configuration objects. It is typically used to retrieve a list of BSM Services for further
    /// processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `BSMService` objects on the
    /// Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response into JSON fails.
    pub async fn get_all_bsmservice_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<BSMService>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<BSMService>(params)
            .await
    }

    /// Gets all contact configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all contact
    /// configuration objects. It is typically used to retrieve a list of contacts for further
    /// processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `Contact` objects on the
    /// Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response into JSON fails.
    pub async fn get_all_contact_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<Contact>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<Contact>(params).await
    }

    /// Gets all host configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all host
    /// configuration objects. It is typically used to retrieve a list of hosts for further
    /// processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `Host` objects on the
    /// Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response into JSON fails.
    pub async fn get_all_hashtag_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<Hashtag>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<Hashtag>(params).await
    }

    /// Gets all host configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all host
    /// configuration objects. It is typically used to retrieve a list of hosts for further
    /// processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `Host` objects on the
    /// Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response into JSON fails.
    pub async fn get_all_host_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<Host>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<Host>(params).await
    }

    /// Gets all host check command configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all host check
    /// command configuration objects. It is typically used to retrieve a list of host check commands
    /// for further processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `HostCheckCommand` objects on the
    /// Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response into JSON fails.
    pub async fn get_all_hostcheckcommand_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<HostCheckCommand>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<HostCheckCommand>(params)
            .await
    }

    /// Gets all host group configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all host group
    /// configuration objects. It is typically used to retrieve a list of host groups for further
    /// processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `HostGroup` objects on the
    /// Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response into JSON fails.
    pub async fn get_all_hostgroup_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<HostGroup>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<HostGroup>(params)
            .await
    }

    /// Gets all host icon configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all host icon
    /// configuration objects. It is typically used to retrieve a list of host icons for further
    /// processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `HostIcon` objects on the
    /// Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response into JSON fails.
    pub async fn get_all_hosticon_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<HostIcon>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<HostIcon>(params)
            .await
    }

    /// Gets all host template configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all host template
    /// configuration objects. It is typically used to retrieve a list of host templates for further
    /// processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `HostTemplate` objects on the
    /// Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response into JSON fails.
    pub async fn get_all_hosttemplate_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<HostTemplate>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<HostTemplate>(params)
            .await
    }

    /// Gets all monitoring cluster configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all monitoring
    /// cluster configuration objects. It is typically used to retrieve a list of monitoring clusters
    /// for further processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `MonitoringCluster` objects on the
    /// Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response into JSON fails.
    pub async fn get_all_monitoringcluster_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<MonitoringCluster>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<MonitoringCluster>(params)
            .await
    }

    /// Gets all netflow collector configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all netflow
    /// collector configuration objects. It is typically used to retrieve a list of netflow collectors
    /// for further processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `NetflowCollector` objects on the
    /// Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response fails.
    pub async fn get_all_netflowcollector_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<NetflowCollector>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<NetflowCollector>(params)
            .await
    }

    /// Gets all netflow source configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all netflow
    /// source configuration objects. It is typically used to retrieve a list of netflow sources
    /// for further processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `NetflowSource` objects on the
    /// Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing fails.
    pub async fn get_all_netflowsource_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<NetflowSource>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<NetflowSource>(params)
            .await
    }

    /// Gets all notification method configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all notification
    /// method configuration objects. It is typically used to retrieve a list of notification methods
    /// for further processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `NotificationMethod` objects on the
    /// Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response into JSON fails.
    pub async fn get_all_notificationmethod_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<NotificationMethod>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<NotificationMethod>(params)
            .await
    }

    /// Gets all object configuration objects of a specific type `T` from the Opsview API.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all object
    /// configuration objects at the `config_path` of the type `T`. It is typically used to retrieve
    /// a list of objects for further processing.
    ///
    /// This method is generic and can be used to retrieve any type of object configuration as long
    /// as the type implements the `ConfigObject` trait.
    ///
    /// # Arguments
    /// * `T` - The type of object to be retrieved.
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping a `ConfigObjectMap<T>` containing the objects on the Opsview system at
    /// the specified path.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response into JSON fails.
    async fn get_all_object_configs_by_type<T: ConfigObject>(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<T>, OpsviewClientError> {
        let path: String = T::config_path().ok_or(OpsviewClientError::NoConfigPath)?;
        let mut summary_totalrows: Option<u64> = None;
        let mut all_objects: ConfigObjectMap<T> = ConfigObjectMap::new();
        let mut page: u64 = 1;
        loop {
            // let paged_path_binding;
            // let paged_path = match page {
            //     1 => path.as_str(),
            //     _ => {
            //         paged_path_binding = format!("{}?page={}", path, page);
            //         paged_path_binding.as_str()
            //     }
            // };
            let paged_params: Option<Params> = match (page, params.clone()) {
                (1, _) => params.clone(),
                (_, Some(mut p)) => {
                    p.push(("page".to_string(), page.to_string()));
                    Some(p)
                }
                (_, None) => Some(vec![("page".to_string(), page.to_string())]),
            };

            let response = self.get(&path, paged_params).await?;
            let summary = parse_summary(&response)?;

            match summary_totalrows {
                None => {
                    summary_totalrows = Some(summary.totalrows);
                }
                Some(st) if st != summary.totalrows => {
                    return Err(OpsviewClientError::RowCountMismatch {
                        old: st,
                        new: summary.totalrows,
                    });
                }
                Some(_) => (),
            }

            let objects = response
                .get("list")
                .ok_or(OpsviewClientError::ObjectNotFound(format!(
                    "'{}' not found",
                    path
                )))?
                .as_array()
                .ok_or(OpsviewClientError::NotAnArray(format!(
                    "ConfigObject at '{}' is not an array",
                    path
                )))?;

            let mut coll: ConfigObjectMap<T> =
                serde_json::from_value(Value::Array(objects.to_vec()))?;
            all_objects.extend(&mut coll);

            if page == summary.totalpages {
                break;
            }

            page += 1;
        }

        if let Some(st) = summary_totalrows {
            if st != all_objects.len() as u64 {
                return Err(OpsviewClientError::RowCountMismatch {
                    old: st,
                    new: all_objects.len() as u64,
                });
            }
        }

        Ok(all_objects)
    }

    /// Gets all plugin configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all plugin
    /// configuration objects. It is typically used to retrieve a list of plugins for further
    /// processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `Plugin` objects on the
    /// Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response into JSON fails.
    pub async fn get_all_plugin_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<Plugin>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<Plugin>(params).await
    }

    /// Gets all role configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all role
    /// configuration objects. It is typically used to retrieve a list of roles for further
    /// processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `Role` objects on the
    /// Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response into JSON fails.
    pub async fn get_all_role_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<Role>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<Role>(params).await
    }

    /// Gets all service check configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all service check
    /// configuration objects. It is typically used to retrieve a list of service checks for further
    /// processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `ServiceCheck` objects on the
    /// Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response into JSON fails.
    pub async fn get_all_servicecheck_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<ServiceCheck>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<ServiceCheck>(params)
            .await
    }

    /// Gets all service group configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all service group
    /// configuration objects. It is typically used to retrieve a list of service groups for further
    /// processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `ServiceGroup` objects on the
    /// Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response into JSON fails.
    pub async fn get_all_servicegroup_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<ServiceGroup>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<ServiceGroup>(params)
            .await
    }

    /// Gets all shared notification profile configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all shared
    /// notification profile configuration objects. It is typically used to retrieve a list of shared
    /// notification profiles for further processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `SharedNotificationProfile` objects
    /// on the Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response into JSON fails.
    pub async fn get_all_sharednotificationprofile_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<SharedNotificationProfile>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<SharedNotificationProfile>(params)
            .await
    }

    /// Gets all tenancy configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all tenancy
    /// configuration objects. It is typically used to retrieve a list of tenancies for further
    /// processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `Tenancy` objects on the
    /// Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response into JSON fails.
    pub async fn get_all_tenancy_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<Tenancy>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<Tenancy>(params).await
    }

    /// Gets all timeperiod configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all timeperiod
    /// configuration objects. It is typically used to retrieve a list of timeperiods for further
    /// processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping an `ConfigObjectMap` containing the `TimePeriod` objects on the
    /// Opsview system.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or if parsing the response into JSON fails.
    pub async fn get_all_timeperiod_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<TimePeriod>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<TimePeriod>(params)
            .await
    }

    /// Gets all varibale (attribute) configuration objects from the Opsview system.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve all variable
    /// (attribute) configuration objects. It is typically used to retrieve a list of variables for
    /// further processing.
    ///
    /// # Arguments
    /// * `params` - Optional query parameters to be included in the request.
    pub async fn get_all_variable_configs(
        &self,
        params: Option<Params>,
    ) -> Result<ConfigObjectMap<Variable>, OpsviewClientError> {
        self.get_all_object_configs_by_type::<Variable>(params)
            .await
    }

    /// Retrieves the configuration of a BSM Component from the Opsview system by its name.
    ///
    /// This asynchronous method queries the Opsview API for the configuration of a BSM Component,
    /// identified by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the BSM Component whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the BSM Component configuration, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the BSM Component is not found or the
    /// configuration is missing.
    ///
    /// # Examples
    /// ```
    /// use opsview::client::OpsviewClient;
    /// use opsview::config::BSMComponent;
    /// use opsview::prelude::*;
    ///
    /// async fn example() -> Result<BSMComponent, OpsviewClientError> {
    ///    let client = OpsviewClient::new("api.example.com", "user", "pass", false).await?;
    ///    let bsm_component = client.get_bsmcomponent_config("exampleBSMComponent", None).await?;
    ///
    ///    println!("BSM Component: {}", bsm_component.name);
    ///
    ///    Ok(bsm_component)
    /// }
    /// ```
    pub async fn get_bsmcomponent_config(
        &self,
        name: &str,
        params: Option<Params>,
    ) -> Result<BSMComponent, OpsviewClientError> {
        self.get_object_config_by_key::<BSMComponent>("name", name, params)
            .await
    }

    /// Retrieves the configuration of a BSM Service from the Opsview system by its name.
    ///
    /// This asynchronous method queries the Opsview API for the configuration of a BSM Service,
    /// identified by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the BSM Service whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the BSM Service configuration, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the BSM Service is not found or the
    /// configuration is missing.
    ///
    /// # Examples
    /// ```
    /// use opsview::client::OpsviewClient;
    /// use opsview::config::BSMService;
    /// use opsview::prelude::*;
    ///
    /// async fn example() -> Result<BSMService, OpsviewClientError> {
    ///    let client = OpsviewClient::new("api.example.com", "user", "pass", false).await?;
    ///    let bsm_service = client.get_bsmservice_config("exampleBSMService", None).await?;
    ///
    ///    println!("BSM Service: {}", bsm_service.name);
    ///
    ///    Ok(bsm_service)
    /// }
    /// ```
    pub async fn get_bsmservice_config(
        &self,
        name: &str,
        params: Option<Params>,
    ) -> Result<BSMService, OpsviewClientError> {
        self.get_object_config_by_key::<BSMService>("name", name, params)
            .await
    }

    /// Retrieves the configuration of a contact from the Opsview system by its name.
    ///
    /// This asynchronous method queries the Opsview API for the configuration of a contact,
    /// identified by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the contact whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the contact configuration, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the contact is not found or the
    /// configuration is missing.
    ///
    /// # Examples
    /// ```
    /// use opsview::client::OpsviewClient;
    /// use opsview::config::Contact;
    /// use opsview::prelude::*;
    ///
    /// async fn example() -> Result<Contact, OpsviewClientError> {
    ///   let client = OpsviewClient::new("api.example.com", "user", "pass", false).await?;
    ///   let contact = client.get_contact_config("exampleContact", None).await?;
    ///
    ///   println!("Username: {}", contact.name);
    ///
    ///   if let Some(ref fullname) = contact.fullname {
    ///       println!("Full name: {}", fullname);
    ///   }
    ///
    ///   Ok(contact)
    ///   }
    /// ```
    pub async fn get_contact_config(
        &self,
        name: &str,
        params: Option<Params>,
    ) -> Result<Contact, OpsviewClientError> {
        self.get_object_config_by_key::<Contact>("name", name, params)
            .await
    }

    /// Retrieves the configuration of a hashtag (keyword) from the Opsview system by its name.
    ///
    /// This asynchronous method queries the Opsview API for the configuration of a hashtag,
    /// identified by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the hashtag whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the hashtag configuration, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the hashtag is not found or the
    /// configuration is missing.
    ///
    /// # Examples
    /// ```
    /// use opsview::client::OpsviewClient;
    /// use opsview::config::Hashtag;
    /// use opsview::prelude::*;
    ///
    /// async fn example() -> Result<Hashtag, OpsviewClientError> {
    ///   let client = OpsviewClient::new("api.example.com", "user", "pass", false).await?;
    ///   let hashtag = client.get_hashtag_config("exampleHashtag", None).await?;
    ///
    ///   println!("Hashtag: {}", hashtag.name);
    ///
    ///   Ok(hashtag)
    ///   }
    /// ```
    pub async fn get_hashtag_config(
        &self,
        name: &str,
        params: Option<Params>,
    ) -> Result<Hashtag, OpsviewClientError> {
        self.get_object_config_by_key::<Hashtag>("name", name, params)
            .await
    }

    /// Retrieves the configuration of a host from the Opsview system by its name.
    ///
    /// This asynchronous method queries the Opsview API for the configuration of a host,
    /// identified by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the host whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the host configuration, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the host is not found or the
    /// configuration is missing.
    ///
    /// # Examples
    /// ```
    /// use opsview::client::OpsviewClient;
    /// use opsview::config::Host;
    /// use opsview::prelude::*;
    ///
    /// async fn example() -> Result<Host, OpsviewClientError> {
    ///     let client = OpsviewClient::new("api.example.com", "user", "pass", false).await?;
    ///     let host = client.get_host_config("exampleHost", None).await?;
    ///
    ///     println!("Host: {}", host.name);
    ///     println!("Host group: {}", host.hostgroup.clone().unwrap().name());
    ///     println!("IP address: {}", host.ip.clone().unwrap());
    ///
    ///     Ok(host)
    /// }
    /// ```
    pub async fn get_host_config(
        &self,
        name: &str,
        params: Option<Params>,
    ) -> Result<Host, OpsviewClientError> {
        self.get_object_config_by_key::<Host>("name", name, params)
            .await
    }

    /// Retrieves the host configs that have a variable with a matching name.
    ///
    /// Unstable and experimental. This method is subject to change.
    pub async fn get_host_configs_by_matching_variable_name(
        &self,
        variable_name: &str,
    ) -> Result<ConfigObjectMap<Host>, OpsviewClientError> {
        let params = Some(vec![(
            "cols".to_string(),
            "hostattributes,id,name,ip".to_string(),
        )]);
        let all_hosts = self.get_all_host_configs(params).await?;
        let mut matching_hosts: ConfigObjectMap<Host> = ConfigObjectMap::new();

        for host in all_hosts.values() {
            if let Some(attributes) = &host.hostattributes {
                for (_name, attribute) in attributes.iter() {
                    if attribute.name == variable_name {
                        matching_hosts
                            .add(self.get_host_config_by_id(host.id.unwrap(), None).await?);
                    }
                }
            }
        }

        Ok(matching_hosts)
    }

    /// Retrieves the host configs that have a matching variable value.
    ///
    /// Unstable and experimental. This method is subject to change.
    pub async fn get_host_configs_by_matching_variable_value(
        &self,
        variable_name: &str,
        value: &str,
    ) -> Result<ConfigObjectMap<Host>, OpsviewClientError> {
        let params = Some(vec![(
            "cols".to_string(),
            "hostattributes,id,name,ip".to_string(),
        )]);
        let all_hosts = self.get_all_host_configs(params).await?;
        let mut matching_hosts: ConfigObjectMap<Host> = ConfigObjectMap::new();

        for host in all_hosts.values() {
            if let Some(attributes) = &host.hostattributes {
                for (_name, attribute) in attributes.iter() {
                    if attribute.name == variable_name
                        && (attribute.value.clone().unwrap_or_default() == value
                            || attribute.arg1.clone().unwrap_or_default() == value
                            || attribute.arg2.clone().unwrap_or_default() == value
                            || attribute.arg3.clone().unwrap_or_default() == value
                            || attribute.arg4.clone().unwrap_or_default() == value)
                    {
                        matching_hosts
                            .add(self.get_host_config_by_id(host.id.unwrap(), None).await?);
                    }
                }
            }
        }

        Ok(matching_hosts)
    }

    /// Retrieves the configuration of a host from the Opsview system by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the host whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the host configuration, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the host is not found or the
    /// configuration is missing.
    ///
    /// # Examples
    /// ```
    /// use opsview::client::OpsviewClient;
    /// use opsview::config::Host;
    /// use opsview::prelude::*;
    ///
    /// async fn example() -> Result<Host, OpsviewClientError> {
    ///    let client = OpsviewClient::new("api.example.com", "user", "pass", false).await?;
    ///    let host = client.get_host_config_by_id(1234, None).await?;
    ///
    ///    println!("Host: {}", host.name);
    ///
    ///    Ok(host)
    ///    }
    /// ```
    pub async fn get_host_config_by_id(
        &self,
        id: u64,
        params: Option<Params>,
    ) -> Result<Host, OpsviewClientError> {
        self.get_object_config_by_id::<Host>(id, params).await
    }

    /// Retrieves the ID of a host from the Opsview system based on the host name.
    ///
    /// This asynchronous method queries the Opsview API for the ID of a host configuration
    /// using the host's name.
    ///
    /// # Arguments
    /// * `name` - The name of the host whose ID is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the ID of the host as a u64, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the host is not found or the ID is missing.
    pub async fn get_host_id(
        &self,
        name: &str,
        params: Option<Params>,
    ) -> Result<u64, OpsviewClientError> {
        self.get_object_id_by_key::<Host>("name", name, params)
            .await
    }

    /// Retrives the configuration of a host check command from the Opsview system by its name.
    ///
    /// This asynchronous method queries the Opsview API for the configuration of a host check
    /// command, identified by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the host check command whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the host check command configuration, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the host check command is not found or the
    /// configuration is missing.
    pub async fn get_hostcheckcommand_config(
        &self,
        name: &str,
        params: Option<Params>,
    ) -> Result<HostCheckCommand, OpsviewClientError> {
        self.get_object_config_by_key::<HostCheckCommand>("name", name, params)
            .await
    }

    /// Retrieves the configuration of a host group from the Opsview system by its name.
    ///
    /// This asynchronous method queries the Opsview API for the configuration of a host group,
    /// identified by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the host group whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the host group configuration, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the host group is not found or the
    /// configuration is missing.
    pub async fn get_hostgroup_config(
        &self,
        name: &str,
        params: Option<Params>,
    ) -> Result<HostGroup, OpsviewClientError> {
        self.get_object_config_by_key::<HostGroup>("name", name, params)
            .await
    }

    /// Retrieves the configuration of a host icon from the Opsview system by its name.
    ///
    /// This asynchronous method queries the Opsview API for the configuration of a host icon,
    /// identified by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the host icon whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the host icon configuration, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the host icon is not found or the
    /// configuration is missing.
    pub async fn get_hosticon_config(
        &self,
        name: &str,
        params: Option<Params>,
    ) -> Result<HostIcon, OpsviewClientError> {
        self.get_object_config_by_key::<HostIcon>("name", name, params)
            .await
    }

    /// Retrieves the configuration of a host template from the Opsview system by its name.
    ///
    /// This asynchronous method queries the Opsview API for the configuration of a host template,
    /// identified by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the host template whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the host template configuration, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the host template is not found or the
    /// configuration is missing.
    pub async fn get_hosttemplate_config(
        &self,
        name: &str,
        params: Option<Params>,
    ) -> Result<HostTemplate, OpsviewClientError> {
        self.get_object_config_by_key::<HostTemplate>("name", name, params)
            .await
    }

    /// Retrieves the configuration of a monitoring cluster from the Opsview system by its name.
    ///
    /// This asynchronous method queries the Opsview API for the configuration of a monitoring
    /// cluster, identified by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the monitoring cluster whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the monitoring cluster configuration, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the monitoring cluster is not found or the
    /// configuration is missing.
    pub async fn get_monitoringcluster_config(
        &self,
        name: &str,
        params: Option<Params>,
    ) -> Result<MonitoringCluster, OpsviewClientError> {
        self.get_object_config_by_key::<MonitoringCluster>("name", name, params)
            .await
    }

    /// Retrieves the configuration of a notification method from the Opsview system by its name.
    ///
    /// This asynchronous method queries the Opsview API for the configuration of a notification
    /// method, identified by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the notification method whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the notification method configuration, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the notification method is not found or the
    /// configuration is missing.
    pub async fn get_notificationmethod_config(
        &self,
        name: &str,
        params: Option<Params>,
    ) -> Result<NotificationMethod, OpsviewClientError> {
        self.get_object_config_by_key::<NotificationMethod>("name", name, params)
            .await
    }

    /// Retrieves the configuration of an object from the Opsview system by its ID.
    ///
    /// This asynchronous method queries the Opsview API for the configuration of an object,
    /// identified by its ID. It is a generic method that can be used for different types
    /// of objects.
    ///
    /// # Type Parameters
    /// * `T` - The type of the object. Must implement the `ConfigObject` trait.
    ///
    /// # Arguments
    /// * `id` - The ID of the object whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the configuration of the object as a generic type, or an error if the
    /// operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the object is not found or the configuration
    /// is missing.
    async fn get_object_config_by_id<T: ConfigObject>(
        &self,
        id: u64,
        params: Option<Params>,
    ) -> Result<T, OpsviewClientError> {
        let path = T::config_path().ok_or(OpsviewClientError::NoConfigPath)?;
        let full_path = format!("{}/{}", path, id);
        let response = self.get(&full_path, params).await?;
        let response_object = response
            .get("object")
            .ok_or(OpsviewClientError::ObjectNotFound(format!(
                "ConfigObject '{}' not found at '{}'",
                id, full_path
            )))?;
        println!("response_object: {:#?}", response_object);
        let object: T = serde_json::from_value(response_object.clone())?;
        Ok(object)
    }

    /// Retrieves the configuration of a specific object of type `T` that implements `ConfigObject`.
    ///
    /// This asynchronous method queries the Opsview API for an object's configuration, using a
    /// specified key-value pair within a given path.
    ///
    /// # Arguments
    /// * `key` - The key used to identify the specific object.
    /// * `value` - The value of the key for the specific object.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the configuration of the object as a generic type, or an error if the
    /// operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the object is not found or the configuration
    /// is missing.
    async fn get_object_config_by_key<T: ConfigObject>(
        &self,
        key: &str,
        value: &str,
        params: Option<Params>,
    ) -> Result<T, OpsviewClientError> {
        let path = T::config_path().ok_or(OpsviewClientError::NoConfigPath)?;
        let response = self
            .get(&format!("{}?s.{}={}", path, key, value), params)
            .await?;
        let response_object = response
            .get("list")
            .and_then(|v| v.get(0))
            .ok_or(OpsviewClientError::ObjectNotFound(format!(
                "ConfigObject with '{}' = '{}' not found at '{}'",
                key, value, path
            )))?
            .clone();
        let object: T = serde_json::from_value(response_object)?;
        Ok(object)
    }

    /// Retrieves the configuration of a specific object of type `T` that implements `ConfigObject`
    /// from the Opsview API.
    ///
    /// This asynchronous method queries the Opsview API for an object's configuration, using a
    /// specified ref.
    ///
    /// # Arguments
    /// * `ref_` - The ref of the object to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the configuration of the object as a generic type, or an error if the
    /// operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the object is not found or the configuration
    /// is missing.
    async fn get_object_config_by_ref<T: ConfigObject>(
        &self,
        ref_: &str,
        params: Option<Params>,
    ) -> Result<T, OpsviewClientError> {
        let path = path_from_ref(ref_)?;
        let response = self.get(&path, params).await?;
        let response_object = response
            .get("object")
            .ok_or(OpsviewClientError::ObjectNotFound(format!(
                "ConfigObject not found at '{}'",
                &path
            )))?
            .clone();
        let object: T = serde_json::from_value(response_object)?;
        Ok(object)
    }

    /// Retrieves the ID of an object from the Opsview system based on a key-value pair.
    ///
    /// This asynchronous method queries the Opsview API for an object's ID, using a specified
    /// key-value pair within a given path.
    ///
    /// # Arguments
    /// * `path` - The API path where the object is located.
    /// * `key` - The key used to identify the specific object.
    /// * `value` - The value of the key for the specific object.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the ID of the object as a u64, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the object is not found or the ID is missing.
    async fn get_object_id_by_key<T: ConfigObject>(
        &self,
        key: &str,
        value: &str,
        params: Option<Params>,
    ) -> Result<u64, OpsviewClientError> {
        let path = T::config_path().ok_or(OpsviewClientError::NoConfigPath)?;
        let response = self
            .get(&format!("{}?s.{}={}", path, key, value), params)
            .await?;
        let object_id = response
            .get("list")
            .and_then(|v| v.get(0))
            .and_then(|o| o.get("id"))
            .ok_or(OpsviewClientError::IdNotFound(format!(
                "ConfigObject '{}' at '{}' has no ID",
                value, path
            )))?
            .as_str()
            .ok_or(OpsviewClientError::IdParseError(format!(
                "ConfigObject '{}' at '{}' has an ID that is not castable to str",
                value, path
            )))?
            .parse::<u64>()?;

        Ok(object_id)
    }

    /// Retrieves the configuration of a plugin from the Opsview system by its name.
    ///
    /// This asynchronous method queries the Opsview API for the configuration of a plugin,
    /// identified by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the plugin whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the plugin configuration, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the plugin is not found or the
    /// configuration is missing.
    pub async fn get_plugin_config(
        &self,
        name: &str,
        params: Option<Params>,
    ) -> Result<Plugin, OpsviewClientError> {
        self.get_object_config_by_key::<Plugin>("name", name, params)
            .await
    }

    /// Returns the raw text response from the Opsview API for a given path.
    ///
    /// This asynchronous method sends a GET request to the Opsview API to retrieve the raw text
    /// response for a given path.
    ///
    /// This can be useful for debugging purposes.
    ///
    /// # Arguments
    /// * `path` - The API path to be queried.
    ///
    /// # Returns
    /// A `Result` wrapping the raw text response from the Opsview API.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the response cannot be parsed into a
    /// string.
    pub async fn get_raw(&self, path: &str) -> Result<String, OpsviewClientError> {
        let url = Url::parse(&format!("{}/rest{}", self.url, path))?;
        let response = self.client.get(url.as_ref()).send().await?;
        let response_body = response.text().await?;
        Ok(response_body)
    }

    /// Retrieves the configuration of a role from the Opsview system by its name.
    ///
    /// This asynchronous method queries the Opsview API for the configuration of a role,
    /// identified by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the role whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the role configuration, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the role is not found or the
    /// configuration is missing.
    pub async fn get_role_config(
        &self,
        name: &str,
        params: Option<Params>,
    ) -> Result<Role, OpsviewClientError> {
        self.get_object_config_by_key::<Role>("name", name, params)
            .await
    }

    /// Retrieves the configuration of a service check from the Opsview system by its name.
    ///
    /// This asynchronous method queries the Opsview API for the configuration of a service check,
    /// identified by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the service check whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the service check configuration, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the service check is not found or the
    /// configuration is missing.
    pub async fn get_servicecheck_config(
        &self,
        name: &str,
        params: Option<Params>,
    ) -> Result<ServiceCheck, OpsviewClientError> {
        self.get_object_config_by_key::<ServiceCheck>("name", name, params)
            .await
    }

    /// Retrieves the configuration of a service group from the Opsview system by its name.
    ///
    /// This asynchronous method queries the Opsview API for the configuration of a service group,
    /// identified by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the service group whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the service group configuration, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the service group is not found or the
    /// configuration is missing.
    pub async fn get_servicegroup_config(
        &self,
        name: &str,
        params: Option<Params>,
    ) -> Result<ServiceGroup, OpsviewClientError> {
        self.get_object_config_by_key::<ServiceGroup>("name", name, params)
            .await
    }

    /// Retrieves the configuration of a shared notification profile from the Opsview system by its
    /// name.
    ///
    /// This asynchronous method queries the Opsview API for the configuration of a shared
    /// notification profile, identified by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the shared notification profile whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the shared notification profile configuration, or an error if the
    /// operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the shared notification profile is not
    /// found or the configuration is missing.
    pub async fn get_sharednotificationprofile_config(
        &self,
        name: &str,
        params: Option<Params>,
    ) -> Result<SharedNotificationProfile, OpsviewClientError> {
        self.get_object_config_by_key::<SharedNotificationProfile>("name", name, params)
            .await
    }

    /// Retrieves the configuration of a timeperiod from the Opsview system by its name.
    ///
    /// This asynchronous method queries the Opsview API for the configuration of a timeperiod,
    /// identified by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the timeperiod whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the timeperiod configuration, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the timeperiod is not found or the
    /// configuration is missing.
    pub async fn get_timeperiod_config(
        &self,
        name: &str,
        params: Option<Params>,
    ) -> Result<TimePeriod, OpsviewClientError> {
        self.get_object_config_by_key::<TimePeriod>("name", name, params)
            .await
    }

    /// Retrieves the configuration of a variable (attribute) from the Opsview system by its name.
    ///
    /// This asynchronous method queries the Opsview API for the configuration of a variable
    /// (attribute), identified by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the variable whose configuration is to be retrieved.
    /// * `params` - Optional parameters to be included in the request.
    ///
    /// # Returns
    /// A `Result` wrapping the variable configuration, or an error if the operation fails.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the variable is not found or the
    /// configuration is missing.
    pub async fn get_variable_config(
        &self,
        name: &str,
        params: Option<Params>,
    ) -> Result<Variable, OpsviewClientError> {
        self.get_object_config_by_key::<Variable>("name", name, params)
            .await
    }

    /// Logs out of the Opsview system, invalidating the token.
    pub async fn logout(&self) -> Result<Value, OpsviewClientError> {
        self.post("/logout", &serde_json::Value::Null).await
    }

    /// Sends a POST request to the Opsview API to create a new configuration object.
    pub async fn post_new_object_config<T: Persistent>(
        &self,
        obj: &T,
    ) -> Result<Value, OpsviewClientError> {
        let path = T::config_path().ok_or(OpsviewClientError::NoConfigPath)?;

        // Serialize the object to a serde_json Value
        let obj_value = serde_json::to_value(obj)?;

        // Wrap the object in the desired structure
        let wrapped_obj = serde_json::json!({ "object": obj_value });

        self.post(&path, &wrapped_obj).await
    }

    /// Sends a POST request to the Opsview API to create a list of new configuration objects.
    pub async fn post_new_object_config_map<T: PersistentMap>(
        &self,
        coll: &T,
    ) -> Result<Value, OpsviewClientError> {
        let path = T::config_path().ok_or(OpsviewClientError::NoConfigPath)?;
        let coll_value = serde_json::to_value(coll)?;
        let wrapped_coll = serde_json::json!({ "list": coll_value });
        self.post(&path, &wrapped_coll).await
    }

    /// Sends a PUT request to the Opsview API to update an existing configuration object.
    ///
    /// This asynchronous method sends a PUT request to the Opsview API to update an existing
    /// configuration object. If the object doesn't exist, it will be created if possible.
    ///
    /// # Arguments
    /// * `object` - The object to be updated.
    ///
    /// # Returns
    /// A `Result` wrapping the JSON response from the Opsview API.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails, or if the response cannot be parsed into a
    /// JSON object.
    ///
    /// # Examples
    /// ```
    /// use opsview::client::OpsviewClient;
    /// use opsview::config::Hashtag;
    /// use opsview::prelude::*;
    ///
    /// async fn example() -> Result<(), OpsviewError> {
    ///   let client = OpsviewClient::builder()
    ///      .url("api.example.com")
    ///      .username("api_user")
    ///      .password("my_password")
    ///      .build().await?;
    ///      
    ///   let hashtag = Hashtag::builder()
    ///     .name("exampleHashtag")
    ///     .description("Example hashtag")
    ///     .enabled(true)
    ///     .all_hosts(true)
    ///     .build()?;
    ///
    ///   client.put_object_config(&hashtag).await?;
    ///
    ///   Ok(())
    /// }
    /// ```
    pub async fn put_object_config<T: Persistent>(
        &self,
        object: &T,
    ) -> Result<Value, OpsviewClientError> {
        let path = T::config_path().ok_or(OpsviewClientError::NoConfigPath)?;
        let obj_value = serde_json::to_value(object)?;
        let wrapped_obj = serde_json::json!({ "object": obj_value });
        self.put(&path, &wrapped_obj).await
    }
}

fn path_from_ref(ref_: &str) -> Result<String, OpsviewClientError> {
    if !ref_.starts_with("/rest/config/") {
        Err(OpsviewClientError::InvalidRef(
            "Ref must start with '/rest/config/'".to_string(),
            ref_.to_string(),
        ))
    } else {
        let path = ref_.replace("/rest/config/", "/config/");
        Ok(path)
    }
}

fn interpret_exists_response(response: Value) -> Result<bool, OpsviewClientError> {
    match response["exists"].as_str() {
        Some("1") => Ok(true),
        Some("0") => Ok(false),
        None => Err(OpsviewClientError::AuthError(
            "Response does not contain 'exists' field".to_string(),
        )),
        Some(v) => Err(OpsviewClientError::AuthError(format!(
            "Unexpected value for 'exists' field: '{}'",
            v
        ))),
    }
}

async fn handle_http_response(response: reqwest::Response) -> Result<Value, OpsviewClientError> {
    match response.status() {
        StatusCode::OK => {
            let response_json = response.json::<Value>().await?;
            Ok(response_json)
        }
        StatusCode::UNAUTHORIZED => {
            let error_body = response.text().await?;
            let error_json: Value = serde_json::from_str(&error_body)
                .unwrap_or_else(|_| json!({"error": "Unauthorized access"}));
            let error_message = error_json["message"]
                .as_str()
                .unwrap_or("Unauthorized access");
            Err(OpsviewClientError::AuthError(error_message.to_string()))
        }
        StatusCode::NOT_FOUND => Err(OpsviewClientError::ResourceNotFound(
            "Resource not found".to_string(),
        )),
        StatusCode::INTERNAL_SERVER_ERROR => Err(OpsviewClientError::InternalServerError(
            "Internal server error".to_string(),
        )),
        StatusCode::BAD_REQUEST => {
            let error_body = response
                .text()
                .await
                .map_err(|e| OpsviewClientError::UndefinedError(e.to_string()))?;
            let error_json: Value = serde_json::from_str(&error_body)
                .unwrap_or_else(|_| json!({"error": "Bad request"}));
            let error_message = error_json["message"].as_str().unwrap_or("Bad request");
            Err(OpsviewClientError::BadRequest(error_message.to_string()))
        }
        _ => {
            let error_body = response.text().await?;
            let error_json: Value = serde_json::from_str(&error_body)
                .unwrap_or_else(|_| json!({"error": "Unknown error"}));
            let error_message = error_json["message"].as_str().unwrap_or("Unknown error");
            Err(OpsviewClientError::UndefinedError(
                error_message.to_string(),
            ))
        }
    }
}

struct Summary {
    pub totalpages: u64,
    pub totalrows: u64,
}

fn required_response_field(response: &Value, field: &str) -> Result<Value, OpsviewClientError> {
    match response.get(field) {
        Some(v) => Ok(v.clone()),
        None => Err(OpsviewClientError::MissingResponseField(field.to_string())),
    }
}

fn summary_value_as_u64(summary: &Value, field: &str) -> Result<u64, OpsviewClientError> {
    match summary.get(field) {
        None => Err(OpsviewClientError::MissingResponseField(field.to_string())),
        Some(v) => match v.as_str() {
            None => Err(OpsviewClientError::TypeParseError(
                field.to_string(),
                "&str".to_string(),
            )),
            Some(s) => Ok(s.parse::<u64>()?),
        },
    }
}

fn parse_summary(response: &Value) -> Result<Summary, OpsviewClientError> {
    let summary = required_response_field(response, "summary")?;
    Ok(Summary {
        totalpages: summary_value_as_u64(&summary, "totalpages")?,
        totalrows: summary_value_as_u64(&summary, "totalrows")?,
    })
}

#[cfg(test)]
mod print_tests {
    use super::*;
    use std::env;
    // Because we might want to tweak what's printed on the fly, we'll ignore clippy's suggestion to
    // remove the unused type parameter.
    #[allow(clippy::extra_unused_type_parameters)]
    async fn print_get_response_by_path<T: ConfigObject>(
        client: &OpsviewClient,
        path: &str,
        params: Option<Params>,
    ) -> Result<(), OpsviewClientError> {
        let params = match params.clone() {
            None => Some(vec![]),
            Some(mut p) => {
                p.push(("s.sort".to_string(), "name".to_string()));
                Some(p)
            }
        };
        let response = client.get(path, params).await?;
        let pretty_json = serde_json::to_string_pretty(&response)?;
        println!("API response:{}", pretty_json);
        Ok(())
    }

    /// Sets up an `OpsviewClient` for testing purposes, if the required environment variables
    /// are set. If not, returns `None`.
    async fn setup_opsview_client() -> Result<Option<OpsviewClient>, OpsviewClientError> {
        if env::var("OV_URL").is_err()
            || env::var("OV_USERNAME").is_err()
            || env::var("OV_PASSWORD").is_err()
        {
            return Ok(None);
        }

        let client = OpsviewClient::builder()
            .url(&env::var("OV_URL").unwrap())
            .username(&env::var("OV_USERNAME").unwrap())
            .password(&env::var("OV_PASSWORD").unwrap())
            .ignore_cert(true)
            .build()
            .await?;

        Ok(Some(client))
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // # cargo test -- print_all_bsmcomponent_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_bsmcomponent_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<BSMComponent>(&client, "/config/bsmcomponent", None)
                .await?;
            client.logout().await?;
        }
        Ok(())
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // # cargo test -- print_all_bsmservice_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_bsmservice_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<BSMService>(&client, "/config/bsmservice", None).await?;
            client.logout().await?;
        }
        Ok(())
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // # cargo test -- print_all_contact_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_contact_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<Contact>(&client, "/config/contact", None).await?;
            client.logout().await?;
        }
        Ok(())
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // # cargo test -- print_all_hashtag_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_hashtag_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<Hashtag>(&client, "/config/keyword", None).await?;
            client.logout().await?;
        }
        Ok(())
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // # cargo test -- print_all_host_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_host_configs() -> Result<(), OpsviewClientError> {
        let params = Some(vec![("cols".to_string(), "+snmpinterfaces".to_string())]);
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<Host>(&client, "/config/host", params).await?;
            client.logout().await?;
        }
        Ok(())
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // # cargo test -- print_all_hostcheckcommand_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_hostcheckcommand_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<HostCheckCommand>(
                &client,
                "/config/hostcheckcommand",
                None,
            )
            .await?;
            client.logout().await?;
        }
        Ok(())
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // # cargo test -- print_all_hostgroup_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_hostgroup_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<HostGroup>(&client, "/config/hostgroup", None).await?;
            client.logout().await?;
        }
        Ok(())
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // # cargo test -- print_all_hosticon_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_hosticon_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<HostIcon>(&client, "/config/hosticons", None).await?;
            client.logout().await?;
        }
        Ok(())
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // # cargo test -- print_all_hosttemplate_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_hosttemplate_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<HostTemplate>(&client, "/config/hosttemplate", None)
                .await?;
            client.logout().await?;
        }
        Ok(())
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // ```
    // # cargo test -- print_all_monitoringcluster_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_monitoringcluster_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<MonitoringCluster>(
                &client,
                "/config/monitoringcluster",
                None,
            )
            .await?;
            client.logout().await?;
        }
        Ok(())
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // # cargo test -- print_all_netflowcollector_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_netflowcollector_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<NetflowCollector>(
                &client,
                "/config/netflow_collector",
                None,
            )
            .await?;
            client.logout().await?;
        }
        Ok(())
    }

    /// Needs to be run with --ignored and --nocapture to print the output.
    /// Example:
    /// ```shell
    /// # cargo test -- print_all_netflowsource_configs --ignored --nocapture
    /// ```
    #[ignore]
    #[tokio::test]
    async fn print_all_netflowsource_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<NetflowSource>(&client, "/config/netflow_source", None)
                .await?;
            client.logout().await?;
        }
        Ok(())
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // # cargo test -- print_all_notificationmethod_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_notificationmethod_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<NotificationMethod>(
                &client,
                "/config/notificationmethod",
                None,
            )
            .await?;
            client.logout().await?;
        }
        Ok(())
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // # cargo test -- print_all_plugin_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_plugin_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<Plugin>(&client, "/config/plugin", None).await?;
            client.logout().await?;
        }
        Ok(())
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // # cargo test print_all_role_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_role_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<Role>(&client, "/config/role", None).await?;
            client.logout().await?;
        }
        Ok(())
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // # cargo test -- print_all_servicecheck_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_servicecheck_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<ServiceCheck>(&client, "/config/servicecheck", None)
                .await?;
            client.logout().await?;
        }
        Ok(())
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // # cargo test -- print_all_servicegroup_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_servicegroup_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<ServiceGroup>(&client, "/config/servicegroup", None)
                .await?;
            client.logout().await?;
        }
        Ok(())
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // # cargo test -- print_all_sharednotificationprofile_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_sharednotificationprofile_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<SharedNotificationProfile>(
                &client,
                "/config/sharednotificationprofile",
                None,
            )
            .await?;
            client.logout().await?;
        }
        Ok(())
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // # cargo test -- print_all_tenancy_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_tenancy_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<Tenancy>(&client, "/config/tenancy", None).await?;
            client.logout().await?;
        }
        Ok(())
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // # cargo test print_all_timeperiod_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_timeperiod_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<TimePeriod>(&client, "/config/timeperiod", None).await?;
            client.logout().await?;
        }
        Ok(())
    }

    // Needs to be run with --ignored and --nocapture to print the output.
    // Example:
    // ```shell
    // # cargo test print_all_variable_configs --ignored --nocapture
    // ```
    #[ignore]
    #[tokio::test]
    async fn print_all_variable_configs() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            print_get_response_by_path::<Variable>(&client, "/config/attribute", None).await?;
            client.logout().await?;
        }
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_object_id_by_key() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            let host_id = client
                .get_object_id_by_key::<Host>("name", "opsview", None)
                .await;

            println!("host_id: {:?}", host_id);
            assert!(host_id.is_ok());
            client.logout().await?;
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_get_host_config_by_variable_name() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            let hosts = client
                .get_host_configs_by_matching_variable_name("DISK")
                .await?;

            println!("matching hosts: {}", hosts.len());
            println!("hosts: {:#?}", hosts);
            assert!(!hosts.is_empty());
            client.logout().await?;
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_get_host_config_by_variable_value() -> Result<(), OpsviewClientError> {
        if let Some(client) = setup_opsview_client().await? {
            let hosts = client
                .get_host_configs_by_matching_variable_value("DISK", "/")
                .await?;

            println!("matching hosts: {}", hosts.len());
            println!("hosts: {:#?}", hosts);
            assert!(!hosts.is_empty());
            client.logout().await?;
        }

        Ok(())
    }
}
