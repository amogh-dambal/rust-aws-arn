//! Higher-level utilities to build ARNs for Amazon S3 (Simple Storage Service).
//!
//! For more information, check out the [AWS documentation](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazons3.html#amazons3-resources-for-iam-policies)

use crate::types::Partition;
use crate::types::Service::S3;
use crate::{AccountIdentifier, Identifier, Region, ResourceIdentifier, ResourceName};

///
/// `arn:${Partition}:s3:::${BucketName}`
///
pub fn bucket_in(partition: Partition, bucket_name: Identifier) -> ResourceName {
    ResourceName::builder()
        .service(S3)
        .in_partition(partition)
        .is(bucket_name)
        .build()
}

///
/// `arn:aws:s3:::${BucketName}`
///
pub fn bucket(bucket_name: Identifier) -> ResourceName {
    bucket_in(Partition::default().into(), bucket_name)
}

///
/// `arn:${Partition}:s3:::${BucketName}/${ObjectName}`
///
pub fn object_in(
    partition: Partition,
    bucket_name: Identifier,
    object_name: Identifier,
) -> ResourceName {
    ResourceName::builder()
        .service(S3)
        .in_partition(partition)
        .is(ResourceIdentifier::from_id_path(&[
            bucket_name,
            object_name,
        ]))
        .build()
}

///
/// `arn:aws:s3:::${BucketName}/${ObjectName}`
///
pub fn object(bucket_name: Identifier, object_name: Identifier) -> ResourceName {
    object_in(Partition::default().into(), bucket_name, object_name)
}

///
/// `arn:aws:s3:::${BucketName}/${ObjectName}`
///
/// This function will panic if `bucket` is not an ResourceName for an S3 bucket.
///
pub fn object_from(bucket: &ResourceName, object_name: Identifier) -> ResourceName {
    if bucket.service != S3.into() {
        panic!(
            "You can't make an S3 object from a {} ResourceName.",
            bucket.service
        );
    }
    ResourceName {
        resource: ResourceIdentifier::from_path(&[bucket.resource.clone(), object_name.into()]),
        ..bucket.clone()
    }
}

///
/// `arn:${Partition}:s3:${Region}:${Account}:job/${JobId}`
///
pub fn job_in(
    partition: Partition,
    region: Region,
    account: AccountIdentifier,
    job_id: Identifier,
) -> ResourceName {
    ResourceName::builder()
        .service(S3)
        .in_partition(partition)
        .in_region(region)
        .owned_by(account)
        .is(job_id)
        .build()
}

///
/// `arn:aws:s3:${Region}:${Account}:job/${JobId}`
///
pub fn job(region: Region, account: AccountIdentifier, job_id: Identifier) -> ResourceName {
    job_in(Partition::default().into(), region, account, job_id)
}
