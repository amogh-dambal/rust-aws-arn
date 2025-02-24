//! Higher-level utilites for Amazon Cognito Identity.
//!
//! For more information, check out the [AWS documentation](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazoncognitoidentity.html#amazoncognitoidentity-resources-for-iam-policies).

use crate::{
    AccountId, Identifier, IdentifierLike, Partition, Region, ResourceIdentifier, ResourceName,
    Service::CognitoIdentity,
};

///
/// `arn:${Partition}:cognito-identity:${Region}:${Account}:identitypool/${IdentityPoolId}`
///
pub fn identity_pool(
    partition: Partition,
    region: Region,
    account: AccountId,
    identity_pool_id: Identifier,
) -> ResourceName {
    ResourceName::builder()
        .service(CognitoIdentity)
        .in_partition(partition)
        .in_region(region)
        .owned_by(account)
        .is(ResourceIdentifier::from_id_path(&[
            Identifier::new_unchecked("identitypool"),
            identity_pool_id,
        ]))
        .build()
}
