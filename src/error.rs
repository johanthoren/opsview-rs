#![allow(missing_docs)]
use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum OpsviewError {
    #[error("Client error: {0}")]
    ClientError(OpsviewClientError),

    #[error("Config error: {0}")]
    ConfigError(OpsviewConfigError),

    #[error("Undefined error: {0}")]
    UndefinedError(String),
}

impl From<OpsviewClientError> for OpsviewError {
    fn from(err: OpsviewClientError) -> Self {
        OpsviewError::ClientError(err)
    }
}

impl From<OpsviewConfigError> for OpsviewError {
    fn from(err: OpsviewConfigError) -> Self {
        OpsviewError::ConfigError(err)
    }
}

impl From<serde_json::Error> for OpsviewError {
    fn from(err: serde_json::Error) -> Self {
        OpsviewError::ClientError(OpsviewClientError::JsonParseError(err.to_string()))
    }
}

impl From<&str> for OpsviewError {
    fn from(err: &str) -> Self {
        OpsviewError::UndefinedError(err.to_string())
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum OpsviewClientError {
    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Field not found: {0}")]
    FieldNotFound(String),

    #[error("HTTP error: {0}")]
    HttpError(String),

    #[error("ID not found: {0}")]
    IdNotFound(String),

    #[error("Unable to parse ID: {0}")]
    IdParseError(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(String),

    #[error("Invalid reference '{0}': {1}")]
    InvalidRef(String, String),

    #[error("Unable to parse JSON: {0}")]
    JsonParseError(String),

    #[error("Missing argument: {0}")]
    MissingArgument(String),

    #[error("Missing identifiers: {0}")]
    MissingIdentifiers(String),

    #[error("No config path set")]
    NoConfigPath,

    #[error("Not an array: {0}")]
    NotAnArray(String),

    #[error("Object not found: {0}")]
    ObjectNotFound(String),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Unable to parse response: {0}")]
    ResponseParseError(String),

    #[error("Unable to parse '{0}' as '{1}'")]
    TypeParseError(String, String),

    #[error("Undefined error: {0}")]
    UndefinedError(String),

    #[error("There are unsaved changes")]
    UnsavedChanges,

    #[error("URL parse error: {0}")]
    UrlParseError(String),
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum OpsviewConfigError {
    #[error("Invalid value: {0} does not match regex {1}")]
    DoesNotMatchRegex(String, String),

    #[error("Duplicate options found: {0}")]
    DuplicateOptions(String),

    #[error("Conflicting options found: {0}")]
    ConflictingOptions(String),

    #[error("Forbidden character found: {0}")]
    ForbiddenCharacter(char),

    #[error("Invalid calculate_rate: Must be one of 'no', 'per_second', 'per_minute', 'per_hour'")]
    InvalidCalculateRate,

    #[error("Invalid IP: {0}")]
    InvalidIP(String),

    #[error("Invalid notification_options: {0} does not conform to the format 'u,d,r,f'")]
    InvalidNotificationOptions(String),

    #[error("Failed to parse percentage: '{0}' with error '{1}'")]
    InvalidPercentage(String, String),

    #[error("Invalid Plugin origin_id: must be either '0' or '1'")]
    InvalidPluginOriginID,

    #[error("Invalid quorum: {0}")]
    InvalidQuorum(String),

    #[error("Invalid SNMP configuration: {0}")]
    InvalidSNMPConfig(String),

    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),

    #[error("UTF-8 error, contains a character that is forbidden in this field: {0}")]
    InvalidUtf8(char),

    #[error("label{0} must not be empty or None when arg{0} is set")]
    MissingArgLabel(u8),

    #[error("arg{0} must not be empty or None when secured{0} is true")]
    MissingSecuredArg(u8),

    #[error("label{0} must not be empty or None when secured{0} is true")]
    MissingSecuredLabel(u8),

    #[error("The port '{0}' is out of range (0-65535)")]
    PortOutOfRange(u64),

    #[error("Found multiple entries of option in '{0}'")]
    RepeatingNotificationOptions(String),

    #[error("Mandatory field '{0}' cannot be empty")]
    RequiredFieldEmpty(String),

    #[error("String too long, expected at most {0} characters, got {1}")]
    StringTooLong(usize, usize),

    #[error("String too short, expected at least {0} characters, got {1}")]
    StringTooShort(usize, usize),

    #[error("String too long when URI encoded, expected at most {0} characters, got {1}")]
    StringTooLongWhenPercentEncoded(usize, usize),

    #[error("State out of range: {0}")]
    StateOutOfRange(u64),

    #[error("Unknown FontAwesomeIcon: {0}")]
    UnknownFontAwesomeIcon(String),

    #[error("Unable to parse URL: {0}")]
    UrlParseError(url::ParseError),
}

impl From<reqwest::Error> for OpsviewClientError {
    fn from(err: reqwest::Error) -> Self {
        OpsviewClientError::HttpError(err.to_string())
    }
}

impl From<url::ParseError> for OpsviewClientError {
    fn from(err: url::ParseError) -> Self {
        OpsviewClientError::UrlParseError(err.to_string())
    }
}

impl From<std::num::ParseIntError> for OpsviewClientError {
    fn from(err: std::num::ParseIntError) -> Self {
        OpsviewClientError::ResponseParseError(err.to_string())
    }
}

impl From<serde_json::Error> for OpsviewClientError {
    fn from(err: serde_json::Error) -> Self {
        OpsviewClientError::JsonParseError(err.to_string())
    }
}

impl From<reqwest::header::InvalidHeaderValue> for OpsviewClientError {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
        OpsviewClientError::InvalidHeaderValue(err.to_string())
    }
}

impl From<&str> for OpsviewClientError {
    fn from(err: &str) -> Self {
        OpsviewClientError::UndefinedError(err.to_string())
    }
}

impl From<url::ParseError> for OpsviewConfigError {
    fn from(err: url::ParseError) -> Self {
        OpsviewConfigError::UrlParseError(err)
    }
}
