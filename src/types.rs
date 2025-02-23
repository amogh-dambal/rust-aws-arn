//! Define primitive types for AWS ARN components
mod partition;
mod region;
mod service;

pub use partition::Partition;
pub use region::Region;
pub use service::Service;
