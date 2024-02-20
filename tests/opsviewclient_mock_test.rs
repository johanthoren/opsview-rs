mod api_responses;
extern crate opsview;
use crate::api_responses::*;
use lazy_static::lazy_static;
use log::{Log, Metadata, Record, SetLoggerError};
use mockito::{Server, ServerGuard};
use opsview::client::OpsviewClient;
use opsview::config::*;
use opsview::prelude::*;
use pretty_assertions::assert_eq;
use std::sync::{Mutex, MutexGuard};

// A simple logger that captures logs to a global buffer
struct TestLogger;
lazy_static! {
    static ref LOGS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

impl Log for TestLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        LOGS.lock().unwrap().push(format!("{}", record.args()));
    }

    fn flush(&self) {}
}

fn setup_logger() -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(TestLogger))
        .map(|()| log::set_max_level(log::LevelFilter::Debug))
}

fn assert_expected_logs(logs: MutexGuard<Vec<String>>) {
    assert_no_error_for_row_count_not_matching_object_count(logs)
}

fn assert_no_error_for_row_count_not_matching_object_count(logs: MutexGuard<Vec<String>>) {
    // Assert that the logs do not contain an error indicating that the total number of rows as
    // listed in the summary does not match the number of objects collected.
    assert!(logs
        .iter()
        .any(|log| !log.contains("Total objects in summary (")));
}

/// Returns a ServerGuard that can be used to mock Opsview API responses.
/// Common responses are already mocked, in this case the login response.
fn setup_mock_server() -> ServerGuard {
    let mut server = Server::new();

    server
        .mock("POST", "/rest/login")
        .with_status(200)
        .with_body(r#"{"token": "some_auth_token"}"#)
        .create();

    server
}

#[tokio::test]
async fn test_get_all_bsmcomponent_configs_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/bsmcomponent")
        .with_status(200)
        .with_body(ALL_BSMCOMPONENT_CONFIGS)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let components = ov.get_all_bsmcomponent_configs().await;

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert!(components.is_ok());

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_bsmservice_config_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/bsmservice?s.name=BSM%201")
        .with_status(200)
        .with_body(BSMSERVICE_CONFIG)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let bsmservice = ov.get_bsmservice_config("BSM 1").await?;
    let component_1 = bsmservice
        .components
        .as_ref()
        .unwrap()
        .get("/rest/config/businesscomponent/1")
        .unwrap();
    let component_2 = bsmservice
        .components
        .unwrap()
        .get("/rest/config/businesscomponent/2")
        .unwrap();

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert_eq!(bsmservice.name, "BSM 1");
    assert_eq!(component_1.name(), "Component 1");
    assert_eq!(component_2.name(), "Component 2");

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_all_bsmservice_configs_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/bsmservice")
        .with_status(200)
        .with_body(ALL_BSMSERVICE_CONFIGS)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let services = ov.get_all_bsmservice_configs().await?;
    let service_1 = services.get("BSM 1-1").unwrap();
    let component_1 = service_1
        .components
        .as_ref()
        .unwrap()
        .get("/rest/config/businesscomponent/1")
        .unwrap();
    let component_2 = service_1
        .components
        .as_ref()
        .unwrap()
        .get("/rest/config/businesscomponent/2")
        .unwrap();

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert!(!services.is_empty());
    assert_eq!(services.len(), 1);
    assert_eq!(service_1.name, "BSM 1");
    assert_eq!(service_1.components.as_ref().unwrap().len(), 2);
    assert_eq!(component_1.name(), "Component 1");
    assert_eq!(component_2.name(), "Component 2");

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_contact_config_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/contact?s.name=admin")
        .with_status(200)
        .with_body(CONTACT_CONFIG)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let contact = ov.get_contact_config("admin").await?;

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert_eq!(contact.name, "admin");
    assert_eq!(contact.id, Some(1));
    assert_eq!(contact.ref_.as_ref().unwrap(), "/rest/config/contact/1");
    assert_eq!(contact.uncommitted, Some(false));
    assert_eq!(
        contact
            .notificationprofiles
            .as_ref()
            .expect("notificationprofiles is None")
            .len(),
        1
    );
    assert_eq!(
        contact
            .notificationprofiles
            .expect("notificationprofiles is None")
            .get("Default")
            .unwrap()
            .ref_
            .as_ref()
            .unwrap(),
        "/rest/config/notificationprofile/1",
    );

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_all_contact_configs_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/contact")
        .with_status(200)
        .with_body(ALL_CONTACT_CONFIGS)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let contacts = ov.get_all_contact_configs().await?;
    let anonymous_guest = contacts
        .get("anonymous-guest")
        .expect("No contact named 'anonymous-guest'");

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert!(!contacts.is_empty());
    assert_eq!(contacts.len(), 3);
    assert_eq!(anonymous_guest.name, "anonymous-guest");
    assert_eq!(
        anonymous_guest.description,
        Some("Anonymous guest".to_string())
    );
    assert_eq!(anonymous_guest.fullname, Some("Guest".to_string()));

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_all_hashtag_configs_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/keyword")
        .with_status(200)
        .with_body(ALL_HASHTAG_CONFIGS)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let hashtags = ov.get_all_hashtag_configs().await?;

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert!(!hashtags.is_empty());

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_host_config_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/host?s.name=opsview")
        .with_status(200)
        .with_body(HOST_CONFIG)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let host = ov.get_host_config("opsview").await?;

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert_eq!(host.name, "opsview");
    assert_eq!(
        host.check_command
            .as_ref()
            .expect("check_command is None")
            .name(),
        "ping"
    );
    assert_eq!(
        host.check_command.expect("check_Command is None").ref_(),
        Some("/rest/config/hostcheckcommand/15".to_string())
    );
    assert_eq!(host.check_interval, Some(300));
    assert_eq!(host.retry_check_interval, Some(60));
    assert_eq!(host.check_attempts, Some(2));

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_all_host_configs_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/host")
        .with_status(200)
        .with_body(ALL_HOST_CONFIGS)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let hosts = ov.get_all_host_configs().await?;
    let host_0 = hosts.get("Amer-Finance-Environment").unwrap();
    let host_48 = hosts.get("opsview").unwrap();

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert!(!hosts.is_empty());
    assert_eq!(hosts.len(), 48);
    assert_eq!(host_0.name, "Amer-Finance-Environment");
    assert_eq!(
        host_0
            .check_command
            .as_ref()
            .expect("check_command is None")
            .name(),
        "ping"
    );
    assert_eq!(
        host_0
            .check_command
            .as_ref()
            .expect("check_command is None")
            .ref_(),
        Some("/rest/config/hostcheckcommand/15".to_string())
    );
    assert_eq!(host_0.check_interval, Some(300));
    assert_eq!(host_0.retry_check_interval, Some(60));
    assert_eq!(host_0.check_attempts, Some(2));
    assert_eq!(
        host_0
            .hostgroup
            .as_ref()
            .expect("hostgroup is None")
            .name()
            .as_str(),
        "192.168.2.88 - VMs"
    );
    assert_eq!(
        host_48
            .hostgroup
            .as_ref()
            .expect("hostgroup is None")
            .ref_(),
        Some("/rest/config/hostgroup/2".to_string())
    );
    assert_eq!(host_48.name, "opsview");
    assert_eq!(
        host_48
            .check_command
            .as_ref()
            .expect("check_command is None")
            .name(),
        "ping"
    );

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_all_hostcheckcommand_configs_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/hostcheckcommand")
        .with_status(200)
        .with_body(ALL_HOSTCHECKCOMMAND_CONFIGS)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let commands = ov.get_all_hostcheckcommand_configs().await?;

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert!(!commands.is_empty());

    assert_expected_logs(logs);

    Ok(())
}

// #[tokio::test]
// async fn test_get_hostgroup_config_mock() -> Result<(), OpsviewClientError> {
//     let mut s = setup_mock_server();

//     s.mock("GET", "/rest/config/hostgroup?s.name=Opsview")
//         .with_status(200)
//         .with_body(HOSTGROUP_CONFIG)
//         .create();

//     let ov = OpsviewClient::new(&s.url(), "username", "password").await?;
//     let hostgroup = ov.get_hostgroup_config("Opsview").await?;

//     assert_eq!(hostgroup.name, "Opsview");

//     Ok(())
// }

#[tokio::test]
async fn test_get_all_hostgroup_configs_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/hostgroup")
        .with_status(200)
        .with_body(ALL_HOSTGROUP_CONFIGS)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let hostgroups = ov.get_all_hostgroup_configs().await?;

    let hostgroup_1 = hostgroups.get("Opsview,").unwrap();

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert!(!hostgroups.is_empty());
    assert_eq!(hostgroups.len(), 14);
    assert_eq!(hostgroup_1.name, "Opsview");
    assert_eq!(hostgroup_1.id, Some(1));
    assert_eq!(
        hostgroup_1.ref_.as_ref().unwrap(),
        "/rest/config/hostgroup/1"
    );

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_all_hosticon_configs_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/hosticons")
        .with_status(200)
        .with_body(ALL_HOSTICON_CONFIGS)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let hosticons = ov.get_all_hosticon_configs().await?;

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert!(!hosticons.is_empty());

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_all_hosttemplate_configs_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/hosttemplate")
        .with_status(200)
        .with_body(ALL_HOSTTEMPLATE_CONFIGS_PAGE_1)
        .create();

    s.mock("GET", "/rest/config/hosttemplate?page=2")
        .with_status(200)
        .with_body(ALL_HOSTTEMPLATE_CONFIGS_PAGE_2)
        .create();

    s.mock("GET", "/rest/config/hosttemplate?page=3")
        .with_status(200)
        .with_body(ALL_HOSTTEMPLATE_CONFIGS_PAGE_3)
        .create();

    s.mock("GET", "/rest/config/hosttemplate?page=4")
        .with_status(200)
        .with_body(ALL_HOSTTEMPLATE_CONFIGS_PAGE_4)
        .create();

    s.mock("GET", "/rest/config/hosttemplate?page=5")
        .with_status(200)
        .with_body(ALL_HOSTTEMPLATE_CONFIGS_PAGE_5)
        .create();

    s.mock("GET", "/rest/config/hosttemplate?page=6")
        .with_status(200)
        .with_body(ALL_HOSTTEMPLATE_CONFIGS_PAGE_6)
        .create();

    let ov = OpsviewClient::new(&s.url(), "username", "password", false).await?;
    let hosttemplates = ov.get_all_hosttemplate_configs().await?;

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert!(!hosttemplates.is_empty());

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_monitoringcluster_config_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/monitoringcluster?s.name=collectors-ny")
        .with_status(200)
        .with_body(MONITORINGCLUSTER_CONFIG)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let cluster = ov.get_monitoringcluster_config("collectors-ny").await?;

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert_eq!(cluster.name, "collectors-ny");
    assert_eq!(cluster.id, Some(2));
    assert_eq!(
        cluster.ref_.as_ref().unwrap(),
        "/rest/config/monitoringcluster/2"
    );

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_all_monitoringcluster_configs_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/monitoringcluster")
        .with_status(200)
        .with_body(ALL_MONITORINGCLUSTER_CONFIGS)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let clusters = ov.get_all_monitoringcluster_configs().await?;
    let cluster_1 = clusters.get("Master Monitoring Server").unwrap();
    let cluster_2 = clusters.get("collectors-ny").unwrap();

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert!(!clusters.is_empty());
    assert_eq!(clusters.len(), 2);
    assert_eq!(cluster_1.name, "Master Monitoring Server");
    assert_eq!(cluster_1.id, Some(1));
    assert_eq!(
        cluster_1.ref_.as_ref().unwrap(),
        "/rest/config/monitoringcluster/1"
    );
    assert_eq!(
        cluster_1.monitors.as_ref().expect("monitors is None").len(),
        1
    );
    assert_eq!(cluster_2.name, "collectors-ny");
    assert_eq!(cluster_2.id, Some(2));
    assert_eq!(
        cluster_2.ref_.as_ref().unwrap(),
        "/rest/config/monitoringcluster/2"
    );
    assert_eq!(
        cluster_1.cloudops_owned.expect("cloudops_owned is None"),
        false
    );
    assert_eq!(
        cluster_2.monitors.as_ref().expect("monitors is None").len(),
        47
    );

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_notificationmethod_config_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/notificationmethod?s.name=Email")
        .with_status(200)
        .with_body(NOTIFICATIONMETHOD_CONFIG)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let method = ov.get_notificationmethod_config("Email").await?;

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert_eq!(method.name, "Email");
    assert_eq!(method.id, Some(3));
    assert_eq!(
        method.ref_.as_ref().unwrap(),
        "/rest/config/notificationmethod/3"
    );
    assert_eq!(method.uncommitted, Some(false));
    assert_eq!(
        method
            .notificationprofiles
            .as_ref()
            .expect("notificationprofiles is None")
            .len(),
        0
    );

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_all_notificationmethod_configs_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/notificationmethod")
        .with_status(200)
        .with_body(ALL_NOTIFICATIONMETHOD_CONFIGS)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let methods = ov.get_all_notificationmethod_configs().await?;
    let method_1 = methods.get("Email").unwrap();
    let method_2 = methods.get("SMS Notification Module").unwrap();

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert!(!methods.is_empty());
    assert_eq!(methods.len(), 20);
    assert_eq!(method_1.name, "Email");
    assert_eq!(method_1.id, Some(3));
    assert_eq!(
        method_1.ref_.as_ref().unwrap(),
        "/rest/config/notificationmethod/3"
    );
    assert_eq!(method_2.name, "SMS Notification Module");
    assert_eq!(method_2.id, Some(2));
    assert_eq!(
        method_2.ref_.as_ref().unwrap(),
        "/rest/config/notificationmethod/2"
    );

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_all_plugin_configs_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/plugin")
        .with_status(200)
        .with_body(ALL_PLUGIN_CONFIGS_PAGE_1)
        .create();

    s.mock("GET", "/rest/config/plugin?page=2")
        .with_status(200)
        .with_body(ALL_PLUGIN_CONFIGS_PAGE_2)
        .create();

    s.mock("GET", "/rest/config/plugin?page=3")
        .with_status(200)
        .with_body(ALL_PLUGIN_CONFIGS_PAGE_3)
        .create();

    s.mock("GET", "/rest/config/plugin?page=4")
        .with_status(200)
        .with_body(ALL_PLUGIN_CONFIGS_PAGE_4)
        .create();

    s.mock("GET", "/rest/config/plugin?page=5")
        .with_status(200)
        .with_body(ALL_PLUGIN_CONFIGS_PAGE_5)
        .create();

    s.mock("GET", "/rest/config/plugin?page=6")
        .with_status(200)
        .with_body(ALL_PLUGIN_CONFIGS_PAGE_6)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let plugins = ov.get_all_plugin_configs().await?;

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert!(!plugins.is_empty());

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_role_config_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/role?s.name=Administrator")
        .with_status(200)
        .with_body(ROLE_CONFIG)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let role = ov.get_role_config("Administrator").await?;

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert_eq!(role.name, "Administrator");
    assert_eq!(role.id, Some(10));
    assert_eq!(role.ref_.as_ref().unwrap(), "/rest/config/role/10");
    assert_eq!(role.uncommitted, Some(false));

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_all_role_configs_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/role")
        .with_status(200)
        .with_body(ALL_ROLE_CONFIGS)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let roles = ov.get_all_role_configs().await?;

    println!("{:#?}", roles);

    let role_1 = roles.get("Administrator").unwrap();
    let role_2 = roles.get("Anonymous Guest").unwrap();

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert!(!roles.is_empty());
    assert_eq!(roles.len(), 9);
    assert_eq!(role_1.name, "Administrator");
    assert_eq!(role_1.description, Some("Administrator access".to_string()));
    assert_eq!(role_1.id, Some(10));
    assert!(role_1
        .accesses
        .as_ref()
        .expect("accesses is None")
        .contains(&Access::ActionAll(Some(
            "/rest/config/access/3".to_string()
        ))));
    assert_eq!(role_1.ref_.as_ref().unwrap(), "/rest/config/role/10");
    assert_eq!(role_2.name, "Anonymous Guest");
    assert_eq!(role_2.id, Some(16));
    assert_eq!(role_2.ref_.as_ref().unwrap(), "/rest/config/role/16");

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_all_servicegroup_configs_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/servicegroup")
        .with_status(200)
        .with_body(ALL_SERVICEGROUP_CONFIGS_PAGE_1)
        .create();

    s.mock("GET", "/rest/config/servicegroup?page=2")
        .with_status(200)
        .with_body(ALL_SERVICEGROUP_CONFIGS_PAGE_2)
        .create();

    s.mock("GET", "/rest/config/servicegroup?page=3")
        .with_status(200)
        .with_body(ALL_SERVICEGROUP_CONFIGS_PAGE_3)
        .create();

    s.mock("GET", "/rest/config/servicegroup?page=4")
        .with_status(200)
        .with_body(ALL_SERVICEGROUP_CONFIGS_PAGE_4)
        .create();

    s.mock("GET", "/rest/config/servicegroup?page=5")
        .with_status(200)
        .with_body(ALL_SERVICEGROUP_CONFIGS_PAGE_5)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let servicegroups = ov.get_all_servicegroup_configs().await?;

    let servicegroup_1 = servicegroups
        .get("Application - Active Directory - Address Book")
        .unwrap();

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert!(!servicegroups.is_empty());
    assert_eq!(servicegroups.len(), 215);
    assert_eq!(
        servicegroup_1.name,
        "Application - Active Directory - Address Book"
    );
    assert_eq!(servicegroup_1.id, Some(136));
    assert_eq!(
        servicegroup_1.ref_.as_ref().unwrap(),
        "/rest/config/servicegroup/136"
    );

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_all_servicecheck_configs_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/servicecheck")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_1)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=2")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_2)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=3")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_3)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=4")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_4)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=5")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_5)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=6")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_6)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=7")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_7)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=8")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_8)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=9")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_9)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=10")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_10)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=11")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_11)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=12")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_12)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=13")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_13)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=14")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_14)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=15")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_15)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=16")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_16)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=17")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_17)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=18")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_18)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=19")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_19)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=20")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_20)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=21")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_21)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=22")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_22)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=23")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_23)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=24")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_24)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=25")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_25)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=26")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_26)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=27")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_27)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=28")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_28)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=29")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_29)
        .create();

    s.mock("GET", "/rest/config/servicecheck?page=30")
        .with_status(200)
        .with_body(ALL_SERVICECHECK_CONFIGS_PAGE_30)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let servicechecks = ov.get_all_servicecheck_configs().await?;

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert!(!servicechecks.is_empty());

    let example_service_1 = servicechecks.get("ACI - APIC - CPU Usage").unwrap();

    assert_eq!(example_service_1.name, "ACI - APIC - CPU Usage");
    assert_eq!(
        example_service_1.stale_state,
        Some(ServiceCheckState::Unknown)
    );

    let example_service_2 = servicechecks.get("Uptime Restart").unwrap();

    assert_eq!(example_service_2.name, "Uptime Restart");
    assert_eq!(example_service_2.stale_state, Some(ServiceCheckState::Ok));

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_sharednotificationprofile_config_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock(
        "GET",
        "/rest/config/sharednotificationprofile?s.name=Receive%20all%20alerts%20during%20work%20hours",
    )
    .with_status(200)
    .with_body(SHAREDNOTIFICATIONPROFILE_CONFIG)
    .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let profile = ov
        .get_sharednotificationprofile_config("Receive all alerts during work hours")
        .await?;

    println!("{:#?}", profile);

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert_eq!(profile.name, "Receive all alerts during work hours");
    assert_eq!(profile.id, Some(2));
    assert_eq!(
        profile.ref_.as_ref().unwrap(),
        "/rest/config/sharednotificationprofile/2"
    );
    assert_eq!(profile.uncommitted, Some(false));
    assert_eq!(
        profile
            .notificationmethods
            .as_ref()
            .expect("notificationmethods is None")
            .len(),
        0
    );

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_all_sharednotificationprofile_configs_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/sharednotificationprofile")
        .with_status(200)
        .with_body(ALL_SHAREDNOTIFICATIONPROFILE_CONFIGS)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let profiles = ov.get_all_sharednotificationprofile_configs().await?;
    let profile_1 = profiles
        .get("Receive all alerts during work hours")
        .unwrap();

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert!(!profiles.is_empty());
    assert_eq!(profiles.len(), 1);
    assert_eq!(profile_1.name, "Receive all alerts during work hours");
    assert_eq!(profile_1.id, Some(2));
    assert_eq!(
        profile_1.ref_.as_ref().unwrap(),
        "/rest/config/sharednotificationprofile/2"
    );

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_all_tenancy_configs_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/tenancy")
        .with_status(200)
        .with_body(ALL_TENANCY_CONFIGS)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let tenancies = ov.get_all_tenancy_configs().await?;

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert!(!tenancies.is_empty());

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_all_timeperiod_configs_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/timeperiod")
        .with_status(200)
        .with_body(ALL_TIMEPERIOD_CONFIGS)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let timeperiods = ov.get_all_timeperiod_configs().await?;

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert!(!timeperiods.is_empty());

    assert_expected_logs(logs);

    Ok(())
}

#[tokio::test]
async fn test_get_all_variable_configs_mock() -> Result<(), OpsviewClientError> {
    let _ = setup_logger();
    let mut s = setup_mock_server();

    s.mock("GET", "/rest/config/attribute")
        .with_status(200)
        .with_body(ALL_VARIABLE_CONFIGS_PAGE_1)
        .create();

    s.mock("GET", "/rest/config/attribute?page=2")
        .with_status(200)
        .with_body(ALL_VARIABLE_CONFIGS_PAGE_2)
        .create();

    s.mock("GET", "/rest/config/attribute?page=3")
        .with_status(200)
        .with_body(ALL_VARIABLE_CONFIGS_PAGE_3)
        .create();

    s.mock("GET", "/rest/config/attribute?page=4")
        .with_status(200)
        .with_body(ALL_VARIABLE_CONFIGS_PAGE_4)
        .create();

    let ov = OpsviewClient::builder()
        .url(&s.url())
        .username("username")
        .password("password")
        .ignore_cert(false)
        .build()
        .await?;

    let variables = ov.get_all_variable_configs().await?;

    let logs = LOGS.lock().unwrap();
    println!("{:#?}", logs);

    assert!(!variables.is_empty());

    assert_expected_logs(logs);

    Ok(())
}
