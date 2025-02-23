//! Higher-level utilites for Amazon Cognito Identity.
//!
//! For more information, check out the [AWS documentation](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazoncognitoidentity.html#amazoncognitoidentity-resources-for-iam-policies).

use crate::{
    AccountIdentifier, Arn, Identifier, IdentifierLike, Partition, Region, ResourceIdentifier,
    Service::CognitoIdentity,
};

///
/// `arn:${Partition}:cognito-identity:${Region}:${Account}:identitypool/${IdentityPoolId}`
///
pub fn identity_pool(
    partition: Partition,
    region: Region,
    account: AccountIdentifier,
    identity_pool_id: Identifier,
) -> Arn {
    Arn::builder()
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
