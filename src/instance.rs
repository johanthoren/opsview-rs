use crate::client::OpsviewClient;
use crate::{config::*, prelude::*};
use serde::{Deserialize, Serialize};
use tokio::join;

/// The `OpsviewInstance` struct represents all the configuration object of an entire Opsview
/// instance.
///
/// This struct is used to represent the entire configuration of an Opsview instance at once. When
/// synced using the `refresh` method, it contains all the configuration objects that can be managed
/// via the Opsview API for a given instance.
///
/// Typical use cases for this could be to compare the configuration of an Opsview instance to
/// another instance, or to a set of objects from a CMDB, or applying some other computational
/// operation best done offline rather than as a series of API calls to the actual client.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpsviewInstance {
    /// A collection of all the BSM Components in the Opsview instance.
    pub bsm_components: ConfigObjectMap<BSMComponent>,

    /// A collection of all the BSM Services in the Opsview instance.
    pub bsm_services: ConfigObjectMap<BSMService>,

    /// A collection of all the Contacts in the Opsview instance.
    pub contacts: ConfigObjectMap<Contact>,

    /// A collection of all the Host Check Commands in the Opsview instance.
    pub host_check_commands: ConfigObjectMap<HostCheckCommand>,

    /// A collection of all the Host Groups in the Opsview instance.
    pub host_groups: ConfigObjectMap<HostGroup>,

    /// A collection of all the Host Templates in the Opsview instance.
    pub host_templates: ConfigObjectMap<HostTemplate>,

    /// A collection of all the Hosts in the Opsview instance.
    pub hosts: ConfigObjectMap<Host>,

    /// A collection of all the Monitoring Clusters in the Opsview instance.
    pub monitoring_clusters: ConfigObjectMap<MonitoringCluster>,

    /// A collection of all the Netflow Collectors in the Opsview instance.
    pub netflow_collectors: ConfigObjectMap<NetflowCollector>,

    /// A collection of all the Netflow Sources in the Opsview instance.
    pub netflow_sources: ConfigObjectMap<NetflowSource>,

    /// A collection of all the Notification Methods in the Opsview instance.
    pub notification_methods: ConfigObjectMap<NotificationMethod>,

    /// A collection of all the Plugins in the Opsview instance.
    pub plugins: ConfigObjectMap<Plugin>,

    /// A collection of all the Service Checks in the Opsview instance.
    pub service_checks: ConfigObjectMap<ServiceCheck>,

    /// A collection of all the Service Groups in the Opsview instance.
    pub service_groups: ConfigObjectMap<ServiceGroup>,

    /// A collection of all the Shared Notification Profiles in the Opsview instance.
    pub shared_notification_profiles: ConfigObjectMap<SharedNotificationProfile>,

    /// A collection of all the Tenancies in the Opsview instance.
    pub tenancies: ConfigObjectMap<Tenancy>,

    /// A collection of all the Time Periods in the Opsview instance.
    pub time_periods: ConfigObjectMap<TimePeriod>,

    /// A collection of all the Variables in the Opsview instance.
    pub variables: ConfigObjectMap<Variable>,
}

/// Defines the default values for an `OpsviewInstance` object.
impl Default for OpsviewInstance {
    fn default() -> Self {
        OpsviewInstance {
            bsm_components: ConfigObjectMap::<BSMComponent>::new(),
            bsm_services: ConfigObjectMap::<BSMService>::new(),
            contacts: ConfigObjectMap::<Contact>::new(),
            host_check_commands: ConfigObjectMap::<HostCheckCommand>::new(),
            host_groups: ConfigObjectMap::<HostGroup>::new(),
            host_templates: ConfigObjectMap::<HostTemplate>::new(),
            hosts: ConfigObjectMap::<Host>::new(),
            monitoring_clusters: ConfigObjectMap::<MonitoringCluster>::new(),
            netflow_collectors: ConfigObjectMap::<NetflowCollector>::new(),
            netflow_sources: ConfigObjectMap::<NetflowSource>::new(),
            notification_methods: ConfigObjectMap::<NotificationMethod>::new(),
            plugins: ConfigObjectMap::<Plugin>::new(),
            service_checks: ConfigObjectMap::<ServiceCheck>::new(),
            service_groups: ConfigObjectMap::<ServiceGroup>::new(),
            shared_notification_profiles: ConfigObjectMap::<SharedNotificationProfile>::new(),
            tenancies: ConfigObjectMap::<Tenancy>::new(),
            time_periods: ConfigObjectMap::<TimePeriod>::new(),
            variables: ConfigObjectMap::<Variable>::new(),
        }
    }
}

impl OpsviewInstance {
    /// Refreshes the `OpsviewInstance` object with the latest configuration from the Opsview
    /// instance.
    ///
    /// # Arguments
    /// * `client` - The `OpsviewClient` object used to communicate with the Opsview API.
    ///
    /// # Returns
    /// A Result containing the updated `OpsviewInstance` object, or an error if the refresh failed.
    ///
    /// # Caution
    /// This method may cause a high load on the Opsview instance, and should be used sparingly.
    pub async fn refresh(
        self,
        client: &OpsviewClient,
    ) -> Result<OpsviewInstance, OpsviewClientError> {
        let bsm_components_future = client.get_all_bsmcomponent_configs(None);
        let bsm_services_future = client.get_all_bsmservice_configs(None);
        let contacts_future = client.get_all_contact_configs(None);
        let host_check_commands_future = client.get_all_hostcheckcommand_configs(None);
        let host_groups_future = client.get_all_hostgroup_configs(None);
        let host_templates_future = client.get_all_hosttemplate_configs(None);
        let hosts_future = client.get_all_host_configs(None);
        let monitoring_clusters_future = client.get_all_monitoringcluster_configs(None);
        let netflow_collectors_future = client.get_all_netflowcollector_configs(None);
        let netflow_sources_future = client.get_all_netflowsource_configs(None);
        let notification_methods_future = client.get_all_notificationmethod_configs(None);
        let plugins_future = client.get_all_plugin_configs(None);
        let service_checks_future = client.get_all_servicecheck_configs(None);
        let service_groups_future = client.get_all_servicegroup_configs(None);
        let shared_notification_profiles_future =
            client.get_all_sharednotificationprofile_configs(None);
        let tenancies_future = client.get_all_tenancy_configs(None);
        let time_periods_future = client.get_all_timeperiod_configs(None);
        let variables_future = client.get_all_variable_configs(None);

        let (
            bsm_components,
            bsm_services,
            contacts,
            host_check_commands,
            host_groups,
            host_templates,
            hosts,
            monitoring_clusters,
            netflow_collectors,
            netflow_sources,
            notification_methods,
            plugins,
            service_checks,
            service_groups,
            shared_notification_profiles,
            tenancies,
            time_periods,
            variables,
        ) = join!(
            bsm_components_future,
            bsm_services_future,
            contacts_future,
            host_check_commands_future,
            host_groups_future,
            host_templates_future,
            hosts_future,
            monitoring_clusters_future,
            netflow_collectors_future,
            netflow_sources_future,
            notification_methods_future,
            plugins_future,
            service_checks_future,
            service_groups_future,
            shared_notification_profiles_future,
            tenancies_future,
            time_periods_future,
            variables_future,
        );

        Ok(OpsviewInstance {
            bsm_components: bsm_components.unwrap(),
            bsm_services: bsm_services.unwrap(),
            contacts: contacts.unwrap(),
            host_check_commands: host_check_commands.unwrap(),
            host_groups: host_groups.unwrap(),
            host_templates: host_templates.unwrap(),
            hosts: hosts.unwrap(),
            monitoring_clusters: monitoring_clusters.unwrap(),
            netflow_collectors: netflow_collectors.unwrap(),
            netflow_sources: netflow_sources.unwrap(),
            notification_methods: notification_methods.unwrap(),
            plugins: plugins.unwrap(),
            service_checks: service_checks.unwrap(),
            service_groups: service_groups.unwrap(),
            shared_notification_profiles: shared_notification_profiles.unwrap(),
            tenancies: tenancies.unwrap(),
            time_periods: time_periods.unwrap(),
            variables: variables.unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test_refresh() -> Result<(), OpsviewClientError> {
        use std::env;
        if env::var("OV_URL").is_err() {
            return Ok(());
        }

        if env::var("OV_USERNAME").is_err() {
            return Ok(());
        }

        if env::var("OV_PASSWORD").is_err() {
            return Ok(());
        }

        let start_time = std::time::Instant::now();

        let client = OpsviewClient::builder()
            .url(std::env::var("OV_URL").unwrap().as_ref())
            .username(std::env::var("OV_USERNAME").unwrap().as_ref())
            .password(std::env::var("OV_PASSWORD").unwrap().as_ref())
            .ignore_cert(true)
            .build()
            .await?;

        let instance = OpsviewInstance::default();
        let instance = instance.refresh(&client).await?;

        // assert!(!instance.bsm_components.is_empty());
        // assert!(!instance.bsm_services.is_empty());
        // assert!(!instance.contacts.is_empty());
        // assert!(!instance.host_check_commands.is_empty());
        // assert!(!instance.host_groups.is_empty());
        // assert!(!instance.host_templates.is_empty());
        // assert!(!instance.hosts.is_empty());
        // assert!(!instance.monitoring_clusters.is_empty());
        // assert!(!instance.notification_methods.is_empty());
        // assert!(!instance.netflow_collectors.is_empty());
        // assert!(!instance.netflow_sources.is_empty());
        // assert!(!instance.plugins.is_empty());
        // assert!(!instance.service_checks.is_empty());
        // assert!(!instance.service_groups.is_empty());
        // assert!(!instance.shared_notification_profiles.is_empty());
        // assert!(!instance.tenancies.is_empty());
        // assert!(!instance.time_periods.is_empty());
        // assert!(!instance.variables.is_empty());

        // println!("{:#?}", instance);

        println!("OpsviewInstance data:");
        println!("BSM Components: {}", instance.bsm_components.len());
        println!("BSM Services: {}", instance.bsm_services.len());
        println!("Contacts: {}", instance.contacts.len());
        println!(
            "Host Check Commands: {}",
            instance.host_check_commands.len()
        );
        println!("Host Groups: {}", instance.host_groups.len());
        println!("Host Templates: {}", instance.host_templates.len());
        println!("Hosts: {}", instance.hosts.len());
        println!(
            "Monitoring Clusters: {}",
            instance.monitoring_clusters.len()
        );
        println!("Netflow Collectors: {}", instance.netflow_collectors.len());
        println!("Netflow Sources: {}", instance.netflow_sources.len());
        println!(
            "Notification Methods: {}",
            instance.notification_methods.len()
        );
        println!("Plugins: {}", instance.plugins.len());
        println!("Service Checks: {}", instance.service_checks.len());
        println!("Service Groups: {}", instance.service_groups.len());
        println!(
            "Shared Notification Profiles: {}",
            instance.shared_notification_profiles.len()
        );
        println!("Tenancies: {}", instance.tenancies.len());
        println!("Time Periods: {}", instance.time_periods.len());
        println!("Variables: {}", instance.variables.len());

        let elapsed = start_time.elapsed();

        println!("Elapsed: {:?}", elapsed);

        client.logout().await?;

        Ok(())
    }
}
