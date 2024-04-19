#![allow(missing_docs)]
// ---- print_hashtags_summary stdout ----
// Object {
//     "list": Array [
//         Object {
//             "calculate_hard_states": String("0"),
//             "computed_state": String("ok"),
//             "description": String("Opsview Monitoring System"),
//             "downtime": Null,
//             "exclude_handled": String("0"),
//             "hosts": Object {
//                 "handled": String("1"),
//                 "total": String("1"),
//                 "unhandled": String("0"),
//                 "up": Object {
//                     "handled": String("1"),
//                 },
//             },
//             "name": String("opsview"),
//             "services": Object {
//                 "computed_state": String("ok"),
//                 "handled": String("12"),
//                 "ok": Object {
//                     "handled": String("12"),
//                 },
//                 "total": String("12"),
//                 "unhandled": String("0"),
//             },
//         },
//         Object {
//             "calculate_hard_states": String("0"),
//             "computed_state": String("critical"),
//             "description": String("Opsview Components"),
//             "downtime": Null,
//             "exclude_handled": String("0"),
//             "hosts": Object {
//                 "handled": String("1"),
//                 "total": String("1"),
//                 "unhandled": String("0"),
//                 "up": Object {
//                     "handled": String("1"),
//                 },
//             },
//             "name": String("opsview-components"),
//             "services": Object {
//                 "computed_state": String("critical"),
//                 "critical": Object {
//                     "unhandled": String("1"),
//                 },
//                 "handled": String("92"),
//                 "ok": Object {
//                     "handled": String("92"),
//                 },
//                 "total": String("115"),
//                 "unhandled": String("23"),
//                 "unknown": Object {
//                     "unhandled": String("22"),
//                 },
//             },
//         },
//     ],
//     "summary": Object {
//         "handled": String("106"),
//         "host": Object {
//             "handled": String("2"),
//             "total": String("2"),
//             "unhandled": String("0"),
//             "up": String("2"),
//         },
//         "service": Object {
//             "critical": String("1"),
//             "handled": String("104"),
//             "ok": String("104"),
//             "total": String("127"),
//             "unhandled": String("23"),
//             "unknown": String("22"),
//         },
//         "total": String("129"),
//         "unhandled": String("23"),
//     },
// }

pub struct HashtagsSummary {}
