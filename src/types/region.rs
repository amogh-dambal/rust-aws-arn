//! AWS regions

use crate::ArnError;

/// A list of known region identifiers from
/// [docs.aws](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-regions-availability-zones.html).
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::AsRefStr,
    strum::Display,
    strum::IntoStaticStr,
    strum::EnumString,
)]
#[strum(
    serialize_all = "kebab-case",
    parse_err_fn = convert_region_parse_err,
    parse_err_ty = ArnError,
    use_phf,
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Region {
    /// Corresponds to the region "af-south-1": Africa (Cape Town)
    #[strum(serialize = "af-south-1")]
    #[cfg(feature = "serde")]
    #[serde(rename = "af-south-1")]
    AfSouth1,

    /// Corresponds to the region "ap-east-1": Asia Pacific (Hong Kong)
    #[strum(serialize = "ap-east-1")]
    #[cfg(feature = "serde")]
    #[serde(rename = "ap-east-1")]
    ApEast1,

    /// Corresponds to the region "ap-northeast-1": Asia Pacific (Tokyo)
    #[strum(serialize = "ap-northeast-1")]
    #[cfg(feature = "serde")]
    #[serde(rename = "ap-northeast-1")]
    ApNortheast1,

    /// Corresponds to the region "ap-northeast-2": Asia Pacific (Seoul)
    #[strum(serialize = "ap-northeast-2")]
    #[cfg(feature = "serde")]
    #[serde(rename = "ap-northeast-2")]
    ApNortheast2,

    /// Corresponds to the region "ap-northeast-3": Asia Pacific (Osaka)
    #[strum(serialize = "ap-northeast-3")]
    #[cfg(feature = "serde")]
    #[serde(rename = "ap-northeast-3")]
    ApNortheast3,

    /// Corresponds to the region "ap-southeast-1": Asia Pacific (Singapore)
    #[strum(serialize = "ap-southeast-1")]
    #[cfg(feature = "serde")]
    #[serde(rename = "ap-southeast-1")]
    ApSoutheast1,

    /// Corresponds to the region "ap-southeast-2": Asia Pacific (Sydney)
    #[strum(serialize = "ap-southeast-2")]
    #[cfg(feature = "serde")]
    #[serde(rename = "ap-southeast-2")]
    ApSoutheast2,

    /// Corresponds to the region "ap-south-1": Asia Pacific (Mumbai)
    #[strum(serialize = "ap-south-1")]
    #[cfg(feature = "serde")]
    #[serde(rename = "ap-south-1")]
    ApSouth1,

    /// Corresponds to the region "ca-central-1": Canada (Central)
    #[strum(serialize = "ca-central-1")]
    #[cfg(feature = "serde")]
    #[serde(rename = "ca-central-1")]
    CaCentral1,

    /// Corresponds to the region "eu-central-1": Europe (Frankfurt)
    #[strum(serialize = "eu-central-1")]
    #[cfg(feature = "serde")]
    #[serde(rename = "eu-central-1")]
    EuCentral1,

    /// Corresponds to the region "eu-north-1": Europe (Stockholm)
    #[strum(serialize = "eu-north-1")]
    #[cfg(feature = "serde")]
    #[serde(rename = "eu-north-1")]
    EuNorth1,

    /// Corresponds to the region "eu-south-1": Europe (Milan)
    #[strum(serialize = "eu-south-1")]
    #[cfg(feature = "serde")]
    #[serde(rename = "eu-south-1")]
    EuSouth1,

    /// Corresponds to the region "eu-west-1": Europe (Ireland)
    #[strum(serialize = "eu-west-1")]
    #[cfg(feature = "serde")]
    #[serde(rename = "eu-west-1")]
    EuWest1,

    /// Corresponds to the region "eu-west-2": Europe (London)
    #[strum(serialize = "eu-west-2")]
    #[cfg(feature = "serde")]
    #[serde(rename = "eu-west-2")]
    EuWest2,

    /// Corresponds to the region "eu-west-3": Europe (Paris)
    #[strum(serialize = "eu-west-3")]
    #[cfg(feature = "serde")]
    #[serde(rename = "eu-west-3")]
    EuWest3,

    /// Corresponds to the region "me-south-1": Europe (Bahrain)
    #[strum(serialize = "me-south-1")]
    #[cfg(feature = "serde")]
    #[serde(rename = "me-south-1")]
    MeSouth1,

    /// Corresponds to the region "sa-east-1": South America (São Paulo)
    #[strum(serialize = "sa-east-1")]
    #[cfg(feature = "serde")]
    #[serde(rename = "sa-east-1")]
    SaEast1,

    /// Corresponds to the region "us-east-1": US East (N. Virginia)
    #[strum(serialize = "us-east-1")]
    #[cfg(feature = "serde")]
    #[serde(rename = "us-east-1")]
    UsEast1,

    /// Corresponds to the region "us-east-2": US East (Ohio)
    #[strum(serialize = "us-east-2")]
    #[cfg(feature = "serde")]
    #[serde(rename = "us-east-2")]
    UsEast2,

    /// Corresponds to the region "us-west-1": US West (N. California)
    #[strum(serialize = "us-west-1")]
    #[cfg(feature = "serde")]
    #[serde(rename = "us-west-1")]
    UsWest1,

    /// Corresponds to the region "us-west-2": US West (Oregon)
    #[strum(serialize = "us-west-2")]
    #[cfg(feature = "serde")]
    #[serde(rename = "us-west-2")]
    UsWest2,

    /// Corresponds to the region "us-gov-west-1": US Gov West
    #[strum(serialize = "us-gov-west-1")]
    #[cfg(feature = "serde")]
    #[serde(rename = "us-gov-west-1")]
    UsGovWest1,

    /// Corresponds to the region "us-gov-east-1": US Gov East
    #[strum(serialize = "us-gov-east-1")]
    #[cfg(feature = "serde")]
    #[serde(rename = "us-gov-east-1")]
    UsGovEast1,
}

fn convert_region_parse_err(r: &str) -> ArnError {
    ArnError::InvalidRegion(r.to_string())
}
