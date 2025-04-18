//! Provides structured types, validation + correctness, builders, and other utilities
//! to manipulate [ARNs](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html)
//! (Amazon Resource Names).
//!
//! The ARN is a key component of all AWS service APIs, yet nearly
//! all client toolkits treat it as a string. While this may be a
//! reasonable and expedient decision, there are use-cases that require
//! applications and libraries to ensure the validity and correctness
//! of AWS ARNs.
//!
//! # `ResourceName` types
//! This crate provides multiple interfaces to manipulate `ResourceName`s.
//! The first is the direct construction of an `ResourceName` using the core types:
//! [`ResourceName`], [`AccountId`], [`Partition`], [`Service`], and [`ResourceIdentifier`].
//!
//! ```rust
//! use aws_arn::{ResourceName, ResourceIdentifier};
//! use aws_arn::{Partition, Region, Service};
//! use std::str::FromStr;
//!
//! let arn = ResourceName {
//!     partition: Partition::default(),
//!     service: Service::S3,
//!     region: None,
//!     account_id: None,
//!     resource: ResourceIdentifier::from_str("my-s3-bucket-name").unwrap(),
//! };
//! ```
//!
//! In the example above, as we are defining a minimal ResourceName. However, we could
//! also use one of the defined constructor functions.
//!
//! ```rust
//! use aws_arn::{ResourceName, ResourceIdentifier};
//! use aws_arn::Service;
//! use std::str::FromStr;
//!
//! let arn = ResourceName::aws(
//!     Service::S3,
//!     ResourceIdentifier::from_str("mythings/thing-1").unwrap()
//! );
//! ```
//!
//! Alternatively, using `FromStr,` you can parse a ResourceName value directly from a
//! `String` or `&str`.
//!
//! ```rust
//! use aws_arn::ResourceName;
//! use std::str::FromStr;
//!
//! let arn: ResourceName = "arn:aws:s3:::mythings/thing-1"
//!     .parse()
//!     .expect("didn't look like an ResourceName");
//! ```
//!
//! For more, see the AWS documentation for [Amazon Resource Name
//! (ResourceName)](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html) documentation.
//!
//! # Features
//! * `serde`: enables (de)serialization using [`serde`](). This feature is enabled by default.
//! * `builders`: enables fluent builders using [`bon`](). This feature is enabled by default.
//!/

#![warn(
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

#[cfg(feature = "builders")]
use bon::builder;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

use std::str::FromStr;

mod types;
pub use types::{
    AccountId, AccountIdentifier, Identifier, IdentifierLike, Partition, Region,
    ResourceIdentifier, Service,
};
use types::{ARN_PREFIX, PART_SEPARATOR, REQUIRED_COMPONENT_COUNT};

/// Amazon Resource Names (Arns) uniquely identify AWS resources. We require an ResourceName when you
/// need to specify a resource unambiguously across all of AWS, such as in IAM policies,
/// Amazon Relational Database Service (Amazon RDS) tags, and API calls.
///
/// The following are the general formats for Arns; the specific components and values used
/// depend on the AWS service.
///
/// ```text
/// arn:partition:service:region:account-id:resource-id
/// arn:partition:service:region:account-id:resource-type/resource-id
/// arn:partition:service:region:account-id:resource-type:resource-id
/// ```
///
/// From [ResourceName Format](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arns-syntax)
///
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "builders", derive(bon::Builder))]
pub struct ResourceName {
    /// The partition that the resource is in. For standard AWS Regions, the partition is` aws`.
    /// If you have resources in other partitions, the partition is `aws-partitionname`. For
    /// example, the partition for resources in the China partition is `aws-cn`.
    #[cfg(feature = "builders")]
    #[builder(into, name = "in_partition", default = Partition::Aws)]
    pub partition: Partition,
    /// The service namespace that identifies the AWS service.
    #[cfg(feature = "builders")]
    #[builder(into)]
    pub service: Service,
    /// The AWS region that the resource resides in. Some resources - like S3 buckets - are considered
    /// "global", and thus the ARN does not require a region.
    #[cfg(feature = "builders")]
    #[builder(into, name = "in_region")]
    pub region: Option<Region>,
    /// The ID of the AWS account that owns the resource, without the hyphens. For example,
    /// `123456789012`. Some resources, like S3 buckets, have ARNs that do not include the AWS
    /// account ID.
    #[cfg(feature = "builders")]
    #[builder(into, name = "in_account")]
    pub account_id: Option<AccountIdentifier>,
    /// The content of this part of the ResourceName varies by service. A resource identifier can
    /// be the name or ID of the resource (for example, `user/Bob` or
    /// `instance/i-1234567890abcdef0`) or a resource path. For example, some resource
    /// identifiers include a parent resource
    /// (`sub-resource-type/parent-resource/sub-resource`) or a qualifier such as a
    /// version (`resource-type:resource-name:qualifier`).
    #[cfg(feature = "builders")]
    #[builder(into)]
    pub resource: ResourceIdentifier,
}

impl Display for ResourceName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let region = self
            .region
            .clone()
            .map_or(String::from(""), |val| val.to_string());
        let account_id = self
            .account_id
            .clone()
            .map_or(String::from(""), |val| val.to_string());

        write!(
            f,
            "{}:{}:{}:{}:{}:{}",
            ARN_PREFIX, self.partition, self.service, region, account_id, self.resource,
        )
    }
}

impl FromStr for ResourceName {
    type Err = ArnError;

    ///
    /// Format:
    ///
    /// * `arn:partition:service:region:account-id: | resource part |`
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(REQUIRED_COMPONENT_COUNT, PART_SEPARATOR).collect();
        if parts.len() < REQUIRED_COMPONENT_COUNT {
            return Err(ArnError::TooFewComponents(parts.len()));
        } else if parts[0] != ARN_PREFIX {
            return Err(ArnError::MissingPrefix);
        }

        let partition = Partition::from_str(parts[1])?;
        let service = Service::from_str(parts[2])?;
        let region = match parts[3] {
            "" => None,
            region => Some(Region::from_str(region)?),
        };
        let account_id = match parts[4] {
            "" => None,
            account_id => Some(AccountIdentifier::from_str(account_id)?),
        };
        let resource = ResourceIdentifier::from_str(parts[5])?;

        Ok(ResourceName {
            account_id,
            partition,
            region,
            service,
            resource,
        })
    }
}

impl From<AccountId> for ResourceName {
    fn from(account: AccountId) -> Self {
        ResourceName {
            account_id: Some(AccountIdentifier::Account(account)),
            partition: Partition::Aws,
            region: None,
            resource: ResourceIdentifier::from_str("root").unwrap(),
            service: Service::IdentityAccessManagement,
        }
    }
}

impl ResourceName {
    /// Construct a minimal `ResourceName` value with simply a service and resource.
    pub fn new(service: Service, resource: ResourceIdentifier) -> Self {
        Self {
            partition: Partition::Aws,
            service,
            region: None,
            account_id: None,
            resource,
        }
    }

    /// Construct a minimal `ResourceName` value with simply a service and resource in the `aws` partition.
    pub fn aws(service: Service, resource: ResourceIdentifier) -> Self {
        Self {
            partition: Partition::Aws,
            service,
            region: None,
            account_id: None,
            resource,
        }
    }

    /// Return `true` if the identifier contains variables of the form
    /// `${name}`, else `false`.
    pub fn has_variables(&self) -> bool {
        self.resource.has_variables()
    }

    /// Replace any variables in the string with values from the context,
    /// returning a new value if the replacements result in a legal identifier
    /// string.
    pub fn replace_variables<V>(&self, context: &HashMap<String, V>) -> ArnResult<Self>
    where
        V: Clone + Into<String>,
    {
        Ok(Self {
            resource: self.resource.replace_variables(context)?,
            ..self.clone()
        })
    }
}

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

#[cfg(feature = "builders")]
pub mod builder;

#[doc(hidden)]
mod error;
pub use error::{ArnError, ArnResult};
