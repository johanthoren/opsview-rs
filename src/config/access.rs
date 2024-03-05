use serde::de;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents an Access entity in Opsview.
///
/// The `Access` enum is used to define access control settings in the Opsview system.
/// It encapsulates the necessary information to identify and describe an access control entity.
/// # Available access points
/// (according to [version 6.8.9 of the Opsview documentation](https://docs.itrsgroup.com/docs/opsview/6.8.9/configuration/users-and-roles/roles/index.html#current-role-definitions))
/// * VIEWALL — view all (AC). This will also include all Business Services and Business Components.
/// * VIEWSOME — view some (AC) - see below for the definition of some.
/// * ACTIONALL — action all (AC).
/// * ACTIONSOME — action some (AC) - see below for the definition of some. Action ability includes: setting acknowledgments, editing the built-in wiki. Setting downtime requires DOWNTIMESOME.
/// * DOWNTIMEALL — see DOWNTIMESOME (AC).
/// * DOWNTIMESOME — set downtime for their list of objects. See below for the definition of some (AC).
/// * TESTALL — run the Test Service Check function.
/// * TESTSOME — as TESTALL, but see below for the definition of some.
/// * TESTCHANGE — run the Test Service Check function and have the ability to change the arguments for troubleshooting.
/// * DASHBOARD — allow access to the dashboard.
/// * DASHBOARDEDIT — allow the user to make private changes to their dashboard.
/// * VIEWPORTACCESS — viewport access.
/// * RRDGRAPHS — RRD graphs. If RRD graphs are set to public, then /graph and /rrdgraph will be available to non-logged in users. They will also be allowed to view all hosts and all services. If RRD graphs are set to authenticated users, then the hosts and services allowed to be accessed will be restricted to the subset of the host group and service group intersection.
/// * NOTIFYSOME — notify some (AC) - see below for the definition of some.
/// * CONFIGUREHOSTS — view configuration for hosts. You choose which points in the Host group hierarchy this role has, which means only hosts within those host groups are allowed to be configured. To be able to configure all hosts, select the top level host group. If you select any monitoring servers, you are only allowed to mark hosts against these particular monitoring servers.
/// * CONFIGUREKEYWORDS — view configuration for Hashtags (formerly called Keywords). A user with this access would be able to get a list of all Service Checks, Hosts, and Contacts for the system.
/// * CONFIGUREPROFILES — view configuration for shared notification profiles for their role. If the user also has ADMINACCESS, they can see profiles for all roles (AO).
/// * CONFIGUREHOSTGROUPS
/// * CONFIGURECONTACTS
/// * CONFIGUREROLES (AO)
/// * CONFIGURETENANCIES
/// * CONFIGURENETFLOW — configure NetFlow.
/// * CONFIGUREVIEW — view configuration of everything else that does not have its own access point above. As new access points are created, less will be covered by CONFIGUREVIEW.
/// * CONFIGUREREMOTECLUSTER — add new remotely managed clusters from the Configuration > Collector Management page. This must also have CONFIGUREVIEW permission.
/// * CONFIGURESAVE — save configuration changes. Removing this access effectively gives a view only ability to look at the configuration data (some passwords will be visible).
/// * RELOADVIEW — displays the number of changes badge in the navigation bar and can open the Apply Changes window from the Configuration menu.
/// * RELOADACCESS — can start the Apply Changes process from the Apply Changes window to put the latest configuration into production.
/// * ADMINACCESS — admin access, including audit log access.
/// * REPORTUSER — access to Opsview Reporting Module.
/// * REPORTADMIN — allow administrator access in Opsview Reporting Module.
/// * NETFLOW — ability to view the NetFlow dashlets.
/// * PASSWORDSAVE — ability to change their own password.
/// * REMOTELYMANAGEDCLUSTERS — add collectors to remotely managed clusters.
///
/// Some additional access points have since then been found in responses from the Opsview API:
/// * BSM
/// * CONFIGUREBSM
/// * CONFIGUREBSMCOMPONENT
/// * DASHBOARDSHARE
/// * NAVOPTIONS
/// * NETAUDITVIEW
/// * NTVIEWALL
#[allow(missing_docs)]
#[non_exhaustive] // New access control settings may be added in future releases of Opsview.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Access {
    ActionAll(Option<String>),
    ActionSome(Option<String>),
    AdminAccess(Option<String>),
    BSM(Option<String>),
    ConfigureBSM(Option<String>),
    ConfigureBSMComponent(Option<String>),
    ConfigureContacts(Option<String>),
    ConfigureHostGroups(Option<String>),
    ConfigureHosts(Option<String>),
    ConfigureKeywords(Option<String>),
    ConfigureNetFlow(Option<String>),
    ConfigureProfiles(Option<String>),
    ConfigureRemoteCluster(Option<String>),
    ConfigureRoles(Option<String>),
    ConfigureSave(Option<String>),
    ConfigureTenancies(Option<String>),
    ConfigureView(Option<String>),
    Dashboard(Option<String>),
    DashboardEdit(Option<String>),
    DashboardShare(Option<String>),
    DowntimeAll(Option<String>),
    DowntimeSome(Option<String>),
    NavOptions(Option<String>),
    NetAuditView(Option<String>),
    NetFlow(Option<String>),
    NotifySome(Option<String>),
    NTViewAll(Option<String>),
    PasswordSave(Option<String>),
    ReloadAccess(Option<String>),
    ReloadView(Option<String>),
    RemotelyManagedClusters(Option<String>),
    ReportAdmin(Option<String>),
    ReportUser(Option<String>),
    RrdGraphs(Option<String>),
    TestAll(Option<String>),
    TestChange(Option<String>),
    TestSome(Option<String>),
    ViewAll(Option<String>),
    ViewPortAccess(Option<String>),
    ViewSome(Option<String>),
}

impl Serialize for Access {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (name, ref_) = match self {
            Access::ActionAll(ref_) => ("ACTIONALL", ref_),
            Access::ActionSome(ref_) => ("ACTIONSOME", ref_),
            Access::AdminAccess(ref_) => ("ADMINACCESS", ref_),
            Access::BSM(ref_) => ("BSM", ref_),
            Access::ConfigureBSM(ref_) => ("CONFIGUREBSM", ref_),
            Access::ConfigureBSMComponent(ref_) => ("CONFIGUREBSMCOMPONENT", ref_),
            Access::ConfigureContacts(ref_) => ("CONFIGURECONTACTS", ref_),
            Access::ConfigureHostGroups(ref_) => ("CONFIGUREHOSTGROUPS", ref_),
            Access::ConfigureHosts(ref_) => ("CONFIGUREHOSTS", ref_),
            Access::ConfigureKeywords(ref_) => ("CONFIGUREKEYWORDS", ref_),
            Access::ConfigureNetFlow(ref_) => ("CONFIGURENETFLOW", ref_),
            Access::ConfigureProfiles(ref_) => ("CONFIGUREPROFILES", ref_),
            Access::ConfigureRemoteCluster(ref_) => ("CONFIGUREREMOTECLUSTER", ref_),
            Access::ConfigureRoles(ref_) => ("CONFIGUREROLES", ref_),
            Access::ConfigureSave(ref_) => ("CONFIGURESAVE", ref_),
            Access::ConfigureTenancies(ref_) => ("CONFIGURETENANCIES", ref_),
            Access::ConfigureView(ref_) => ("CONFIGUREVIEW", ref_),
            Access::Dashboard(ref_) => ("DASHBOARD", ref_),
            Access::DashboardEdit(ref_) => ("DASHBOARDEDIT", ref_),
            Access::DashboardShare(ref_) => ("DASHBOARDSHARE", ref_),
            Access::DowntimeAll(ref_) => ("DOWNTIMEALL", ref_),
            Access::DowntimeSome(ref_) => ("DOWNTIMESOME", ref_),
            Access::NavOptions(ref_) => ("NAVOPTIONS", ref_),
            Access::NetAuditView(ref_) => ("NETAUDITVIEW", ref_),
            Access::NetFlow(ref_) => ("NETFLOW", ref_),
            Access::NotifySome(ref_) => ("NOTIFYSOME", ref_),
            Access::NTViewAll(ref_) => ("NTVIEWALL", ref_),
            Access::PasswordSave(ref_) => ("PASSWORDSAVE", ref_),
            Access::ReloadAccess(ref_) => ("RELOADACCESS", ref_),
            Access::ReloadView(ref_) => ("RELOADVIEW", ref_),
            Access::RemotelyManagedClusters(ref_) => ("REMOTELYMANAGEDCLUSTERS", ref_),
            Access::ReportAdmin(ref_) => ("REPORTADMIN", ref_),
            Access::ReportUser(ref_) => ("REPORTUSER", ref_),
            Access::RrdGraphs(ref_) => ("RRDGRAPHS", ref_),
            Access::TestAll(ref_) => ("TESTALL", ref_),
            Access::TestChange(ref_) => ("TESTCHANGE", ref_),
            Access::TestSome(ref_) => ("TESTSOME", ref_),
            Access::ViewAll(ref_) => ("VIEWALL", ref_),
            Access::ViewPortAccess(ref_) => ("VIEWPORTACCESS", ref_),
            Access::ViewSome(ref_) => ("VIEWSOME", ref_),
        };

        let mut state = serializer.serialize_struct("Access", 2)?;
        state.serialize_field("name", name)?;
        if let Some(ref ref_value) = ref_ {
            state.serialize_field("ref", ref_value)?;
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for Access {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            name: String,
            #[serde(rename = "ref")]
            ref_: Option<String>,
        }

        let helper = Helper::deserialize(deserializer)?;

        match helper.name.as_str() {
            "ACTIONALL" => Ok(Access::ActionAll(helper.ref_)),
            "ACTIONSOME" => Ok(Access::ActionSome(helper.ref_)),
            "ADMINACCESS" => Ok(Access::AdminAccess(helper.ref_)),
            "BSM" => Ok(Access::BSM(helper.ref_)),
            "CONFIGUREBSM" => Ok(Access::ConfigureBSM(helper.ref_)),
            "CONFIGUREBSMCOMPONENT" => Ok(Access::ConfigureBSMComponent(helper.ref_)),
            "CONFIGURECONTACTS" => Ok(Access::ConfigureContacts(helper.ref_)),
            "CONFIGUREHOSTGROUPS" => Ok(Access::ConfigureHostGroups(helper.ref_)),
            "CONFIGUREHOSTS" => Ok(Access::ConfigureHosts(helper.ref_)),
            "CONFIGUREKEYWORDS" => Ok(Access::ConfigureKeywords(helper.ref_)),
            "CONFIGURENETFLOW" => Ok(Access::ConfigureNetFlow(helper.ref_)),
            "CONFIGUREPROFILES" => Ok(Access::ConfigureProfiles(helper.ref_)),
            "CONFIGUREREMOTECLUSTER" => Ok(Access::ConfigureRemoteCluster(helper.ref_)),
            "CONFIGUREROLES" => Ok(Access::ConfigureRoles(helper.ref_)),
            "CONFIGURESAVE" => Ok(Access::ConfigureSave(helper.ref_)),
            "CONFIGURETENANCIES" => Ok(Access::ConfigureTenancies(helper.ref_)),
            "CONFIGUREVIEW" => Ok(Access::ConfigureView(helper.ref_)),
            "DASHBOARD" => Ok(Access::Dashboard(helper.ref_)),
            "DASHBOARDEDIT" => Ok(Access::DashboardEdit(helper.ref_)),
            "DASHBOARDSHARE" => Ok(Access::DashboardShare(helper.ref_)),
            "DOWNTIMEALL" => Ok(Access::DowntimeAll(helper.ref_)),
            "DOWNTIMESOME" => Ok(Access::DowntimeSome(helper.ref_)),
            "NAVOPTIONS" => Ok(Access::NavOptions(helper.ref_)),
            "NETAUDITVIEW" => Ok(Access::NetAuditView(helper.ref_)),
            "NETFLOW" => Ok(Access::NetFlow(helper.ref_)),
            "NOTIFYSOME" => Ok(Access::NotifySome(helper.ref_)),
            "NTVIEWALL" => Ok(Access::NTViewAll(helper.ref_)),
            "PASSWORDSAVE" => Ok(Access::PasswordSave(helper.ref_)),
            "RELOADACCESS" => Ok(Access::ReloadAccess(helper.ref_)),
            "RELOADVIEW" => Ok(Access::ReloadView(helper.ref_)),
            "REMOTELYMANAGEDCLUSTERS" => Ok(Access::RemotelyManagedClusters(helper.ref_)),
            "REPORTADMIN" => Ok(Access::ReportAdmin(helper.ref_)),
            "REPORTUSER" => Ok(Access::ReportUser(helper.ref_)),
            "RRDGRAPHS" => Ok(Access::RrdGraphs(helper.ref_)),
            "TESTALL" => Ok(Access::TestAll(helper.ref_)),
            "TESTCHANGE" => Ok(Access::TestChange(helper.ref_)),
            "TESTSOME" => Ok(Access::TestSome(helper.ref_)),
            "VIEWALL" => Ok(Access::ViewAll(helper.ref_)),
            "VIEWPORTACCESS" => Ok(Access::ViewPortAccess(helper.ref_)),
            "VIEWSOME" => Ok(Access::ViewSome(helper.ref_)),
            _ => Err(de::Error::unknown_variant(&helper.name, VALID_NAMES)),
        }
    }
}

const VALID_NAMES: &[&str] = &[
    "ACTIONALL",
    "ACTIONSOME",
    "ADMINACCESS",
    "BSM",
    "CONFIGUREBSM",
    "CONFIGUREBSMCOMPONENT",
    "CONFIGURECONTACTS",
    "CONFIGUREHOSTGROUPS",
    "CONFIGUREHOSTS",
    "CONFIGUREKEYWORDS",
    "CONFIGURENETFLOW",
    "CONFIGUREPROFILES",
    "CONFIGUREREMOTECLUSTER",
    "CONFIGUREROLES",
    "CONFIGURESAVE",
    "CONFIGURETENANCIES",
    "CONFIGUREVIEW",
    "DASHBOARD",
    "DASHBOARDEDIT",
    "DASHBOARDSHARE",
    "DOWNTIMEALL",
    "DOWNTIMESOME",
    "NAVOPTIONS",
    "NETAUDITVIEW",
    "NETFLOW",
    "NOTIFYSOME",
    "NTVIEWALL",
    "PASSWORDSAVE",
    "RELOADACCESS",
    "RELOADVIEW",
    "REMOTELYMANAGEDCLUSTERS",
    "REPORTADMIN",
    "REPORTUSER",
    "RRDGRAPHS",
    "TESTALL",
    "TESTCHANGE",
    "TESTSOME",
    "VIEWALL",
    "VIEWPORTACCESS",
    "VIEWSOME",
];
