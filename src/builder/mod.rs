//! Provides a more natural builder interface for constructing Arns.
//!
//! The builder pattern allows for a more readable construction of Arns, and in this case we
//! provide a number of *verb* prefixes on *noun* constructors, so we have `in_region` as well as
//! `and_region` which is more readable if it is preceded by `in_partition`. For the account id
//! field there is `in_account`, `and_account`, `any_account`, and `owned_by`; all of these
//! accomplish the same goal but allow for a choice that makes code easir to understand.
//!
//! # Resource-Specific Constructor Functions
//!
//! For the service-specific submodules (`iam`, `lambda`, `s3`, etc.) the functions are simply named
//! for the noun that represents the resource type as described in the AWS documentation. As the
//! partition in commonly left to default to "aws" there are also a set of `{noun}_in()` functions
//! that take a partition, and corresponding `{noun}()` functions which do not.
//!
//! In some cases where an Arn may be dependent on another, for example an S3 object Arn might be
//! constructed from an existing bucket Arn, additional `{noun}_from(other,...)` functions will
//! be provided.
//!
//! Note that the final `build()` function will call `validate()`, and so it is possible to call
//! intermediate functions with bad data which is only caught at build time.
//!
//! # Example
//!
//! The following shows the construction of an AWS versioned layer Arn.
//!
//!

use crate::known::{Partition, Region, Service};
use crate::types::Partition;
use crate::{AccountIdentifier, Arn, Identifier, IdentifierLike, ResourceIdentifier};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Builder type for an AWS `Arn`.
///
#[derive(Clone, Debug)]
pub struct ArnBuilder {
    arn: Arn,
}

///
/// Builder type for a `ResourceIdentifier`.
///
/// The methods `build_resource_path` and `build_qualified_id` are used to construct identifiers
/// with either the '/' or ':' separator between the collected components.
///
#[derive(Clone, Debug, Default)]
pub struct ResourceBuilder {
    resource: Vec<ResourceIdentifier>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<ArnBuilder> for Arn {
    fn from(v: ArnBuilder) -> Self {
        v.arn
    }
}

impl From<&mut ArnBuilder> for Arn {
    fn from(v: &mut ArnBuilder) -> Self {
        v.arn.clone()
    }
}

impl ArnBuilder {
    /// Construct an Arn for the specified `service`.
    pub fn service(service: Service) -> Self {
        Self::service_id(service.into())
    }

    /// Construct an Arn for the specified `service`.
    pub fn service_id(service: Service) -> Self {
        Self {
            arn: Arn {
                partition: Partition::Aws,
                service,
                region: None,
                account_id: None,
                resource: ResourceIdentifier::default(),
            },
        }
    }

    /// Set a specific `partition` for this Arn.
    pub fn in_partition<P: Into<Partition>>(&mut self, partition: P) -> &mut Self {
        self.arn.partition = partition.into();
        self
    }

    /// Set a specific `partition` for this Arn.
    pub fn in_default_partition(&mut self) -> &mut Self {
        self.arn.partition = Partition::default();
        self
    }

    /// Set a specific `region` for this Arn.
    pub fn in_region<R: Into<Region>>(&mut self, region: R) -> &mut Self {
        self.arn.region = region.into();
        self
    }

    /// Set a specific `region` for this Arn.
    pub fn and_region(&mut self, region: Region) -> &mut Self {
        self.in_region_id(region.into())
    }

    /// Set a specific `region` for this Arn.
    pub fn and_region_id(&mut self, region: Identifier) -> &mut Self {
        self.in_region_id(region)
    }

    /// Set `region` to a wildcard for this Arn.
    pub fn in_any_region(&mut self) -> &mut Self {
        self.in_region_id(Identifier::default())
    }

    /// Set a specific `account` for this Arn.
    pub fn in_account(&mut self, account: AccountIdentifier) -> &mut Self {
        self.arn.account_id = Some(account);
        self
    }

    /// Set a specific `account` for this Arn.
    pub fn and_account(&mut self, account: AccountIdentifier) -> &mut Self {
        self.in_account(account)
    }

    /// Set a specific `account` for this Arn.
    pub fn owned_by(&mut self, account: AccountIdentifier) -> &mut Self {
        self.in_account(account)
    }

    /// Set `account` to a wildcard for this Arn.
    pub fn in_any_account(&mut self) -> &mut Self {
        self.in_account(AccountIdentifier::default())
    }

    /// Set a specific `resource` for this Arn.
    pub fn resource(&mut self, resource: ResourceIdentifier) -> &mut Self {
        self.arn.resource = resource;
        self
    }

    /// Set a specific `resource` for this Arn.
    pub fn is(&mut self, resource: ResourceIdentifier) -> &mut Self {
        self.resource(resource)
    }

    /// Set a specific `resource` for this Arn.
    pub fn a(&mut self, resource: ResourceIdentifier) -> &mut Self {
        self.resource(resource)
    }

    /// Set `resource` to a wildcard for this Arn.
    pub fn any_resource(&mut self) -> &mut Self {
        self.arn.resource = ResourceIdentifier::any();
        self
    }

    /// Set `resource` to a wildcard for this Arn.
    pub fn for_any_resource(&mut self) -> &mut Self {
        self.any_resource()
    }
}

// ------------------------------------------------------------------------------------------------

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

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod cognito;

pub mod iam;

pub mod lambda;

pub mod s3;
