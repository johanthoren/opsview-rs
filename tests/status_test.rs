#![allow(unused_imports)]
extern crate opsview;
use once_cell::sync::Lazy;
use opsview::client::OpsviewClient;
use opsview::{config::*, prelude::*, status::*, util::*};
use pretty_assertions::assert_eq;
use std::env;
use tokio::sync::Mutex;

static CLIENT: Lazy<Mutex<Option<OpsviewClient>>> = Lazy::new(|| Mutex::new(None));

async fn get_or_initialize_client() -> &'static Mutex<Option<OpsviewClient>> {
    let mut client = CLIENT.lock().await;
    if client.is_none() {
        *client = Some(
            OpsviewClient::builder()
                .url(&env::var("OV_URL").expect("OV_URL not set"))
                .username(&env::var("OV_USERNAME").expect("OV_USERNAME not set"))
                .password(&env::var("OV_PASSWORD").expect("OV_PASSWORD not set"))
                .ignore_cert(true)
                .build()
                .await
                .expect("Failed to create OpsviewClient"),
        );
    }

    &*CLIENT
}

#[tokio::test]
async fn print_hostgroup_summary() -> Result<(), OpsviewClientError> {
    if env::var("OV_URL").is_err() {
        return Ok(());
    }

    if env::var("OV_USERNAME").is_err() {
        return Ok(());
    }

    if env::var("OV_PASSWORD").is_err() {
        return Ok(());
    }

    let client_lock = get_or_initialize_client().await;
    let client_guard = client_lock.lock().await;

    let client = client_guard
        .as_ref()
        .ok_or("OpsviewClient is not initialized")?;

    let status = client.get_hostgroup_status_summary().await?;

    println!("{:#?}", status);

    assert!(false);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn print_host_summary() -> Result<(), OpsviewClientError> {
    if env::var("OV_URL").is_err() {
        return Ok(());
    }

    if env::var("OV_USERNAME").is_err() {
        return Ok(());
    }

    if env::var("OV_PASSWORD").is_err() {
        return Ok(());
    }

    let client_lock = get_or_initialize_client().await;
    let client_guard = client_lock.lock().await;

    let client = client_guard
        .as_ref()
        .ok_or("OpsviewClient is not initialized")?;

    let status = client.get("/status/host").await?;

    println!("{:#?}", status);

    assert!(false);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn print_hashtags_summary() -> Result<(), OpsviewClientError> {
    if env::var("OV_URL").is_err() {
        return Ok(());
    }

    if env::var("OV_USERNAME").is_err() {
        return Ok(());
    }

    if env::var("OV_PASSWORD").is_err() {
        return Ok(());
    }

    let client_lock = get_or_initialize_client().await;
    let client_guard = client_lock.lock().await;

    let client = client_guard
        .as_ref()
        .ok_or("OpsviewClient is not initialized")?;

    let status = client.get("/status/viewport").await?;

    println!("{:#?}", status);

    assert!(false);

    Ok(())
}
