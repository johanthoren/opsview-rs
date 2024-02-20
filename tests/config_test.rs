use opsview::{config::*, prelude::*};
use pretty_assertions::assert_eq;

// Example BSMComponent from the Opsview API:
// {'has_icon': '0',
//  'host_template': {'name': 'Network - Base',
//                    'ref': '/rest/config/hosttemplate/117'},
//  'host_template_id': '117',
//  'hosts': [{'name': 'GeneosGatewayLab01', 'ref': '/rest/config/host/341'},
//            {'name': 'IP-Switch-1', 'ref': '/rest/config/host/338'},
//            {'name': 'lnux100', 'ref': '/rest/config/host/339'}],
//  'id': '1',
//  'name': 'Component 1',
//  'quorum_pct': '66.67',
//  'ref': '/rest/config/bsmcomponent/1',
//  'uncommitted': '0'}
#[test]
fn test_bsmcomponent() -> Result<(), OpsviewError> {
    let cluster_1 = MonitoringCluster::minimal("Cluster 1")
        .expect("Failed to create cluster with name 'Cluster 1'");

    let check_icmp = Plugin::builder().name("check_icmp").build()?;

    let root_hostgroup = HostGroup::builder()
        .name("Opsview")
        .clear_parent()
        .build()?;

    // let hostgroup_1 = HostGroup::builder()
    //     .name("Linux Servers".to_string())
    //     .build()?;

    let ping = HostCheckCommand::builder()
        .name("ping")
        .args("-H $HOSTADDRESS$ -t 3 -w 500.0,80% -c 1000.0,100%")
        .plugin(check_icmp)
        .build()?;

    let template = HostTemplate::builder().name("Template").build().unwrap();

    let mut templates = ConfigObjectMap::<HostTemplate>::new();
    templates.add(template.clone());

    let host_1 = Host::builder()
        .name("GeneosGatewayLab01")
        .ip("127.0.0.1")
        .hosttemplates(&templates)
        .check_command(ping)
        .hostgroup(root_hostgroup)
        .monitored_by(cluster_1)
        .build()?;

    // let host_2 = Host::builder()
    //     .name("IP-Switch-1".to_string())
    //     .ip("127.0.0.1".to_string())
    //     .check_command(ping.clone())
    //     .hostgroup(root_hostgroup.clone())
    //     .monitored_by(cluster_1.clone())
    //     .build()?;

    // let host_3 = Host::builder()
    //     .name("lnux100".to_string())
    //     .ip("127.0.0.1".to_string())
    //     .check_command(ping)
    //     .hostgroup(root_hostgroup)
    //     .monitored_by(cluster_1)
    //     .build()?;

    let mut hosts = ConfigObjectMap::<Host>::new();
    hosts.add(host_1);

    let bsm_component_1 = BSMComponent::builder()
        .name("Component 1")
        .host_template(template)
        .host_template_id(117)
        .hosts(&hosts)
        .quorum_pct("100.00")
        .build()?;

    let serialized_1 = serde_json::to_string(&bsm_component_1)?;

    assert_eq!(
        serialized_1,
        r#"{"name":"Component 1","host_template":{"name":"Template"},"host_template_id":117,"hosts":[{"name":"GeneosGatewayLab01"}],"quorum_pct":"100.00"}"#,
    );

    let deserialized_1 = serde_json::from_str::<BSMComponent>(&serialized_1);
    assert!(deserialized_1.is_ok());

    Ok(())
}
