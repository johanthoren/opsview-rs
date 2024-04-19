#![allow(missing_docs)]
use crate::prelude::*;
use serde::{Deserialize, Serialize};
// ---- print_host_summary stdout ----
// Object {
//     "list": Array [
//         Object {
//             "alias": String("Main Router"),
//             "current_check_attempt": String("1"),
//             "downtime": String("0"),
//             "icon": String("router"),
//             "last_check": String("1709736631"),
//             "max_check_attempts": String("2"),
//             "name": String("192.168.1.1"),
//             "num_interfaces": String("0"),
//             "num_services": String("1"),
//             "output": String("OK - 192.168.1.1: rta 0.111ms, lost 0%"),
//             "state": String("up"),
//             "state_duration": String("1004317"),
//             "state_type": String("hard"),
//             "summary": Object {
//                 "handled": String("1"),
//                 "ok": Object {
//                     "handled": String("1"),
//                 },
//                 "total": String("1"),
//                 "unhandled": String("0"),
//             },
//             "unhandled": String("0"),
//         },
//         Object {
//             "alias": String("Bedroom AP"),
//             "current_check_attempt": String("1"),
//             "downtime": String("0"),
//             "icon": String("wireless"),
//             "last_check": String("1709736631"),
//             "max_check_attempts": String("2"),
//             "name": String("192.168.1.2"),
//             "num_interfaces": String("1"),
//             "num_services": String("9"),
//             "output": String("OK - 192.168.1.2: rta 0.276ms, lost 0%"),
//             "state": String("up"),
//             "state_duration": String("1004254"),
//             "state_type": String("hard"),
//             "summary": Object {
//                 "critical": Object {
//                     "unhandled": String("2"),
//                 },
//                 "handled": String("2"),
//                 "ok": Object {
//                     "handled": String("2"),
//                 },
//                 "total": String("9"),
//                 "unhandled": String("7"),
//                 "unknown": Object {
//                     "unhandled": String("5"),
//                 },
//             },
//             "unhandled": String("0"),
//         },
//         Object {
//             "alias": String("Kitchen AP"),
//             "current_check_attempt": String("1"),
//             "downtime": String("0"),
//             "icon": String("wireless"),
//             "last_check": String("1709736631"),
//             "max_check_attempts": String("2"),
//             "name": String("192.168.1.3"),
//             "num_interfaces": String("1"),
//             "num_services": String("9"),
//             "output": String("OK - 192.168.1.3: rta 0.362ms, lost 0%"),
//             "state": String("up"),
//             "state_duration": String("353135"),
//             "state_type": String("hard"),
//             "summary": Object {
//                 "critical": Object {
//                     "unhandled": String("2"),
//                 },
//                 "handled": String("2"),
//                 "ok": Object {
//                     "handled": String("2"),
//                 },
//                 "total": String("9"),
//                 "unhandled": String("7"),
//                 "unknown": Object {
//                     "unhandled": String("5"),
//                 },
//             },
//             "unhandled": String("0"),
//         },
//         Object {
//             "alias": String("Bedroom MacMini Server"),
//             "current_check_attempt": String("1"),
//             "downtime": String("0"),
//             "icon": String("server"),
//             "last_check": String("1709736631"),
//             "max_check_attempts": String("2"),
//             "name": String("mini"),
//             "num_interfaces": String("1"),
//             "num_services": String("11"),
//             "output": String("OK - 192.168.1.101: rta 0.364ms, lost 0%"),
//             "state": String("up"),
//             "state_duration": String("1004557"),
//             "state_type": String("hard"),
//             "summary": Object {
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
//             "unhandled": String("0"),
//         },
//         Object {
//             "alias": String("Opsview Master Server"),
//             "current_check_attempt": String("1"),
//             "downtime": String("0"),
//             "icon": String("opsview"),
//             "last_check": String("1709736632"),
//             "max_check_attempts": String("2"),
//             "name": String("opsview"),
//             "num_interfaces": String("0"),
//             "num_services": String("141"),
//             "output": String("OK - t490.thoren.xyz: rta 0.020ms, lost 0%"),
//             "state": String("up"),
//             "state_duration": String("6929273"),
//             "state_type": String("hard"),
//             "summary": Object {
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
//             "unhandled": String("0"),
//         },
//     ],
//     "summary": Object {
//         "handled": String("137"),
//         "host": Object {
//             "handled": String("5"),
//             "total": String("5"),
//             "unhandled": String("0"),
//             "up": String("5"),
//         },
//         "service": Object {
//             "critical": String("7"),
//             "handled": String("132"),
//             "ok": String("132"),
//             "total": String("171"),
//             "unhandled": String("39"),
//             "unknown": String("32"),
//         },
//         "total": String("176"),
//         "unhandled": String("39"),
//     },
// }

//             "hosts": Object {
//                 "handled": String("1"),
//                 "total": String("1"),
//                 "unhandled": String("0"),
//                 "up": Object {
//                     "handled": String("1"),
//                 },
//             },

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct HostStatusSummary {
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub handled: Option<u64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub total: Option<u64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_string_or_number_to_u64",
        default
    )]
    pub unhandled: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down: Option<HandledCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unreachable: Option<HandledCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up: Option<HandledCount>,
}

impl StatusSummary for HostStatusSummary {
    fn handled(&self) -> u64 {
        self.down.clone().unwrap_or_default().handled()
            + self.unreachable.clone().unwrap_or_default().handled()
            + self.up.clone().unwrap_or_default().handled()
    }

    fn unhandled(&self) -> u64 {
        self.down.clone().unwrap_or_default().unhandled()
            + self.unreachable.clone().unwrap_or_default().unhandled()
            + self.up.clone().unwrap_or_default().unhandled()
    }

    fn total(&self) -> u64 {
        self.down.clone().unwrap_or_default().total()
            + self.unreachable.clone().unwrap_or_default().total()
            + self.up.clone().unwrap_or_default().total()
    }
}
