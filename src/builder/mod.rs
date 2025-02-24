//! A natural fluent builder interface for constructing [`ResourceName`]s.
//!
//! The builder pattern allows for a more readable construction of [`ResourceName`]s.
//! We provide a number of *verb* prefixes on *noun* constructors, e.g. `in_region` as well as
//! `and_region`, which is more readable if it is preceded by `in_partition`. For the account id
//! field, there is `in_account`, `and_account`, `any_account`, and `owned_by`; all of these
//! accomplish the same goal but allow for a choice that makes code easier to understand.
//!
//! # Resource-Specific Constructor Functions
//!
//! For the service-specific submodules (`iam`, `lambda`, `s3`, etc.) the functions are simply named
//! for the noun that represents the resource type as described in the AWS documentation. As the
//! partition in commonly left to default to "aws" there are also a set of `{noun}_in()` functions
//! that take a partition, and corresponding `{noun}()` functions which do not.
//!
//! In some cases where an ResourceName may be dependent on another, for example an S3 object ResourceName might be
//! constructed from an existing bucket ResourceName, additional `{noun}_from(other,...)` functions will
//! be provided.
//!
//! Note that the final `build()` function will call `validate()`, and so it is possible to call
//! intermediate functions with bad data which is only caught at build time.
//!
//! # Example
//!
//! The following shows the construction of an AWS versioned layer ResourceName.
//! ```rust
//! use aws_arn::builder::{ResourceNameBuilder, ResourceBuilder};
//! use aws_arn::{
//!     AccountId,
//!     Identifier,
//!     IdentifierLike,
//!     ResourceIdentifier,
//!     ResourceName,
//!     Region,
//!     Service
//! };
//! use std::str::FromStr;
//!
//! // 'arn:aws:lambda:us-east-2:123456789012:layer:my-layer:3'
//! let arn: ResourceName = ResourceName::builder()
//!     .service(Service::Lambda)
//!     .resource(
//!         ResourceBuilder::typed(Identifier::new_unchecked("layer"))
//!             .resource_name(Identifier::new_unchecked("my-layer"))
//!             .version(3)
//!             .build_qualified_id(),
//!     )
//!     .in_region(Region::UsEast2)
//!     .owned_by(AccountId::from_str("123456789012").unwrap())
//!     .build();
//! ```

pub use crate::ResourceNameBuilder;
use crate::{
    resource_name_builder::{IsUnset, SetInAccount, SetInRegion, SetResource, State},
    AccountId, Identifier, IdentifierLike, Region, ResourceIdentifier,
};

impl<S: State> ResourceNameBuilder<S> {
    /// Specifies the AWS region where the resource described by the ARN being built
    /// is located.
    pub fn and_region(self, region: impl Into<Region>) -> ResourceNameBuilder<SetInRegion<S>>
    where
        S::InRegion: IsUnset,
    {
        self.in_region(region)
    }

    /// Used for ARNs that describe resources that have no associated region, e.g.
    /// S3 buckets or IAM roles.
    pub fn in_any_region(self) -> ResourceNameBuilder<SetInRegion<S>>
    where
        S::InRegion: IsUnset,
    {
        self.maybe_in_region(None::<Region>)
    }

    /// Specifies the AWS account that owns the resource for which we are constructing an ARN.
    pub fn and_account(self, account: impl Into<AccountId>) -> ResourceNameBuilder<SetInAccount<S>>
    where
        S::InAccount: IsUnset,
    {
        self.in_account(account)
    }

    /// A more readable alias for `and_account`
    pub fn owned_by(self, account: impl Into<AccountId>) -> ResourceNameBuilder<SetInAccount<S>>
    where
        S::InAccount: IsUnset,
    {
        self.in_account(account)
    }

    /// Specifies the AWS resource being described by the AWS ARN.
    pub fn is(self, resource: impl Into<ResourceIdentifier>) -> ResourceNameBuilder<SetResource<S>>
    where
        S::Resource: IsUnset,
    {
        self.resource(resource)
    }
}

/// Builder type for a `ResourceIdentifier`.
///
/// The methods `build_resource_path` and `build_qualified_id` are used to construct identifiers
/// with either the '/' or ':' separator between the collected components.
///
#[derive(Clone, Debug, Default)]
pub struct ResourceBuilder {
    resource: Vec<ResourceIdentifier>,
}

impl From<ResourceIdentifier> for ResourceBuilder {
    fn from(v: ResourceIdentifier) -> Self {
        Self { resource: vec![v] }
    }
}

impl From<Identifier> for ResourceBuilder {
    fn from(v: Identifier) -> Self {
        Self {
            resource: vec![v.into()],
        }
    }
}

impl ResourceBuilder {
    /// Construct a new resource builder containing only the provided identifier.
    pub fn named(id: Identifier) -> Self {
        Self {
            resource: vec![id.into()],
        }
    }

    /// Construct a new resource builder containing only the provided identifier.
    pub fn typed(id: Identifier) -> Self {
        Self {
            resource: vec![id.into()],
        }
    }

    /// Add the provided `ResourceIdentifier` to the inner list of components.
    pub fn add(&mut self, id: ResourceIdentifier) -> &mut Self {
        self.resource.push(id);
        self
    }

    /// Add the provided `ResourceIdentifier` to the inner list of components.
    pub fn qualified_name(&mut self, id: ResourceIdentifier) -> &mut Self {
        self.resource.push(id);
        self
    }

    /// Add the provided `ResourceIdentifier` to the inner list of components.
    pub fn resource_path(&mut self, id: ResourceIdentifier) -> &mut Self {
        self.resource.push(id);
        self
    }

    /// Add the provided `Identifier` to the inner list of components.
    pub fn type_name(&mut self, id: Identifier) -> &mut Self {
        self.resource.push(id.into());
        self
    }

    /// Add the provided `Identifier` to the inner list of components.
    pub fn resource_name(&mut self, id: Identifier) -> &mut Self {
        self.resource.push(id.into());
        self
    }

    /// Add the provided `Identifier` to the inner list of components.
    pub fn sub_resource_name(&mut self, id: Identifier) -> &mut Self {
        self.resource.push(id.into());
        self
    }

    /// Add the provided integer version number to the inner list of components.
    pub fn version(&mut self, v: u32) -> &mut Self {
        self.resource
            .push(Identifier::new_unchecked(&v.to_string()).into());
        self
    }

    /// Return the iner list of components as a resource identifier path.
    pub fn build_resource_path(&mut self) -> ResourceIdentifier {
        ResourceIdentifier::from_path(&self.resource)
    }

    /// Return the iner list of components as a qualified resource identifier.
    pub fn build_qualified_id(&mut self) -> ResourceIdentifier {
        ResourceIdentifier::from_qualified(&self.resource)
    }
}

pub mod cognito;
pub mod iam;
pub mod lambda;
pub mod s3;
