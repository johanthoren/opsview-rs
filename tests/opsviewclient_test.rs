extern crate opsview;
use once_cell::sync::Lazy;
use opsview::client::OpsviewClient;
use opsview::{config::*, prelude::*, util::*};
use pretty_assertions::assert_eq;
use rand::{distributions::Uniform, Rng};
use std::env;
use std::net::Ipv4Addr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinError;

fn generate_random_ipv4() -> Ipv4Addr {
    let mut rng = rand::thread_rng();
    Ipv4Addr::new(rng.gen(), rng.gen(), rng.gen(), rng.gen())
}
fn generate_random_string() -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
    let range = Uniform::new(0, chars.len());

    (0..10).map(|_| chars[rng.sample(range)]).collect()
}

fn random_name() -> String {
    format!("opsview_rs_test_{}", generate_random_string())
}

// Function to convert JoinError to io::Error
fn handle_join_error(e: JoinError) -> std::io::Error {
    if e.is_cancelled() {
        std::io::Error::new(std::io::ErrorKind::Interrupted, "Task was cancelled")
    } else if e.is_panic() {
        std::io::Error::new(std::io::ErrorKind::Other, "Task panicked")
    } else {
        std::io::Error::new(std::io::ErrorKind::Other, e)
    }
}

async fn bail_on_unsaved_changes(client: &OpsviewClient) -> Result<(), OpsviewClientError> {
    match client.changes_to_apply().await {
        Ok(false) => Ok(()),
        Ok(true) => Err(OpsviewClientError::UnsavedChanges),
        Err(e) => Err(OpsviewClientError::UndefinedError(format!(
            "Failed to check for changes to apply: {}",
            e
        ))),
    }
}

async fn cleanup(client: &OpsviewClient) -> Result<(), OpsviewClientError> {
    let all_hosts = client.get_all_host_configs().await.unwrap();

    assert!(!all_hosts.is_empty());
    println!("all_hosts: {:#?}", all_hosts);

    for host in all_hosts.values() {
        if host
            .name()
            .is_some_and(|n| n.starts_with("opsview_rs_test_"))
        {
            println!("Removing host: {}", host.name().unwrap());
            host.remove(client).await.unwrap();
        }
    }

    let all_hashtags = client.get_all_hashtag_configs().await.unwrap();

    assert!(!all_hashtags.is_empty());

    for hashtag in all_hashtags.values() {
        if hashtag
            .name()
            .is_some_and(|n| n.starts_with("opsview_rs_test_"))
        {
            println!("Removing hashtag: {}", hashtag.name().unwrap());
            hashtag.remove(client).await.unwrap();
        }
    }

    let all_hosttemplates = client.get_all_hosttemplate_configs().await.unwrap();

    assert!(!all_hosttemplates.is_empty());

    for hosttemplate in all_hosttemplates.values() {
        if hosttemplate
            .name()
            .is_some_and(|n| n.starts_with("opsview_rs_test_"))
        {
            println!("Removing host template: {}", hosttemplate.name().unwrap());
            hosttemplate.remove(client).await.unwrap();
        }
    }

    let all_hostgroups = client.get_all_hostgroup_configs().await.unwrap();

    assert!(!all_hostgroups.is_empty());

    for hostgroup in all_hostgroups.values() {
        if hostgroup
            .name()
            .is_some_and(|n| n.starts_with("opsview_rs_test_"))
        {
            println!("Removing host group: {:#?}", hostgroup);
            hostgroup.remove(client).await.unwrap();
        }
    }

    let all_contacts = client.get_all_contact_configs().await.unwrap();

    for contact in all_contacts.values() {
        if contact
            .name()
            .is_some_and(|n| n.starts_with("opsview_rs_test_"))
        {
            println!("Removing contact: {}", contact.name().unwrap());
            contact.remove(client).await.unwrap();
        }
    }

    let all_roles = client.get_all_role_configs().await.unwrap();

    for role in all_roles.values() {
        if role
            .name()
            .is_some_and(|n| n.starts_with("opsview_rs_test_"))
        {
            println!("Removing role: {}", role.name().unwrap());
            role.remove(client).await.unwrap();
        }
    }

    client.apply_changes().await?;

    Ok(())
}

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

#[ignore]
#[tokio::test]
async fn test_get_all_bsmcomponent_configs() -> Result<(), OpsviewClientError> {
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

    let bsmcomponents = client.get_all_bsmcomponent_configs().await?;

    println!("bsmcomponents: {:#?}", bsmcomponents);

    assert!(!bsmcomponents.is_empty());

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_get_all_hosttemplate_configs() -> Result<(), OpsviewClientError> {
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

    let hosttemplates = client.get_all_hosttemplate_configs().await?;

    assert!(!hosttemplates.is_empty());
    println!("hosttemplates: {:#?}", hosttemplates);
    println!("hosttemplates.len(): {}", hosttemplates.len());

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_get_all_host_configs() -> Result<(), OpsviewClientError> {
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

    let hosts = client.get_all_host_configs().await?;

    assert!(!hosts.is_empty());

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_get_all_hosticon_configs() -> Result<(), OpsviewClientError> {
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

    let hosticons = client.get_all_hosticon_configs().await?;

    assert!(!hosticons.is_empty());

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_get_all_contact_configs() -> Result<(), OpsviewClientError> {
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

    let contacts = client.get_all_contact_configs().await?;

    println!("contacts: {:#?}", contacts);

    assert!(!contacts.is_empty());

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_get_single_host() -> Result<(), OpsviewClientError> {
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

    let host = client.get_host_config("Opsview").await?;

    assert_eq!(host.name, "opsview");

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_put_and_delete_single_hashtag() -> Result<(), OpsviewError> {
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

    bail_on_unsaved_changes(client).await?;

    // A random name, 10 characters long
    let random_name = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(10)
        .map(char::from)
        .collect::<String>();

    let hashtag = Hashtag::builder()
        .name(&random_name)
        .description("A random hashtag created by opsview-rs as part of a test")
        .build()?;

    client.put_object_config::<Hashtag>(&hashtag).await?;

    let hashtag = hashtag.fetch(client).await?;

    assert_eq!(hashtag.name, random_name);
    assert!(hashtag.description.is_some());
    assert!(hashtag.id.is_some());

    let delete_response = hashtag.remove(client).await;

    println!("delete_response: {:#?}", delete_response);

    assert!(delete_response.is_ok());

    println!("Applying changes. Please wait...");
    client.apply_changes().await?;
    println!("Done");

    cleanup(client).await?;

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_changes_to_apply() -> Result<(), OpsviewClientError> {
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

    let changes = client.changes_to_apply().await;

    println!("changes: {:#?}", changes);

    assert!(changes.is_ok());

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_get_host_by_id() -> Result<(), OpsviewClientError> {
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

    let host = client.get_host_config("opsview").await?;

    println!("host: {:#?}", host);

    let host_by_id = client.get_host_config_by_id(host.id.unwrap()).await?;

    assert_eq!(host_by_id.name, "opsview");

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_create_and_remove_bsmcomponent() -> Result<(), OpsviewError> {
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

    bail_on_unsaved_changes(client).await?;

    let template = HostTemplate::builder().name("Network - Base").build()?;
    let template = template.fetch(client).await?;

    let host_1 = client.get_host_config("opsview").await?;

    let mut hosts = ConfigObjectMap::<Host>::new();
    hosts.add(host_1);

    let component = BSMComponent::builder()
        .name("Test BSM Component from opsview-rs integration test")
        .host_template(template)
        .hosts(&hosts)
        .quorum_pct("100.00")
        .build()?;

    component.create(client).await?;
    client.apply_changes().await?;

    let existing_components = client.get_all_bsmcomponent_configs().await?;

    for component in existing_components.values() {
        if &component.name == "Test BSM Component from opsview-rs integration test" {
            component.remove(client).await?;
        }
    }

    client.apply_changes().await?;
    cleanup(client).await?;
    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_last_update() -> Result<(), OpsviewClientError> {
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

    let last_updated = client.last_updated().await;

    println!("last_updated: {:#?}", last_updated);

    assert!(last_updated.is_ok());

    assert!(is_valid_past_unix_timestamp(last_updated.unwrap()));

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_host_exists_by_name() -> Result<(), OpsviewError> {
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

    let host = Host::minimal("opsview")?;

    assert!(host.id().is_none());

    let host_exists = host.exists(client).await;

    assert!(host_exists.is_ok());
    assert_eq!(host_exists.unwrap(), true);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_host_exists_by_id() -> Result<(), OpsviewClientError> {
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

    // Get the host by name
    let host = client.get_host_config("opsview").await?;

    assert!(host.id().is_some());

    // Lookup the host, which will use its ID.
    let host_exists = host.exists(client).await;

    assert!(host_exists.is_ok());
    assert_eq!(host_exists.unwrap(), true);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_create_and_delete_new_host_with_deps() -> Result<(), std::io::Error> {
    if env::var("OV_URL").is_err() {
        return Ok(());
    }

    if env::var("OV_USERNAME").is_err() {
        return Ok(());
    }

    if env::var("OV_PASSWORD").is_err() {
        return Ok(());
    }

    let client = OpsviewClient::builder()
        .url(&env::var("OV_URL").expect("OV_URL not set"))
        .username(&env::var("OV_USERNAME").expect("OV_USERNAME not set"))
        .password(&env::var("OV_PASSWORD").expect("OV_PASSWORD not set"))
        .ignore_cert(true)
        .build()
        .await
        .expect("Failed to create OpsviewClient");

    bail_on_unsaved_changes(&client).await.unwrap();

    let client_arc = Arc::new(Mutex::new(client));

    let n = 10;
    let mut handles = Vec::new();

    for _ in 0..n {
        let client_clone = client_arc.clone(); // Clone the Arc for each task
        let handle = tokio::spawn(async move {
            let client = client_clone.lock().await; // Lock the mutex inside the task

            let hashtag = Hashtag::builder().name(&random_name()).build().unwrap();

            let mut hashtag_coll = ConfigObjectMap::<Hashtag>::new();
            hashtag_coll.add(hashtag.clone());

            let hashtag_coll = hashtag_coll;

            let service_group = ServiceGroup::builder()
                .name(&random_name())
                .build()
                .unwrap();

            let service = ServiceCheck::builder()
                .name(&random_name())
                .plugin(
                    Plugin::minimal("check_icmp")
                        .expect("Failed to create minimal Plugin with name check_icmp"),
                )
                .args("")
                .servicegroup(service_group.clone())
                .build()
                .unwrap();

            let mut servicechecks = ConfigObjectMap::<ServiceCheck>::new();
            servicechecks.add(service.clone());

            let servicechecks = servicechecks;

            let hosttemplate = HostTemplate::builder()
                .name(&random_name())
                .servicechecks(&servicechecks)
                .build()
                .unwrap();

            let mut hosttemplate_coll = ConfigObjectMap::<HostTemplate>::new();
            hosttemplate_coll.add(hosttemplate.clone());

            let hosttemplate_coll = hosttemplate_coll;

            let hostgroup = HostGroup::builder()
                .parent(
                    HostGroup::minimal("Opsview")
                        .expect("Failed to create minimal HostGroup with name 'Opsview'"),
                )
                .name(&random_name())
                .build()
                .unwrap();

            let cluster = MonitoringCluster::minimal("Master Monitoring Server").expect(
                "Failed to create minimal MonitoringCluster with name 'Master Monitoring Server",
            );

            let rancid_vendor = RancidVendor::builder()
                .name("AXIS T8504R")
                .ref_("/rest/config/rancidvendor/1234")
                .build()
                .unwrap();

            let host = Host::builder()
                .name(&random_name())
                .ip(generate_random_ipv4().to_string().as_ref())
                .check_command(
                    HostCheckCommand::minimal("ping")
                        .expect("Failed to create minimal HostCheckCommand with name 'ping'"),
                )
                .hostgroup(hostgroup.clone())
                .hashtags(&hashtag_coll)
                .hosttemplates(&hosttemplate_coll)
                .monitored_by(cluster)
                .notification_options("d,u,r")
                .rancid_vendor(rancid_vendor)
                .build()
                .unwrap();

            hashtag_coll.create_all(&client).await.unwrap();

            let api_hashtag = hashtag.fetch(&client).await.unwrap();

            assert_eq!(hashtag.name, api_hashtag.name);
            assert!(api_hashtag.id().is_some());
            assert!(api_hashtag.ref_().is_some());

            service_group.create(&client).await.unwrap();

            let api_service_group = service_group.fetch(&client).await.unwrap();

            assert_eq!(service_group.clone().name, api_service_group.name);
            assert!(api_service_group.id().is_some());
            assert!(api_service_group.ref_().is_some());

            servicechecks.create_all(&client).await.unwrap();

            let api_service = service.fetch(&client).await.unwrap();

            assert_eq!(service.name, api_service.name);
            assert!(api_service.id().is_some());
            assert!(api_service.ref_().is_some());

            hosttemplate_coll.create_all(&client).await.unwrap();

            let api_hosttemplate = hosttemplate.fetch(&client).await.unwrap();

            assert_eq!(hosttemplate.name, api_hosttemplate.name);
            assert!(api_hosttemplate.id().is_some());
            assert!(api_hosttemplate.ref_().is_some());

            hostgroup.create(&client).await.unwrap();

            let api_hostgroup = hostgroup.fetch(&client).await.unwrap();

            assert_eq!(hostgroup.name, api_hostgroup.name);
            assert!(api_hostgroup.id().is_some());
            assert!(api_hostgroup.ref_().is_some());

            host.create(&client).await.unwrap();

            assert!(host.exists(&client).await.unwrap());

            let api_host = host.fetch(&client).await.unwrap();

            assert_eq!(host.name, api_host.name);
            assert!(api_host.id().is_some());
            assert!(api_host.ref_().is_some());

            // api_service.remove(&client).await.unwrap();
            // api_service_group.remove(&client).await.unwrap();
            // api_host.remove(&client).await.unwrap();
            // api_hostgroup.remove(&client).await.unwrap();
            // api_hosttemplate.remove(&client).await.unwrap();
            // api_hashtag.remove(&client).await.unwrap();

            // assert!(!api_host.exists(&client).await.unwrap());
            // assert!(!api_hostgroup.exists(&client).await.unwrap());
            // assert!(!api_hosttemplate.exists(&client).await.unwrap());
            // assert!(!api_hashtag.exists(&client).await.unwrap());

            Ok::<(), std::io::Error>(()) // Return result from the task
        });
        handles.push(handle);
    }

    // Await all tasks to complete and handle potential errors
    for handle in handles {
        match handle.await {
            Ok(result) => result?,                      // Handle the result of the closure
            Err(e) => return Err(handle_join_error(e)), // Handle join error
        }
    }

    let client = OpsviewClient::builder()
        .url(&env::var("OV_URL").expect("OV_URL not set"))
        .username(&env::var("OV_USERNAME").expect("OV_USERNAME not set"))
        .password(&env::var("OV_PASSWORD").expect("OV_PASSWORD not set"))
        .ignore_cert(true)
        .build()
        .await
        .expect("Failed to create OpsviewClient");

    client.apply_changes().await.unwrap();
    cleanup(&client).await.unwrap();
    client.logout().await.unwrap();
    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_update_persistent_method() -> Result<(), OpsviewError> {
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

    bail_on_unsaved_changes(client).await?;

    let test_var_zero = Variable::builder()
        .name("OPSVIEW_RS_TEST_COUNTER")
        .value("0")
        .build()?;

    test_var_zero.update(client).await?;
    let test_var_zero = test_var_zero.fetch(client).await?;

    let mut test_var_one = test_var_zero.clone();
    test_var_one.value = Some("1".to_string());

    let host = Host::minimal("opsview")?;
    let mut host = host.fetch(client).await?;

    host.update_variable(test_var_one);

    //println!("host: {:#?}", host);

    host.update(client).await?;

    let updated_host = host.fetch(client).await?;
    let updated_host_attributes = updated_host.hostattributes.clone().unwrap();

    assert!(updated_host_attributes.contains_name("OPSVIEW_RS_TEST_COUNTER"));
    assert_eq!(
        updated_host_attributes
            .filter_by_name("OPSVIEW_RS_TEST_COUNTER")
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        updated_host_attributes
            .filter_by_name("OPSVIEW_RS_TEST_COUNTER")
            .unwrap()
            .first()
            .unwrap()
            .value
            .as_ref()
            .unwrap(),
        "1"
    );

    let mut new_host_attributes = updated_host_attributes;
    let mut host_without_test_var_one = updated_host;

    new_host_attributes.remove_named("OPSVIEW_RS_TEST_COUNTER");
    host_without_test_var_one.hostattributes = Some(new_host_attributes);
    host_without_test_var_one.update(client).await?;
    test_var_zero.remove(client).await?;

    client.apply_changes().await?;
    cleanup(client).await?;
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_invalid_name() -> Result<(), OpsviewClientError> {
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

    bail_on_unsaved_changes(client).await?;

    let object = Host {
        name: "//foo".to_string(),
        ..Default::default()
    };

    match object.create(client).await {
        Ok(_) => panic!("Expected an error when creating host."),
        Err(e) => {
            assert!(e.to_string().contains("Error trying to create object"));
        }
    }

    cleanup(client).await?;
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_create_and_delete_contact() -> Result<(), OpsviewError> {
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

    bail_on_unsaved_changes(client).await?;

    let role = Role::minimal("opsview_rs_test_role")?;

    let link_1 = ContactLink::builder()
        .name("link_1")
        .url("https://www.example.com")
        .fontawesome_icon(FontAwesomeIcon::FiveHundredPx)
        .build()?;

    let link_2 = ContactLink::builder()
        .name("link_2")
        .url("https://www.youtube.com")
        .fontawesome_icon(FontAwesomeIcon::YoutubeSquare)
        .build()?;

    let mut links = ConfigObjectMap::<ContactLink>::new();

    links.add(link_1);
    links.add(link_2);

    let links = links;

    role.create(client).await?;
    let api_role = role.fetch(client).await?;

    let contact = Contact::builder()
        .name("opsview_rs_test_contact")
        .fullname("Opsview RS Test Contact")
        .role(api_role.clone())
        .mylinks(links)
        .build()?;

    let client = OpsviewClient::builder()
        .url(&env::var("OV_URL").expect("OV_URL not set"))
        .username(&env::var("OV_USERNAME").expect("OV_USERNAME not set"))
        .password(&env::var("OV_PASSWORD").expect("OV_PASSWORD not set"))
        .ignore_cert(true)
        .build()
        .await
        .expect("Failed to create OpsviewClient");

    contact.create(&client).await?;

    let api_contact = contact.fetch(&client).await?;
    api_contact.remove(&client).await?;
    api_role.remove(&client).await?;

    cleanup(&client).await?;

    Ok(())
}
