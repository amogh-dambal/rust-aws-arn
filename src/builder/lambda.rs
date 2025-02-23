//! High-level utilities to provide ARNs for AWS Lambda.
//!
//! For more information, check out the [AWS documentation](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_awslambda.html#awslambda-resources-for-iam-policies)

use crate::{
    AccountIdentifier, Arn, Identifier, IdentifierLike, Partition, Region, ResourceIdentifier,
    Service::Lambda,
};

///
/// `arn:${Partition}:lambda:${Region}:${Account}:function:${FunctionName}`
///
pub fn function(
    partition: Partition,
    region: Region,
    account: AccountIdentifier,
    function_name: Identifier,
) -> Arn {
    Arn::builder()
        .service(Lambda)
        .in_partition(partition)
        .in_region(region)
        .owned_by(account)
        .is(ResourceIdentifier::from_qualified_id(&[
            Identifier::new_unchecked("function"),
            function_name,
        ]))
        .build()
}

///
/// `arn:${Partition}:lambda:${Region}:${Account}:layer:${LayerName}`
///
pub fn layer(
    partition: Partition,
    region: Region,
    account: AccountIdentifier,
    layer_name: Identifier,
) -> Arn {
    Arn::builder()
        .service(Lambda)
        .in_partition(partition)
        .in_region(region)
        .owned_by(account)
        .is(ResourceIdentifier::from_qualified_id(&[
            Identifier::new_unchecked("layer"),
            layer_name,
        ]))
        .build()
}

///
/// `arn:${Partition}:lambda:${Region}:${Account}:layer:${LayerName}:${LayerVersion}`
///
pub fn layer_version(
    partition: Partition,
    region: Region,
    account: AccountIdentifier,
    layer_name: Identifier,
    layer_version: i32,
) -> Arn {
    Arn::builder()
        .service(Lambda)
        .in_partition(partition)
        .in_region(region)
        .owned_by(account)
        .is(ResourceIdentifier::from_qualified_id(&[
            Identifier::new_unchecked("layer"),
            layer_name,
            Identifier::new_unchecked(&layer_version.to_string()),
        ]))
        .build()
}

///
/// `arn:${Partition}:lambda:${Region}:${Account}:event-source-mapping:${UUID}`
///
pub fn event_source_mapping(
    partition: Partition,
    region: Region,
    account: AccountIdentifier,
    mapping_uuid: Identifier,
) -> Arn {
    Arn::builder()
        .service(Lambda)
        .in_partition(partition)
        .in_region(region)
        .owned_by(account)
        .is(ResourceIdentifier::from_qualified_id(&[
            Identifier::new_unchecked("event-source-mapping"),
            mapping_uuid,
        ]))
        .build()
}
