#![allow(missing_docs)]
use crate::prelude::*;
use serde::{Deserialize, Serialize};

use super::{HostStatusSummary, ServiceCheckStatusSummary};
// Example from the Opsview API at /rest/status/hostgroup
// ---- print_status stdout ----
// Object {
//     "list": Array [
//         Object {
//             "computed_state": String("critical"),
//             "downtime": Null,
//             "hostgroupid": String("2"),
//             "hosts": Object {
//                 "handled": String("1"),
//                 "total": String("1"),
//                 "unhandled": String("0"),
//                 "up": Object {
//                     "handled": String("1"),
//                 },
//             },
//             "leaf": String("1"),
//             "matpath": Array [
//                 Object {
//                     "id": String("1"),
//                     "name": String("Opsview"),
//                 },
//                 Object {
//                     "id": String("3"),
//                     "name": String("Solhagen"),
//                 },
//             ],
//             "name": String("Monitoring Servers"),
//             "services": Object {
//                 "computed_state": String("critical"),
//                 "critical": Object {
//                     "unhandled": String("1"),
//                 },
//                 "handled": String("118"),
//                 "ok": Object {
//                     "handled": String("118"),
//                 },
//                 "total": String("141"),
//                 "unhandled": String("23"),
//                 "unknown": Object {
//                     "unhandled": String("22"),
//                 },
//             },
//         },
//         Object {
//             "computed_state": String("critical"),
//             "downtime": Null,
//             "hostgroupid": String("4"),
//             "hosts": Object {
//                 "handled": String("3"),
//                 "total": String("3"),
//                 "unhandled": String("0"),
//                 "up": Object {
//                     "handled": String("3"),
//                 },
//             },
//             "leaf": String("1"),
//             "matpath": Array [
//                 Object {
//                     "id": String("1"),
//                     "name": String("Opsview"),
//                 },
//                 Object {
//                     "id": String("3"),
//                     "name": String("Solhagen"),
//                 },
//             ],
//             "name": String("Network Devices"),
//             "services": Object {
//                 "computed_state": String("critical"),
//                 "critical": Object {
//                     "unhandled": String("4"),
//                 },
//                 "handled": String("5"),
//                 "ok": Object {
//                     "handled": String("5"),
//                 },
//                 "total": String("19"),
//                 "unhandled": String("14"),
//                 "unknown": Object {
//                     "unhandled": String("10"),
//                 },
//             },
//         },
//         Object {
//             "computed_state": String("critical"),
//             "downtime": Null,
//             "hostgroupid": String("1"),
//             "hosts": Object {
//                 "handled": String("5"),
//                 "total": String("5"),
//                 "unhandled": String("0"),
//                 "up": Object {
//                     "handled": String("5"),
//                 },
//             },
//             "leaf": String("0"),
//             "matpath": Array [],
//             "name": String("Opsview"),
//             "services": Object {
//                 "computed_state": String("critical"),
//                 "critical": Object {
//                     "unhandled": String("7"),
//                 },
//                 "handled": String("132"),
//                 "ok": Object {
//                     "handled": String("132"),
//                 },
//                 "total": String("171"),
//                 "unhandled": String("39"),
//                 "unknown": Object {
//                     "unhandled": String("32"),
//                 },
//             },
//         },
//         Object {
//             "computed_state": String("critical"),
//             "downtime": Null,
//             "hostgroupid": String("6"),
//             "hosts": Object {
//                 "handled": String("1"),
//                 "total": String("1"),
//                 "unhandled": String("0"),
//                 "up": Object {
//                     "handled": String("1"),
//                 },
//             },
//             "leaf": String("1"),
//             "matpath": Array [
//                 Object {
//                     "id": String("1"),
//                     "name": String("Opsview"),
//                 },
//                 Object {
//                     "id": String("3"),
//                     "name": String("Solhagen"),
//                 },
//             ],
//             "name": String("Servers"),
//             "services": Object {
//                 "computed_state": String("critical"),
//                 "critical": Object {
//                     "unhandled": String("2"),
//                 },
//                 "handled": String("9"),
//                 "ok": Object {
//                     "handled": String("9"),
//                 },
//                 "total": String("11"),
//                 "unhandled": String("2"),
//             },
//         },
//         Object {
//             "computed_state": String("critical"),
//             "downtime": Null,
//             "hostgroupid": String("3"),
//             "hosts": Object {
//                 "handled": String("5"),
//                 "total": String("5"),
//                 "unhandled": String("0"),
//                 "up": Object {
//                     "handled": String("5"),
//                 },
//             },
//             "leaf": String("0"),
//             "matpath": Array [
//                 Object {
//                     "id": String("1"),
//                     "name": String("Opsview"),
//                 },
//             ],
//             "name": String("Solhagen"),
//             "services": Object {
//                 "computed_state": String("critical"),
//                 "critical": Object {
//                     "unhandled": String("7"),
//                 },
//                 "handled": String("132"),
//                 "ok": Object {
//                     "handled": String("132"),
//                 },
//                 "total": String("171"),
//                 "unhandled": String("39"),
//                 "unknown": Object {
//                     "unhandled": String("32"),
//                 },
//             },
//         },
//     ],
//     "summary": Object {
//         "handled": String("411"),
//         "host": Object {
//             "handled": String("15"),
//             "total": String("15"),
//             "unhandled": String("0"),
//             "up": String("15"),
//         },
//         "service": Object {
//             "critical": String("21"),
//             "handled": String("396"),
//             "ok": String("396"),
//             "total": String("513"),
//             "unhandled": String("117"),
//             "unknown": String("96"),
//         },
//         "total": String("528"),
//         "totalhgs": String("9"),
//         "unhandled": String("117"),
//     },
// }

impl CreateFromJson for HostGroupStatusSummary {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HostGroupStatusSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    computed_state: Option<ServiceCheckState>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    downtime: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    hostgroupid: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    hosts: Option<HostStatusSummary>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_option_bool",
        serialize_with = "serialize_option_bool_as_string",
        default
    )]
    leaf: Option<bool>,

    matpath: Matpath,

    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    services: Option<ServiceCheckStatusSummary>,
    //             "computed_state": String("critical"),
    //             "downtime": Null,
    //             "hostgroupid": String("2"),
    //             "hosts": Object {
    //                 "handled": String("1"),
    //                 "total": String("1"),
    //                 "unhandled": String("0"),
    //                 "up": Object {
    //                     "handled": String("1"),
    //                 },
    //             },
    //             "leaf": String("1"),
    //             "matpath": Array [
    //                 Object {
    //                     "id": String("1"),
    //                     "name": String("Opsview"),
    //                 },
    //                 Object {
    //                     "id": String("3"),
    //                     "name": String("Solhagen"),
    //                 },
    //             ],
    //             "name": String("Monitoring Servers"),
    //             "services": Object {
    //                 "computed_state": String("critical"),
    //                 "critical": Object {
    //                     "unhandled": String("1"),
    //                 },
    //                 "handled": String("118"),
    //                 "ok": Object {
    //                     "handled": String("118"),
    //                 },
    //                 "total": String("141"),
    //                 "unhandled": String("23"),
    //                 "unknown": Object {
    //                     "unhandled": String("22"),
    //                 },
    //             },
    //         },
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
struct MatpathComponent {
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    id: Option<u64>,
    name: String,
}

type Matpath = Vec<MatpathComponent>;
