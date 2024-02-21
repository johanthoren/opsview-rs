#![allow(missing_docs)]
use crate::prelude::*;
use decimal_percentage::Percentage;
use lazy_static::lazy_static;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use regex::Regex;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn is_valid_past_unix_timestamp(ts: u64) -> bool {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(now) => ts < now.as_millis() as u64,
        Err(_) => false, // SystemTime is before UNIX_EPOCH
    }
}

// Validation related functions and constants

pub const BSM_COMPONENT_NAME_REGEX_STR: &str = r"^[\p{L}\p{N}][\p{L}\p{N}\p{S}\p{P} ]*$";
pub const CONTACT_NAME_REGEX_STR: &str = r"^[\p{L}\p{N}][\p{L}\p{N}._@#-]*$";
pub const CONTACTLINK_NAME_REGEX_STR: &str = r"^[\p{L}\p{N}][\p{L}\p{N} ._-]*$";
pub const CONTACTLINK_URL_REGEX_STR: &str = r"^(https?:\/)?\/[\p{L}\p{N}\p{S}\p{P} ]*$";
pub const CRITICAL_COMPARISON_REGEX_STR: &str = r"^(eq|ne|regex|==|<|>)$";
pub const HASHTAG_NAME_REGEX_STR: &str = r"^[\p{L}\p{N}][\p{L}\p{N}_-]*$";
pub const HOST_NAME_REGEX_STR: &str = r"^[\p{L}\p{N}.\-_]+$";
pub const HOST_NOTIFICATION_OPTIONS_REGEX_STR: &str = r"^[udrfsn]*(,[udrfs])*$";
pub const HOSTGROUP_NAME_REGEX_STR: &str = r"^[\p{L}\p{N}][\p{L}\p{N} ./+\-_]*$";
pub const HOSTTEMPLATE_NAME_REGEX_STR: &str = r"^[\p{L}\p{N}][\p{L}\p{N} .\-_]*$";
pub const INLINE_FREE_TEXT_REGEX_STR: &str = r"^[\P{Z}\p{N}\p{S}\p{P} ]*$";
pub const MONITORINGCLUSTER_NAME_REGEX_STR: &str = r"^[\p{L}\p{N} .+\-_]+$";
pub const NOTIFICATIONMETHOD_NAME_REGEX_STR: &str = r"^[a-zA-Z0-9.\- ]+$";
pub const NOTIFICATIONPROFILE_NAME_REGEX_STR: &str = r"^[\p{L}\p{N}][\p{L}\p{N} .@\-_]*$";
pub const NOTIFICATIONPROFILE_BSM_COMPONENT_OPTIONS_REGEX_STR: &str = r"^[rfian](,[rfia])*$";
pub const NOTIFICATIONPROFILE_BSM_SERVICE_OPTIONS_REGEX_STR: &str = r"^[roian](,[roia])*$";
pub const RANCID_VENDOR_NAME_REGEX_STR: &str = r"^[\p{L}\p{N} \-/]+$";
pub const ROLE_NAME_REGEX_STR: &str = r"^[\p{L}\p{N} ,\-_]+$";
pub const SERVICECHECK_LABEL_REGEX_STR: &str = r"^[\p{L}\p{N}\-_]*$";
pub const SERVICECHECK_NAME_REGEX_STR: &str = r"^[\p{L}\p{N}./\-_ ]+$";
pub const SERVICECHECK_NOTIFICATION_OPTIONS_REGEX_STR: &str = r"^[wcurfsn]*(,[wcurfs])*$";
pub const SERVICECHECK_OID_REGEX_STR: &str = r#"^ *[a-zA-Z0-9.:\-"/_]+$"#;
pub const SERVICECHECK_STALKING_REGEX_STR: &str = r"^[wcuo](,[wcuo])*$";
pub const SERVICEGROUP_NAME_REGEX_STR: &str = r"^[\p{L}\p{N}][\p{L}\p{N} ./+\-_]*$";
pub const TENANCY_NAME_REGEX_STR: &str = r"^[\p{L}\p{N}][\p{L}\p{N}\p{S}\p{P} ]*$";
pub const TIMEPERIOD_ALIAS_REGEX_STR: &str = r"^[\p{L}\p{N} .,:/\-_]*$";
pub const TIMEPERIOD_NAME_REGEX_STR: &str = r"^[\p{L}\p{N}][\p{L}\p{N}./\-_]*$";
pub const TIMEPERIOD_WEEKDAY_REGEX_STR: &str = r"^\d\d:\d\d-\d\d:\d\d(,\d\d:\d\d-\d\d:\d\d)*$";
pub const URL_REGEX_STR: &str = r"^[a-zA-Z][a-zA-Z0-9-+.]*://[\p{L}\p{N}\p{S}\p{P} ]*$";
pub const VARIABLE_NAME_REGEX_STR: &str = r"^[A-Z0-9_]+$";
pub const VARIABLE_VALUE_REGEX_STR: &str = r"^[ \p{L}\p{N}./\-\\_()]*$";
pub const WARNING_COMPARISON_REGEX_STR: &str = r"^(==|<|>)$";

lazy_static! {
    static ref BSM_COMPONENT_NAME_REGEX: Regex = Regex::new(BSM_COMPONENT_NAME_REGEX_STR).unwrap();
    static ref CONTACT_NAME_REGEX: Regex = Regex::new(CONTACT_NAME_REGEX_STR).unwrap();
    static ref CONTACTLINK_NAME_REGEX: Regex = Regex::new(CONTACTLINK_NAME_REGEX_STR).unwrap();
    static ref CONTACTLINK_URL_REGEX: Regex = Regex::new(CONTACTLINK_URL_REGEX_STR).unwrap();
    static ref CRITICAL_COMPARISON_REGEX: Regex =
        Regex::new(CRITICAL_COMPARISON_REGEX_STR).unwrap();
    static ref HASHTAG_NAME_REGEX: Regex = Regex::new(HASHTAG_NAME_REGEX_STR).unwrap();
    static ref HOST_NAME_REGEX: Regex = Regex::new(HOST_NAME_REGEX_STR).unwrap();
    static ref HOST_NOTIFICATION_OPTIONS_REGEX: Regex =
        Regex::new(HOST_NOTIFICATION_OPTIONS_REGEX_STR).unwrap();
    static ref HOSTGROUP_NAME_REGEX: Regex = Regex::new(HOSTGROUP_NAME_REGEX_STR).unwrap();
    static ref HOSTTEMPLATE_NAME_REGEX: Regex = Regex::new(HOSTTEMPLATE_NAME_REGEX_STR).unwrap();
    static ref INLINE_FREE_TEXT_REGEX: Regex = Regex::new(INLINE_FREE_TEXT_REGEX_STR).unwrap();
    static ref MONITORINGCLUSTER_NAME_REGEX: Regex =
        Regex::new(MONITORINGCLUSTER_NAME_REGEX_STR).unwrap();
    static ref NOTIFICATIONMETHOD_NAME_REGEX: Regex =
        Regex::new(NOTIFICATIONMETHOD_NAME_REGEX_STR).unwrap();
    static ref NOTIFICATIONPROFILE_NAME_REGEX: Regex =
        Regex::new(NOTIFICATIONPROFILE_NAME_REGEX_STR).unwrap();
    static ref NOTIFICATIONPROFILE_BSM_COMPONENT_OPTIONS_REGEX: Regex =
        Regex::new(NOTIFICATIONPROFILE_BSM_COMPONENT_OPTIONS_REGEX_STR).unwrap();
    static ref NOTIFICATIONPROFILE_BSM_SERVICE_OPTIONS_REGEX: Regex =
        Regex::new(NOTIFICATIONPROFILE_BSM_SERVICE_OPTIONS_REGEX_STR).unwrap();
    static ref RANCID_VENDOR_NAME_REGEX: Regex = Regex::new(RANCID_VENDOR_NAME_REGEX_STR).unwrap();
    static ref ROLE_NAME_REGEX: Regex = Regex::new(ROLE_NAME_REGEX_STR).unwrap();
    static ref SERVICECHECK_LABEL_REGEX: Regex = Regex::new(SERVICECHECK_LABEL_REGEX_STR).unwrap();
    static ref SERVICECHECK_NAME_REGEX: Regex = Regex::new(SERVICECHECK_NAME_REGEX_STR).unwrap();
    static ref SERVICECHECK_NOTIFICATION_OPTIONS_REGEX: Regex =
        Regex::new(SERVICECHECK_NOTIFICATION_OPTIONS_REGEX_STR).unwrap();
    static ref SERVICECHECK_OID_REGEX: Regex = Regex::new(SERVICECHECK_OID_REGEX_STR).unwrap();
    static ref SERVICECHECK_STALKING_REGEX: Regex =
        Regex::new(SERVICECHECK_STALKING_REGEX_STR).unwrap();
    static ref SERVICEGROUP_NAME_REGEX: Regex = Regex::new(SERVICEGROUP_NAME_REGEX_STR).unwrap();
    static ref TENANCY_NAME_REGEX: Regex = Regex::new(TENANCY_NAME_REGEX_STR).unwrap();
    static ref TIMEPERIOD_ALIAS_REGEX: Regex = Regex::new(TIMEPERIOD_ALIAS_REGEX_STR).unwrap();
    static ref TIMEPERIOD_NAME_REGEX: Regex = Regex::new(TIMEPERIOD_NAME_REGEX_STR).unwrap();
    static ref TIMEPERIOD_WEEKDAY_REGEX: Regex = Regex::new(TIMEPERIOD_WEEKDAY_REGEX_STR).unwrap();
    static ref URL_REGEX: Regex = Regex::new(URL_REGEX_STR).unwrap();
    static ref VARIABLE_NAME_REGEX: Regex = Regex::new(VARIABLE_NAME_REGEX_STR).unwrap();
    static ref VARIABLE_VALUE_REGEX: Regex = Regex::new(VARIABLE_VALUE_REGEX_STR).unwrap();
    static ref WARNING_COMPARISON_REGEX: Regex = Regex::new(WARNING_COMPARISON_REGEX_STR).unwrap();
}

fn validate_string(
    s: &str,
    min_length: usize,
    max_length: usize,
    re: &Regex,
    trim: bool,
) -> Result<String, OpsviewConfigError> {
    let trimmed_s = match trim {
        true => s.trim(),
        false => s,
    };

    let trimmed_s_length = trimmed_s.len();

    if trimmed_s_length < min_length {
        return Err(OpsviewConfigError::StringTooShort(
            min_length,
            trimmed_s_length,
        ));
    }

    if trimmed_s_length > max_length {
        return Err(OpsviewConfigError::StringTooLong(
            max_length,
            trimmed_s_length,
        ));
    }

    if re.is_match(trimmed_s) {
        Ok(trimmed_s.to_string())
    } else {
        Err(OpsviewConfigError::DoesNotMatchRegex(
            trimmed_s.to_string(),
            re.as_str().to_string(),
        ))
    }
}

fn validate_trimmed_string(
    s: &str,
    min_length: usize,
    max_length: usize,
    re: &Regex,
) -> Result<String, OpsviewConfigError> {
    validate_string(s, min_length, max_length, re, true)
}

fn validate_untrimmed_string(
    s: &str,
    min_length: usize,
    max_length: usize,
    re: &Regex,
) -> Result<String, OpsviewConfigError> {
    validate_string(s, min_length, max_length, re, false)
}

fn percent_encoded_str_not_too_long(s: &str, max_length: usize) -> Result<(), OpsviewConfigError> {
    let trimmed_s = s.trim_end();
    let percent_encoded_s = utf8_percent_encode(trimmed_s, NON_ALPHANUMERIC).to_string();
    let percent_encoded_s_length = percent_encoded_s.len();
    if percent_encoded_s_length > max_length {
        return Err(OpsviewConfigError::StringTooLongWhenPercentEncoded(
            max_length,
            percent_encoded_s_length,
        ));
    }
    Ok(())
}

// pub fn validate_opt_copy<T, U, E, F>(option: Option<T>, validator: F) -> Result<Option<U>, E>
// where
//     T: Copy,
//     F: FnOnce(T) -> Result<U, E>,
// {
//     option.map(validator).transpose()
// }

pub fn validate_opt_string<E, F>(option: Option<String>, validator: F) -> Result<Option<String>, E>
where
    F: FnOnce(&str) -> Result<String, E>,
{
    match option {
        Some(value) => validator(value.as_str()).map(Some),
        None => Ok(None),
    }
}

pub fn validate_arg_string(arg: &str) -> Result<String, OpsviewConfigError> {
    validate_untrimmed_string(arg, 0, 16000, &INLINE_FREE_TEXT_REGEX)
}

pub fn validate_and_trim_bsmcomponent_name(name: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(name, 1, 255, &BSM_COMPONENT_NAME_REGEX)
}

pub fn validate_and_trim_bsmservice_name(name: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(name, 1, 255, &INLINE_FREE_TEXT_REGEX)
}

pub fn validate_and_trim_contact_name(name: &str) -> Result<String, OpsviewConfigError> {
    is_valid_utfmb3(name)?;
    validate_trimmed_string(name, 1, 64, &CONTACT_NAME_REGEX)
}

pub fn validate_and_trim_contactlink_name(name: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(name, 3, 128, &CONTACTLINK_NAME_REGEX)
}

pub fn validate_and_trim_contactlink_url(url: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(url, 1, 255, &CONTACTLINK_URL_REGEX)
}

pub fn validate_and_trim_critical_comparison(comp: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(comp, 0, 10, &CRITICAL_COMPARISON_REGEX)
}

pub fn validate_and_trim_critical_value(value: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(value, 0, 255, &INLINE_FREE_TEXT_REGEX)
}

pub fn validate_and_trim_description(description: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(description, 0, 255, &INLINE_FREE_TEXT_REGEX)
}

pub fn validate_and_trim_hashtag_name(name: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(name, 1, 128, &HASHTAG_NAME_REGEX)
}

pub fn validate_and_trim_host_event_handler(
    event_handler: &str,
) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(event_handler, 0, 255, &INLINE_FREE_TEXT_REGEX)
}

pub fn validate_and_trim_host_name(name: &str) -> Result<String, OpsviewConfigError> {
    percent_encoded_str_not_too_long(name, 255)?;
    validate_trimmed_string(name, 1, 64, &HOST_NAME_REGEX)
}

pub fn validate_and_trim_host_notification_options(
    options: &str,
) -> Result<String, OpsviewConfigError> {
    contains_only_unique_options(options)?;
    validate_trimmed_string(options, 0, 16, &HOST_NOTIFICATION_OPTIONS_REGEX)
}

pub fn validate_and_trim_hostcheckcommand_name(name: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(name, 1, 128, &INLINE_FREE_TEXT_REGEX)
}

pub fn validate_and_trim_hostgroup_name(name: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(name, 1, 128, &HOSTGROUP_NAME_REGEX)
}

pub fn validate_and_trim_hosticon_name(name: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(name, 1, 128, &INLINE_FREE_TEXT_REGEX)
}

pub fn validate_and_trim_hosttemplate_name(name: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(name, 1, 128, &HOSTTEMPLATE_NAME_REGEX)
}

pub fn validate_and_trim_ipv4(s: &str) -> Result<String, OpsviewConfigError> {
    let trimmed_input = s.trim();
    if trimmed_input.parse::<Ipv4Addr>().is_ok() {
        Ok(trimmed_input.to_string())
    } else {
        Err(OpsviewConfigError::InvalidIP(trimmed_input.to_string()))
    }
}

pub fn validate_and_trim_ip_or_hostname(s: &str) -> Result<String, OpsviewConfigError> {
    let trimmed_input = s.trim_end();
    if trimmed_input.parse::<Ipv4Addr>().is_ok() {
        return Ok(trimmed_input.to_string());
    }
    if trimmed_input.parse::<Ipv6Addr>().is_ok() {
        return Ok(trimmed_input.to_string());
    }
    match url::Host::parse(trimmed_input) {
        Ok(url::Host::Domain(_)) | Ok(url::Host::Ipv4(_)) | Ok(url::Host::Ipv6(_)) => {
            Ok(trimmed_input.to_string())
        }
        _ => Err(OpsviewConfigError::InvalidIP(trimmed_input.to_string())),
    }
}

pub fn validate_and_trim_label_string(label: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(label, 0, 64, &INLINE_FREE_TEXT_REGEX)
}

pub fn validate_and_trim_managementurl_name(name: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(name, 1, 191, &INLINE_FREE_TEXT_REGEX)
}

pub fn validate_and_trim_monitoringcluster_name(name: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(name, 1, 64, &MONITORINGCLUSTER_NAME_REGEX)
}

pub fn validate_and_trim_netflowcollector_name(name: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(name, 1, 64, &INLINE_FREE_TEXT_REGEX)
}

pub fn validate_and_trim_notificationmethod_name(name: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(name, 1, 64, &NOTIFICATIONMETHOD_NAME_REGEX)
}

pub fn validate_and_trim_notificationprofile_name(
    name: &str,
) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(name, 1, 128, &NOTIFICATIONPROFILE_NAME_REGEX)
}

pub fn validate_and_trim_notificationprofile_bsm_component_options(
    options: &str,
) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(
        options,
        1,
        16,
        &NOTIFICATIONPROFILE_BSM_COMPONENT_OPTIONS_REGEX,
    )
}

pub fn validate_and_trim_notificationprofile_bsm_service_options(
    options: &str,
) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(
        options,
        1,
        16,
        &NOTIFICATIONPROFILE_BSM_SERVICE_OPTIONS_REGEX,
    )
}

pub fn validate_and_trim_other_addresses(
    other_addresses: &str,
) -> Result<String, OpsviewConfigError> {
    let trimmed_other_addresses = other_addresses.trim_end();
    let addresses = trimmed_other_addresses.split(',').map(str::trim);
    for addr in addresses {
        validate_and_trim_ip_or_hostname(addr)?;
    }
    Ok(trimmed_other_addresses.to_string())
}

pub fn validate_rancid_password(password: &str) -> Result<String, OpsviewConfigError> {
    contains_only_allowed_characters(
        &validate_untrimmed_string(password, 0, 255, &INLINE_FREE_TEXT_REGEX)?,
        &['{', '}'],
    )
}

pub fn validate_rancid_username(username: &str) -> Result<String, OpsviewConfigError> {
    contains_only_allowed_characters(
        &validate_untrimmed_string(username, 0, 128, &INLINE_FREE_TEXT_REGEX)?,
        &['{', '}'],
    )
}

pub fn validate_and_trim_rancid_vendor_name(name: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(name, 1, 128, &RANCID_VENDOR_NAME_REGEX)
}

pub fn validate_port(port: u64) -> Result<u64, OpsviewConfigError> {
    if port > 65535 {
        return Err(OpsviewConfigError::PortOutOfRange(port));
    }
    Ok(port)
}

pub fn validate_and_trim_role_name(name: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(name, 1, 128, &ROLE_NAME_REGEX)
}

pub fn validate_and_trim_servicecheck_label(label: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(label, 0, 40, &SERVICECHECK_LABEL_REGEX)
}

pub fn validate_and_trim_servicecheck_name(name: &str) -> Result<String, OpsviewConfigError> {
    percent_encoded_str_not_too_long(name, 255)?;
    validate_trimmed_string(name, 1, 64, &SERVICECHECK_NAME_REGEX)
}

pub fn validate_servicecheck_args(args: &str) -> Result<String, OpsviewConfigError> {
    validate_untrimmed_string(args, 0, 16000, &INLINE_FREE_TEXT_REGEX)
}

pub fn validate_and_trim_servicecheck_notification_options(
    options: &str,
) -> Result<String, OpsviewConfigError> {
    contains_only_unique_options(options)?;
    validate_trimmed_string(options, 0, 16, &SERVICECHECK_NOTIFICATION_OPTIONS_REGEX)
}

pub fn validate_and_trim_servicegroup_name(name: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(name, 1, 128, &SERVICEGROUP_NAME_REGEX)
}

pub fn validate_servicecheck_oid(oid: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(oid, 1, 255, &SERVICECHECK_OID_REGEX)
}

pub fn validate_and_trim_servicecheck_stalking(
    options: &str,
) -> Result<String, OpsviewConfigError> {
    contains_only_unique_options(options)?;
    validate_trimmed_string(options, 0, 16, &SERVICECHECK_STALKING_REGEX)
}

pub fn validate_and_trim_snmp_community(community: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(community, 1, 1600, &INLINE_FREE_TEXT_REGEX)
}

pub fn validate_snmpv3_password(password: &str) -> Result<String, OpsviewConfigError> {
    validate_untrimmed_string(password, 8, 1600, &INLINE_FREE_TEXT_REGEX)
}

pub fn validate_snmpv3_username(username: &str) -> Result<String, OpsviewConfigError> {
    validate_untrimmed_string(username, 1, 128, &INLINE_FREE_TEXT_REGEX)
}

pub fn validate_state(state: u64) -> Result<u64, OpsviewConfigError> {
    if state > 3 {
        return Err(OpsviewConfigError::StateOutOfRange(state));
    }
    Ok(state)
}

pub fn validate_and_trim_tenancy_name(name: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(name, 1, 191, &TENANCY_NAME_REGEX)
}

pub fn validate_and_trim_timeperiod_alias(alias: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(alias, 1, 128, &TIMEPERIOD_ALIAS_REGEX)
}

pub fn validate_and_trim_timeperiod_name(name: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(name, 1, 128, &TIMEPERIOD_NAME_REGEX)
}

pub fn validate_and_trim_timeperiod_weekday(s: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(s, 1, 255, &TIMEPERIOD_WEEKDAY_REGEX)
}

pub fn validate_uri(uri: &str) -> Result<String, OpsviewConfigError> {
    is_valid_utfmb3(uri)?;

    // This adds additional validation not done by the API, might remove this in the future.
    url::Url::parse(uri)?;

    validate_untrimmed_string(uri, 1, 16000, &URL_REGEX)
}

pub fn validate_and_trim_variable_name(name: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(name, 1, 64, &VARIABLE_NAME_REGEX)
}

pub fn validate_variable_value(value: &str) -> Result<String, OpsviewConfigError> {
    validate_untrimmed_string(value, 0, 64, &VARIABLE_VALUE_REGEX)
}

pub fn validate_and_trim_warning_comparison(comp: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(comp, 1, 10, &WARNING_COMPARISON_REGEX)
}

pub fn validate_and_trim_warning_value(value: &str) -> Result<String, OpsviewConfigError> {
    validate_trimmed_string(value, 0, 255, &INLINE_FREE_TEXT_REGEX)
}

fn contains_only_unique_options(options: &str) -> Result<(), OpsviewConfigError> {
    let mut chars = std::collections::HashSet::new();
    match options
        .chars()
        .filter(|&c| c != ',')
        .all(|c| chars.insert(c))
    {
        true => Ok(()),
        false => Err(OpsviewConfigError::DuplicateOptions(options.to_string())),
    }
}

fn contains_only_allowed_characters(
    s: &str,
    forbidden: &[char],
) -> Result<String, OpsviewConfigError> {
    if let Some(forbidden_char) = s.chars().find(|c| forbidden.contains(c)) {
        Err(OpsviewConfigError::ForbiddenCharacter(forbidden_char))
    } else {
        Ok(s.to_string())
    }
}

fn is_valid_utfmb3(s: &str) -> Result<(), OpsviewConfigError> {
    match s.chars().find(|&ch| ch.len_utf8() > 3) {
        Some(offending_char) => Err(OpsviewConfigError::InvalidUtf8(offending_char)),
        None => Ok(()),
    }
}

pub fn percentage_between_0_and_100(p: Percentage) -> Result<(), OpsviewConfigError> {
    if p < Percentage::from(0.0) || p > Percentage::from(100.00) {
        Err(OpsviewConfigError::InvalidPercentage(
            p.to_string(),
            "Percentage not between 0 and 100".to_string(),
        ))
    } else {
        Ok(())
    }
}

pub fn require_field<T: Clone>(
    field: &Option<T>,
    field_name: &str,
) -> Result<T, OpsviewConfigError> {
    field
        .as_ref()
        .cloned()
        .ok_or_else(|| OpsviewConfigError::RequiredFieldEmpty(field_name.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_valid_variable_name() {
        // Test valid names
        assert!(validate_and_trim_variable_name("VARIABLE1").is_ok());
        assert!(validate_and_trim_variable_name("VARIABLE_NAME_2").is_ok());
        assert!(validate_and_trim_variable_name("A1234567890").is_ok());
        assert!(validate_and_trim_variable_name(&"A".repeat(64)).is_ok());

        // Test invalid names
        assert!(validate_and_trim_variable_name("").is_err()); // Empty name
        assert!(validate_and_trim_variable_name("lowercase").is_err()); // Lowercase not allowed
        assert!(validate_and_trim_variable_name("Invalid-Name").is_err()); // Hyphen not allowed
        assert!(validate_and_trim_variable_name("Invalid Name").is_err()); // Space not allowed
        assert!(validate_and_trim_variable_name(&"A".repeat(65)).is_err()); // Exceeds max length
    }

    #[test]
    fn test_valid_other_addresses_succeeds() {
        let valid_addresses = [
            "192.168.1.1,127.0.1.1",
            "localhost,192.168.1.1",
            "foo.bar,localhost",
            "foo-bar,foo.bar,127.0.0.1",
        ];

        for &address in &valid_addresses {
            assert!(
                validate_and_trim_other_addresses(address).is_ok(),
                "Expected '{}' to be valid",
                address
            );
        }
    }

    #[test]
    fn test_is_invalid_rancid_password() {
        let invalid_passwords = ["rancid}", " {foo", &"a".repeat(256)];

        for pass in invalid_passwords.iter() {
            assert!(validate_rancid_password(pass).is_err());
        }
    }

    #[test]
    fn test_is_valid_rancid_password() {
        let valid_passwords = [
            "rancid",
            " foo",
            "bar ",
            "oekrmv03i4093mv3oim!?;",
            &"a".repeat(255),
        ];

        for pass in valid_passwords.iter() {
            assert!(validate_rancid_password(pass).is_ok());
        }
    }

    #[test]
    fn test_is_invalid_rancid_username() {
        let invalid_usernames = ["rancid}", " {foo", &"a".repeat(129)];

        for username in invalid_usernames.iter() {
            assert!(validate_rancid_username(username).is_err());
        }
    }

    #[test]
    fn test_is_valid_rancid_username() {
        let valid_usernames = [
            "rancid",
            " foo",
            "bar ",
            "oekrmv03i4093mv3oim!?;",
            &"a".repeat(128),
        ];

        for username in valid_usernames.iter() {
            assert!(validate_rancid_username(username).is_ok());
        }
    }

    #[test]
    fn test_invalid_other_addresses_fails() {
        let invalid_addresses = [
            "foo_bar,foo-bar,foo.bar,,",
            "foo_bar,foo-bar,foo.bar, ",
            "foo_bar,foo-bar,foo.bar, ,",
            "foo_bar,foo-bar,foo.bar, , ",
            ",foo_bar,foo-bar,foo.bar",
            " ,foo_bar,foo-bar,foo.bar",
            "192.168.1.1,https://example.com",
            "127.0.0.1,foo?bar",
            "localhost,foo@bar",
            "",
            " ",
            " ,",
            ",",
        ];

        for &address in &invalid_addresses {
            assert!(
                validate_and_trim_other_addresses(address).is_err(),
                "Expected '{}' to be invalid",
                address
            );
        }
    }

    #[test]
    fn test_is_valid_notification_profile_name() {
        // Test valid names
        assert!(validate_and_trim_notificationprofile_name("Notification Profile 1").is_ok());
        assert!(validate_and_trim_notificationprofile_name("N").is_ok());
        assert!(validate_and_trim_notificationprofile_name("N ").is_ok());
        assert!(validate_and_trim_notificationprofile_name(&"N".repeat(128)).is_ok());
        assert!(validate_and_trim_notificationprofile_name("N@").is_ok());

        // Test invalid names
        assert!(validate_and_trim_notificationprofile_name("").is_err());
        assert!(validate_and_trim_notificationprofile_name(" ").is_err());
        assert!(validate_and_trim_notificationprofile_name("N!").is_err());
        assert!(validate_and_trim_notificationprofile_name(&"N".repeat(129)).is_err());
    }
}
