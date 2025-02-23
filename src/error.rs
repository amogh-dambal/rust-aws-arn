//! Errors thrown by this crate.

use std::fmt::Debug;

/// Any error that may arise from handling an ARN using this crate.
/// Errors that may arise parsing an ResourceName with `FromStr::from_str()`.
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum ArnError {
    /// String length must be greater than 8 corresponding to `"arn:::::"`.
    #[error("provided string has length {0}, must be at least 8")]
    TooShort(usize),
    /// String length must be under 2048 characters..
    #[error("provided string has length {0}, must be at most 2048")]
    TooLong(usize),
    /// Need at least 6 components.
    #[error("provided string has {0} components, must have at least 6")]
    TooFewComponents(usize),
    /// Invalid `Identifier` string value.
    #[error("{0} is not a valid identifier")]
    InvalidIdentifier(String),
    /// Missing the 'arn' prefix string.
    #[error("provided string is missing the 'arn' prefix")]
    MissingPrefix,
    /// Missing the partition component.
    #[error("provided string is missing the partition component")]
    MissingPartition,
    /// The partition component provided is not valid.
    #[error("{0} is not a valid partition")]
    InvalidPartition(String),
    /// Missing the service component.
    #[error("provided string is missing the partition component")]
    MissingService,
    /// The service component provided is not valid.
    #[error("{0} is not a valid service")]
    InvalidService(String),
    /// Missing the region component.
    #[error("provided string is missing the region component")]
    MissingRegion,
    /// The partition region provided is not valid.
    #[error("{0} is not a valid region")]
    InvalidRegion(String),
    /// The particular resource type does not allow region wildcards.
    #[error("resource type {0} does not allow region wildcards")]
    RegionWildcardNotAllowed(String),
    /// Missing the account id component.
    #[error("provided string is missing the account ID component")]
    MissingAccountId,
    /// The partition account id provided is not valid.
    #[error("{0} is not a valid account ID: must match ^[0-9]{{12}}$")]
    InvalidAccountId(String),
    /// The particular resource type does not allow account wildcards.
    #[error("resource type {0} does not allow account wildcards")]
    AccountIdWildcardNotAllowed(String),
    /// Missing the resource component.
    #[error("provided string is missing the resource component")]
    MissingResource,
    /// The partition resource provided is not valid, the name of the particular component
    /// in error is included.
    #[error("{0} is not a valid resource")]
    InvalidResource(String),
    /// The particular resource type does not allow resource wildcards.
    #[error("resource type {0} does not allow resource wildcards")]
    ResourceWildcardNotAllowed(String),
}

pub type ArnResult<T> = Result<T, ArnError>;
