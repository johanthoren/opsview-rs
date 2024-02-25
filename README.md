# Rust Opsview API Client Library

[![crates.io](https://img.shields.io/crates/v/opsview.svg)](https://crates.io/crates/opsview)
[![Documentation](https://docs.rs/opsview/badge.svg)](https://docs.rs/opsview)
[![ISC licensed](https://img.shields.io/crates/l/opsview.svg)](./LICENSE)

## Introduction

The *opsview* crate is a Rust library designed to interact with the [Opsview
monitoring software](https://www.itrsgroup.com/infrastructure-monitoring). It
provides a comprehensive coverage of the Opsview REST API and allows you to
handle Opsview objects such as hosts, service checks, host groups, and more. The
initial focus is on the *client* and *config* modules which enables the user to
perform all kinds of object configuration management using this Rust library.

## Project Status

This project is currently in development and is subject to change at any time.
It is not yet complete.

## Features

- Comprehensive coverage of Opsview objects: Hosts, Service Checks, Host Groups,
  etc. All objects in the configuration API are available as native Rust
  objects.
- Builder pattern objects for creating and configuring Opsview objects.
- Asynchronous API interactions with built-in error handling.
- Custom serialization and deserialization for Opsview API compatibility.

## Why should you use this library?

This library aims to provide client side validation of as many fields as
possible. That means that you can add solid error handling of any incompatible
field values before even making an API call to the Opsview server and instead
catch and handle the error returned by the API.

For example, let's say that your program parses a name from some file and
decides to create a `Hashtag` with the name `My New Hashtag`. You don't have to
cover all name checks on your side, you can simply try to build a `Hashtag`
using either of the associated `builder` or `minimal` functions and then match
the `Result`. No need to even connect to the Opsview API.

Example:

```rust,no_run
use opsview::config::Hashtag;
use opsview::prelude::*;

#[tokio::main]
async fn main() {
    let my_new_hashtag = Hashtag::minimal("My New Hashtag");

    match my_new_hashtag {
        Err(OpsviewConfigError::DoesNotMatchRegex(_, _)) => { todo!("Do something") }, // <-- This is the matching branch
        Err(_) => { todo!("Do something else") }
        Ok(_) => (), // Do nothing.
    }
}
```

This allows you to write robust solutions with fewer errors.

Adding these checks is a priority but still a work in progress. All fields
missing validation are marked with a TODO in the source code. If you find one
that doesn't, please let me know.

## Basic Usage

This library makes frequent use of the Builder pattern to create and configure
Opsview objects.

All configuration related objects such as `Hashtag`, `Host`, `ServiceCheck` and
so on are represented as native Rust structs with the trait `ConfigObject`.
These all have the `Builder` trait which defines the `builder()` function which
will initiate a new builder object.

The standard pattern for creating a new object is to use the associated
`builder()` function of the type of `ConfigObject` that you want to create to
create a new builder, then chain the builder's methods to configure the object,
and finally call `build()` to create the object. Using the `build()` method will
give you some assurances that the `ConfigObject` that you are trying to create
is valid.

Note that there are no *online* checks when building the object, you will still
have to check for existing names, etc, at some point. But it will force you to
populate all required fields with valid data.

It is generally discouraged to create `ConfigObject` structs directly, as this
may result in invalid objects that cannot be used with the Opsview API. These
objects are primarily used for deserialization from the Opsview API.

Here's a quick example to get you started with this library:

```rust,no_run
use opsview::client::OpsviewClient;
use opsview::config::Hashtag;
use opsview::prelude::*;

async fn new_hashtag(
  client: &OpsviewClient, 
  name: &str
  ) -> Result<(), OpsviewError> {
    let new_hashtag = Hashtag::builder()
        .name(name)
        .description("This Hashtag was created using Rust")
        .enabled(true)
        .all_hosts(true)
        .build()?;
    
    new_hashtag.create(client).await?;
    
    Ok(())
}

#[tokio::main]
async fn main() {
    let client = OpsviewClient::builder()
        .url("https://opsview.example.com")
        .username("admin")
        .password("password")
        .build()
        .await
        .unwrap();

    let result = new_hashtag(&client, "foo").await;
    
    match result {
        Ok(_) => { println!("Success") },
        Err(_) => { println!("Failure") },
    }

    client.apply_changes().await.expect("Failed to apply changes");
    
    client.logout().await.expect("Failed to log out");
}
```

A more complex example:

``` rust,no_run
use opsview::client::OpsviewClient;
use opsview::config::{Host,Hashtag,HostGroup, HostTemplate};
use opsview::prelude::*;

#[tokio::main]
async fn main() {
    let client = OpsviewClient::builder()
        .url("https://opsview.example.com")
        .username("admin")
        .password("password")
        .build()
        .await
        .unwrap();
        
    let master_monitoring_server = client
        .get_monitoringcluster_config("Master Monitoring Server")
        .await
        .expect("Couldn't fetch 'Master Monitoring Server' from the API");

    let root_hostgroup = client
        .get_hostgroup_config("Opsview")
        .await
        .expect("Couldn't fetch HostGroup with the name 'Opsview' from the API");
    
    let opsview_rs_hostgroup = HostGroup::builder()
        .name("OpsviewRS")
        .parent(root_hostgroup)
        .build()
        .expect("Failed to build hostgroup 'opsview_rs_hostgroup'");

    // Create the HostGroup before adding is to the Host, since doing so will
    // consume the HostGroup and require a clone if the call to create is done
    // later.
    opsview_rs_hostgroup
        .create(&client)
        .await
        .expect("Failed to create hostgroup 'opsview_rs_hostgroup'");
        
    let network_base_template = client
        .get_hosttemplate_config("Network - Base")
        .await
        .expect("Couldn't fetch 'Network - Base' from the API");
    
    let mut templates = ConfigObjectMap::<HostTemplate>::new();
    templates.add(network_base_template);
    let templates = templates; // Optional shadowing to avoid mutable objects.

    let opsview_rs_hashtag = Hashtag::builder()
        .name("OpsviewRS")
        .description("This Hashtag represents objects created by opsview-rs")
        .build()
        .expect("Failed to build hashtag 'OpsviewRS'");

    // Create the Hashtag before adding it to the ConfigObjectMap<Hashtag>
    // since adding it will consume the Hashtag and require a clone if the
    // call to create is done later.
    opsview_rs_hashtag
        .create(&client)
        .await
        .expect("Failed to create hashtag 'opsview_rs_hashtag'");
        
    let mut hashtags = ConfigObjectMap::<Hashtag>::new();
    hashtags.add(opsview_rs_hashtag);
    let hashtags = hashtags; // Optional shadowing to avoid mutable objects.

    let new_host = Host::builder()
        .name("MyNewHost")
        .alias("This host was created using opsview-rs")
        .ip("127.0.0.1")
        .monitored_by(master_monitoring_server)
        .hostgroup(opsview_rs_hostgroup)
        .hosttemplates(&templates)
        .hashtags(&hashtags)
        .build()
        .expect("Failed to build host 'MyNewHost'");
        
    new_host
        .create(&client)
        .await
        .expect("Failed to create host 'new_host'");
        
    client.apply_changes().await.expect("Failed to apply changes");

    client.logout().await.expect("Failed to log out");
}
```

## Documentation

For detailed documentation on all available modules, structs, and functions,
please refer to the generated docs using:

```bash
cargo doc --open
```

## Affiliation with ITRS Group

This project is not affiliated with [ITRS Group](https://www.itrsgroup.com).

## Support and bug reports

Please direct any questions or bug reports to [the GitHub page of the
project](https://github.com/johanthoren/opsview-rs) and not to ITRS Group
Support.

## Testing

For testing with a live Opsview server, make sure to populate the following
environment variables:

``` bash
OV_URL
OV_USERNAME
OV_PASSWORD
```

When running the `ignored` tests, make sure to use `--test-threads=1` and that
there are no unsaved changes on the Opsview server in question.

## License

Copyright © 2024 Johan Thorén <johan@thoren.xyz>

This project is released under the ISC license. See the LICENSE file for more
details.
