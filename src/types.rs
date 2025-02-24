//! Define primitive types for AWS ARN components
mod identifier;
mod partition;
mod region;
mod service;

pub use identifier::{AccountId, Identifier, IdentifierLike, ResourceIdentifier};
pub(crate) use identifier::{ARN_PREFIX, PART_SEPARATOR, REQUIRED_COMPONENT_COUNT};
pub use partition::Partition;
pub use region::Region;
pub use service::Service;
