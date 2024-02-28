#![warn(missing_docs)]
#![recursion_limit = "1024"]
//! # Rust Opsview API Client library
//!
//! The `opsview` crate provides a Rust interface to the [Opsview REST
//! API](https://docs.itrsgroup.com/docs/opsview/current/rest-api/rest-api-background/api-introduction/index.html).
//!
//! ## Example
//! ```rust,no_run
//! use opsview::prelude::*;
//! use opsview::client::OpsviewClient;
//! use opsview::config::{Host, HostGroup, HostCheckCommand, MonitoringCluster};
//!
//! #[tokio::main]
//! async fn main() {
//!    // Create a new OpsviewClient instance.
//!    let client = OpsviewClient::builder()
//!        .url("https://opsview.example.com/")
//!        .username("admin")
//!        .password("initial")
//!        .ignore_cert(false) // Set to true if using a self-signed certificate.
//!        .build()
//!        .await
//!        .unwrap();
//!
//!    // Build a new `HostGroup` configuration object.
//!    let mut host_group = HostGroup::builder()
//!        .name("My Hostgroup")
//!        .parent(HostGroup::minimal("Opsview").unwrap())
//!        .build()
//!        .unwrap();
//!
//!    // If it exists already, then fetch it.
//!    if host_group.exists(&client).await.unwrap() {
//!        host_group = host_group.fetch(&client).await.unwrap();
//!    } else {
//!        host_group.create(&client).await.unwrap();
//!    }
//!
//!    // Build a new `HostCheckCommand` configuration object.
//!    let mut host_check_command = HostCheckCommand::minimal("ping").unwrap();
//!
//!    // If it exists already, then fetch it.
//!    if host_check_command.exists(&client).await.unwrap() {
//!        host_check_command = host_check_command.fetch(&client).await.unwrap();
//!    } else {
//!        host_check_command.create(&client).await.unwrap();
//!    }
//!
//!    // Build a new `Host` configuration object.
//!    let host = Host::builder()
//!        .name("My_Host")
//!        .hostgroup(host_group)
//!        .check_command(host_check_command)
//!        .ip("192.168.1.100")
//!        .monitored_by(MonitoringCluster::minimal("Master Monitoring Server").unwrap())
//!        .build()
//!        .unwrap();
//!
//!    // Create the host if it doesn't exist already.
//!    if !host.exists(&client).await.unwrap() {
//!        host.create(&client).await.unwrap();
//!    }
//!
//!    // Apply the changes and logout.
//!    client.apply_changes().await.unwrap();
//!    client.logout().await.unwrap();
//! }
//! ```

/// The `client` module contains the `OpsviewClient` struct and methods for interacting with the
/// Opsview API using this Client.
pub mod client;

/// The `config` module contains most of the structs and methods for interacting with the Opsview
/// via the REST API /config endpoint.
pub mod config;

/// The `error` module contains the `OpsviewClientError` enum , the `OpsviewConfigError`, and
/// methods for handling errors
pub mod error;

/// The `instance` module contains the `OpsviewInstance` struct and methods for interacting with
/// an Opsview instance at large.
pub mod instance;

/// The `prelude` module contains the most commonly used types and traits from the `opsview` crate.
pub mod prelude;

/// The `util` module contains utility functions and types used throughout the `opsview` crate.
pub mod util;

/// The `state` module contains the `HostState` and `ServiceCheckState` enums.
pub mod state;
