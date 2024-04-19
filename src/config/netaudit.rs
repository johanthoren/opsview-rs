use crate::{prelude::*, util::*};
use serde::{Deserialize, Serialize};

/// Represents a type of RANCID connection.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RancidConnectionType {
    /// Use Telnet to connect to the NetAudit host.
    Telnet,
    /// Use SSH to connect to the NetAudit host.
    SSH,
}

/// Represents a [RANCID vendor](https://docs.itrsgroup.com/docs/opsview/6.8.9/monitoring/net-audit/netaudit/index.html#Heading-overview) in Opsview.
///
/// The `RancidVendor` struct is used to define RANCID vendors in the Opsview system.
/// It encapsulates the necessary information to identify and describe a RANCID vendor.
/// There is no API endpoint for managing RANCID vendors, so this struct is only used as a
/// component of other entities.
///
/// This struct defines the structure for a RANCID vendor entity as used in Opsview.
///
/// # Example
/// ```rust
/// use opsview::config::RancidVendor;
/// use opsview::prelude::*;
///
/// let rancid_vendor = RancidVendor::builder()
///    .name("My RANCID Vendor")
///    .ref_("/rest/config/rancidvendor/1234")
///    .build()
///    .unwrap();
///
/// assert_eq!(rancid_vendor.name, "My RANCID Vendor".to_string());
/// ```
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct RancidVendor {
    /// The name of the `RancidVendor`. This is used as a unique identifier in Opsview.
    pub name: String,

    /// An optional reference string unique to this `RancidVendor`.
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
}

/// Enables the creation of a `RancidVendor` instance from a JSON representation.
/// Typically used when parsing JSON data from the Opsview API.
impl CreateFromJson for RancidVendor {}

impl ConfigObject for RancidVendor {
    type Builder = RancidVendorBuilder;

    /// Returns a builder for constructing a [`RancidVendor`] object.
    ///
    /// # Returns
    /// A [`RancidVendorBuilder`] object.
    fn builder() -> Self::Builder {
        RancidVendorBuilder::new()
    }

    /// Returns the unique name of the [`RancidVendor`] object.
    ///
    /// This name is used to identify the `RancidVendor` when building the `HashMap` for a
    /// [`ConfigObjectMap`].
    ///
    /// # Returns
    /// A string representing the unique name of the `RancidVendor` entity.
    fn unique_name(&self) -> String {
        self.name.clone()
    }
}

/// Builder for creating instances of [`RancidVendor`].
///
/// Provides a fluent interface for constructing an `RancidVendor` object with optional fields.
#[derive(Clone, Debug, Default)]
pub struct RancidVendorBuilder {
    name: Option<String>,
    ref_: Option<String>,
}

impl Builder for RancidVendorBuilder {
    type ConfigObject = RancidVendor;

    /// Creates a new instance of [`RancidVendorBuilder`] with default values.
    fn new() -> Self {
        RancidVendorBuilder {
            name: None,
            ref_: None,
        }
    }

    /// Sets the name of the `RancidVendor` object.
    ///
    /// # Arguments
    /// * `name` - The name of the `RancidVendor` object.
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Consumes the builder and returns a [`RancidVendor`] object.
    ///
    /// # Returns
    /// A `RancidVendor` object constructed from the values provided to the builder.
    ///
    /// # Errors
    /// Returns an `Error` if the name is not set.
    fn build(self) -> Result<Self::ConfigObject, OpsviewConfigError> {
        let name = require_field(&self.name, "name")?;
        Ok(RancidVendor {
            name: validate_and_trim_rancid_vendor_name(&name)?,
            ref_: self.ref_,
        })
    }
}

impl RancidVendorBuilder {
    /// Sets the reference string of the `RancidVendor` object.
    ///
    /// # Arguments
    /// * `ref_` - The reference string of the `RancidVendor` object.
    pub fn ref_(mut self, ref_: &str) -> Self {
        self.ref_ = Some(ref_.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let vendor = RancidVendor::default();

        assert_eq!(vendor.name, "".to_string());
    }

    #[test]
    fn test_minimal() {
        let vendor = RancidVendor::minimal("AXIS T8504R");

        assert_eq!(vendor.unwrap().name, "AXIS T8504R".to_string());
    }

    #[test]
    fn test_serialize_rancid_connection_type() {
        let telnet = RancidConnectionType::Telnet;
        let ssh = RancidConnectionType::SSH;

        assert_eq!(serde_json::to_string(&telnet).unwrap(), "\"telnet\"");
        assert_eq!(serde_json::to_string(&ssh).unwrap(), "\"ssh\"");
    }

    #[test]
    fn test_deserialize_rancid_connection_type() {
        let telnet = RancidConnectionType::Telnet;
        let ssh = RancidConnectionType::SSH;

        assert_eq!(
            serde_json::from_str::<RancidConnectionType>("\"telnet\"").unwrap(),
            telnet
        );
        assert_eq!(
            serde_json::from_str::<RancidConnectionType>("\"ssh\"").unwrap(),
            ssh
        );
    }
}
