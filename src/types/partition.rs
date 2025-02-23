//! AWS partition information.

use crate::Error;

/// A list of known partition identifiers from
/// [docs.aws](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html).
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::AsRefStr,
    strum::Display,
    strum::EnumString,
    strum::IntoStaticStr,
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[strum(
    parse_err_fn = convert_partition_parse_err,
    parse_err_ty = Error,
)]
pub enum Partition {
    /// Corresponds to the partition "aws": AWS region
    #[serde(rename = "aws")]
    #[strum(serialize = "aws")]
    #[default]
    Aws,

    /// Corresponds to the partition "aws-cn": AWS China regions
    #[serde(rename = "aws-cn")]
    #[strum(serialize = "aws-cn")]
    AwsChina,

    /// Corresponds to the partition "aws-us-gov": AWS GovCloud (US) regions
    #[serde(rename = "aws-us-gov")]
    #[strum(serialize = "aws-us-gov")]
    AwsUsGov,
}

fn convert_partition_parse_err(_: &str) -> Error {
    Error::InvalidPartition
}
