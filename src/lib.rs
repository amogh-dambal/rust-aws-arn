//! Provides structured types, validation + correctness, builders, and other utilities
//! to manipulate [ARNs](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html)
//! (Amazon Resource Names).
//!
//! The ARN is a key component of all AWS service APIs, yet nearly
//! all client toolkits treat it as a string. While this may be a
//! reasonable and expedient decision, there is a need to ensure validity and correctness
//! of AWS ARN
//! AWS ARNs for it seems there might be a need to not
//! only ensure correctness of ResourceNames with validators but also
//! constructors that allow making these strings correclt in the first place.
//!
//! # `Arn` types
//! This crate provides multiple interfaces to manipulate `Arn`s.
//! The first is the direct construction of an `Arn` using the core types:
//! [`Arn`], [`AccountIdentifier`], [`Partition`], [`Service`], and [`ResourceIdentifier`].
//!
//! ```rust
//! use aws_arn::{ResourceName, ResourceIdentifier};
//! use aws_arn::types::{Partition, Region, Service};
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
//! use aws_arn::types::Service;
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

use regex::{Captures, Regex};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;
use std::sync::LazyLock;
use types::{Partition, Region, Service};

/// This trait is implemented by the `ResourceName` component types. It
/// represents a string-based identifier that is generally constructed using
/// `FromStr::from_str`.
///
pub trait IdentifierLike
where
    Self: Clone + Display + FromStr + Deref<Target = str>,
{
    /// Construct a new `Identifier` from the provided string **without** checking it's validity.
    /// This can be a useful method to improve performance for statically, or well-known, values;
    /// however, in general `FromStr::from_str` should be used.
    fn new_unchecked(s: &str) -> Self
    where
        Self: Sized;

    /// Returns `true` if the provided string is a valid `Identifier` value, else `false`.
    fn is_valid(s: &str) -> bool;

    /// Construct an account identifier that represents *any*.
    fn any() -> Self {
        Self::new_unchecked(STRING_WILD_ANY)
    }

    /// Return `true` if this is simply the *any* wildcard, else `false`.
    fn is_any(&self) -> bool {
        self.deref().chars().any(|c| c == CHAR_WILD_ANY)
    }

    /// Returns `true` if this identifier contains any wildcard characeters,
    /// else `false`.
    fn has_wildcards(&self) -> bool {
        self.deref()
            .chars()
            .any(|c| c == CHAR_WILD_ONE || c == CHAR_WILD_ANY)
    }

    /// Return `true` if this identifier has no wildcards, else `false`.
    fn is_plain(&self) -> bool {
        !self.has_wildcards()
    }
}

///
/// A string value that is used to capture the partition, service, and region components
/// of an ResourceName. These are ASCII only, may not include control characters, spaces, '/', or ':'.
///
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Identifier(String);

///
/// A string value that is used to capture the account ID component
/// of an ResourceName. These are ASCII digits only and a fixed length of 12 characters.
///
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AccountIdentifier(String);

///
/// A string value that is used to capture the resource component of an ResourceName. These are ASCII only,
/// may not include control characters but unlike `Identifier` they may include spaces, '/', and ':'.
///
/// > *The content of this part of the ResourceName varies by service. A resource identifier can be the name
/// > or ID of the resource (for example, `user/Bob` or `instance/i-1234567890abcdef0`) or a
/// > resource path. For example, some resource identifiers include a parent resource
/// > (`sub-resource-type/parent-resource/sub-resource`) or a qualifier such as a version
/// > (`resource-type:resource-name:qualifier`).*
///
/// > *Some resource ResourceNames can include a path. For example, in Amazon S3, the resource identifier
/// > is an object name that can include slashes ('/') to form a path. Similarly, IAM user names
/// > and group names can include paths.*
///
/// > *In some circumstances, paths can include a wildcard character, namely an asterisk ('*').*
///
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ResourceIdentifier(String);

///
/// Amazon Resource Names (ResourceNames) uniquely identify AWS resources. We require an ResourceName when you
/// need to specify a resource unambiguously across all of AWS, such as in IAM policies,
/// Amazon Relational Database Service (Amazon RDS) tags, and API calls.
///
/// The following are the general formats for ResourceNames; the specific components and values used
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
pub struct ResourceName {
    /// The partition that the resource is in. For standard AWS Regions, the partition is` aws`.
    /// If you have resources in other partitions, the partition is `aws-partitionname`. For
    /// example, the partition for resources in the China partition is `aws-cn`.
    pub partition: Partition,
    /// The service namespace that identifies the AWS service.
    pub service: Service,
    /// The AWS region that the resource resides in. Some resources - like S3 buckets - are considered
    /// "global", and thus the ARN does not require a region.
    pub region: Option<Region>,
    /// The ID of the AWS account that owns the resource, without the hyphens. For example,
    /// `123456789012`. Some resources, like S3 buckets, have ARNs that do not include the AWS
    /// account ID.
    pub account_id: Option<AccountIdentifier>,
    /// The content of this part of the ResourceName varies by service. A resource identifier can
    /// be the name or ID of the resource (for example, `user/Bob` or
    /// `instance/i-1234567890abcdef0`) or a resource path. For example, some resource
    /// identifiers include a parent resource
    /// (`sub-resource-type/parent-resource/sub-resource`) or a qualifier such as a
    /// version (`resource-type:resource-name:qualifier`).
    pub resource: ResourceIdentifier,
}

const ARN_PREFIX: &str = "arn";

const PART_SEPARATOR: char = ':';
const PATH_SEPARATOR: char = '/';

const STRING_WILD_ANY: &str = "*";

const CHAR_ASCII_START: char = '\u{1F}';
const CHAR_ASCII_END: char = '\u{7F}';
const CHAR_SPACE: char = ' ';
const CHAR_WILD_ONE: char = '?';
const CHAR_WILD_ANY: char = '*';

const REQUIRED_COMPONENT_COUNT: usize = 6;

static REGEX_VARIABLE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\$\{([^$}]+)\}").expect("failed to initialize regex"));

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Identifier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(Error::InvalidIdentifier(s.to_string()))
        }
    }
}

impl From<Identifier> for String {
    fn from(v: Identifier) -> Self {
        v.0
    }
}

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IdentifierLike for Identifier {
    fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

    fn is_valid(s: &str) -> bool {
        !s.is_empty()
            && s.chars().all(|c| {
                c > CHAR_ASCII_START
                    && c < CHAR_ASCII_END
                    && c != CHAR_SPACE
                    && c != PATH_SEPARATOR
                    && c != PART_SEPARATOR
            })
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for AccountIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for AccountIdentifier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(Error::InvalidAccountId(s.to_string()))
        }
    }
}

impl From<AccountIdentifier> for String {
    fn from(v: AccountIdentifier) -> Self {
        v.0
    }
}

impl From<AccountIdentifier> for ResourceName {
    fn from(account: AccountIdentifier) -> Self {
        ResourceName::from_str(&format!("arn:aws:iam::{}:root", account)).unwrap()
    }
}

impl Deref for AccountIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IdentifierLike for AccountIdentifier {
    fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

    fn is_valid(s: &str) -> bool {
        (s.len() == 12 && s.chars().all(|c| c.is_ascii_digit()))
            || (!s.is_empty()
                && s.len() <= 12
                && s.chars()
                    .all(|c| c.is_ascii_digit() || c == CHAR_WILD_ONE || c == CHAR_WILD_ANY)
                && s.chars().any(|c| c == CHAR_WILD_ONE || c == CHAR_WILD_ANY))
    }
}

impl Display for ResourceIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ResourceIdentifier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(Error::InvalidResource(s.to_string()))
        }
    }
}

impl From<ResourceIdentifier> for String {
    fn from(v: ResourceIdentifier) -> Self {
        v.0
    }
}

impl From<Identifier> for ResourceIdentifier {
    fn from(v: Identifier) -> Self {
        ResourceIdentifier::new_unchecked(&v.0)
    }
}

impl Deref for ResourceIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IdentifierLike for ResourceIdentifier {
    fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

    fn is_valid(s: &str) -> bool {
        !s.is_empty() && s.chars().all(|c| c > '\u{1F}' && c < '\u{7F}')
    }

    fn is_plain(&self) -> bool {
        !self.has_wildcards() && !self.has_variables()
    }
}

impl ResourceIdentifier {
    /// Construct a resource identifier, as a path, using the `Identifier` path components.
    pub fn from_id_path(path: &[Identifier]) -> Self {
        Self::new_unchecked(
            &path
                .iter()
                .map(Identifier::to_string)
                .collect::<Vec<String>>()
                .join(&PATH_SEPARATOR.to_string()),
        )
    }

    /// Construct a resource identifier, as a qualified ID, using the `Identifier` path components.
    pub fn from_qualified_id(qualified: &[Identifier]) -> Self {
        Self::new_unchecked(
            &qualified
                .iter()
                .map(Identifier::to_string)
                .collect::<Vec<String>>()
                .join(&PART_SEPARATOR.to_string()),
        )
    }

    /// Construct a resource identifier, as a path, using the `ResourceIdentifier` path components.
    pub fn from_path(path: &[ResourceIdentifier]) -> Self {
        Self::new_unchecked(
            &path
                .iter()
                .map(ResourceIdentifier::to_string)
                .collect::<Vec<String>>()
                .join(&PATH_SEPARATOR.to_string()),
        )
    }

    /// Construct a resource identifier, as a qualified ID, using the `ResourceIdentifier` path components.
    pub fn from_qualified(qualified: &[ResourceIdentifier]) -> Self {
        Self::new_unchecked(
            &qualified
                .iter()
                .map(ResourceIdentifier::to_string)
                .collect::<Vec<String>>()
                .join(&PART_SEPARATOR.to_string()),
        )
    }

    /// Return `true` if this identifier contains path separator characters, else `false`.
    pub fn contains_path(&self) -> bool {
        self.0.contains(PATH_SEPARATOR)
    }

    /// Return the list of path components when split using the path separator character.
    pub fn path_split(&self) -> Vec<ResourceIdentifier> {
        self.0
            .split(PATH_SEPARATOR)
            .map(ResourceIdentifier::new_unchecked)
            .collect()
    }

    /// Return `true` if this identifier contains qualifier separator characters, else `false`.
    pub fn contains_qualified(&self) -> bool {
        self.0.contains(PART_SEPARATOR)
    }

    /// Return the list of path components when split using the qualifier separator character.
    pub fn qualifier_split(&self) -> Vec<ResourceIdentifier> {
        self.0
            .split(PART_SEPARATOR)
            .map(ResourceIdentifier::new_unchecked)
            .collect()
    }

    /// Return `true` if the identifier contains variables of the form
    /// `${name}`, else `false`.
    pub fn has_variables(&self) -> bool {
        REGEX_VARIABLE.is_match(self.deref())
    }

    /// Replace any variables in the string with values from the context,
    /// returning a new value if the replacements result in a legal identifier
    /// string. The
    pub fn replace_variables<V>(&self, context: &HashMap<String, V>) -> Result<Self, Error>
    where
        V: Clone + Into<String>,
    {
        let new_text = REGEX_VARIABLE.replace_all(self.deref(), |caps: &Captures<'_>| {
            if let Some(value) = context.get(&caps[1]) {
                value.clone().into()
            } else {
                format!("${{{}}}", &caps[1])
            }
        });
        Self::from_str(&new_text)
    }
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
    type Err = Error;

    ///
    /// Format:
    ///
    /// * `arn:partition:service:region:account-id: | resource part |`
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(REQUIRED_COMPONENT_COUNT, PART_SEPARATOR).collect();
        if parts.len() < REQUIRED_COMPONENT_COUNT {
            return Err(Error::TooFewComponents);
        } else if parts[0] != ARN_PREFIX {
            return Err(Error::MissingPrefix);
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
    /// string. The
    pub fn replace_variables<V>(&self, context: &HashMap<String, V>) -> Result<Self, Error>
    where
        V: Clone + Into<String>,
    {
        Ok(Self {
            resource: self.resource.replace_variables(context)?,
            ..self.clone()
        })
    }
}

// #[cfg(doctest)]
// doc_comment::doctest!("../README.md");

pub mod types;

#[cfg(feature = "builders")]
pub mod builder;

#[doc(hidden)]
mod error;
pub use error::Error;
