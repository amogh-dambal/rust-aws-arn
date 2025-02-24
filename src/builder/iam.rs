//! Higher-level utilities to provide ARNs for AWS IAM (Identity and Access Managment).
//!
//! For more information, see the [AWS documentation](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_identityandaccessmanagement.html#identityandaccessmanagement-resources-for-iam-policies).
//! See [here](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns) for
//! documentation on the AWS ARN for root AWS accounts.

use crate::{
    AccountId, Identifier, IdentifierLike, Partition, ResourceIdentifier, ResourceName,
    Service::IdentityAccessManagement,
};

///
/// `arn:aws:iam::123456789012:root`
///
pub fn root(account: AccountId) -> ResourceName {
    ResourceName::builder()
        .service(IdentityAccessManagement)
        .owned_by(account)
        .is(ResourceIdentifier::new_unchecked("root"))
        .build()
}

///
/// `arn:${Partition}:iam::${Account}:user/${UserNameWithPath}`
///
pub fn user(partition: Partition, account: AccountId, user_name: Identifier) -> ResourceName {
    ResourceName::builder()
        .service(IdentityAccessManagement)
        .in_partition(partition)
        .owned_by(account)
        .is(ResourceIdentifier::from_id_path(&[
            Identifier::new_unchecked("user"),
            user_name,
        ]))
        .build()
}

///
/// `arn:${Partition}:iam::${Account}:role/${RoleNameWithPath}`
///
pub fn role(partition: Partition, account: AccountId, role_name: Identifier) -> ResourceName {
    ResourceName::builder()
        .service(IdentityAccessManagement)
        .in_partition(partition)
        .owned_by(account)
        .is(ResourceIdentifier::from_id_path(&[
            Identifier::new_unchecked("role"),
            role_name,
        ]))
        .build()
}

///
/// `arn:${Partition}:iam::${Account}:group/${GroupNameWithPath}`
///
pub fn group(partition: Partition, account: AccountId, group_name: Identifier) -> ResourceName {
    ResourceName::builder()
        .service(IdentityAccessManagement)
        .in_partition(partition)
        .owned_by(account)
        .is(ResourceIdentifier::from_id_path(&[
            Identifier::new_unchecked("group"),
            group_name,
        ]))
        .build()
}

///
/// `arn:${Partition}:iam::${Account}:policy/${PolicyNameWithPath}`
///
pub fn policy(partition: Partition, account: AccountId, policy_name: Identifier) -> ResourceName {
    ResourceName::builder()
        .service(IdentityAccessManagement)
        .in_partition(partition)
        .owned_by(account)
        .is(ResourceIdentifier::from_id_path(&[
            Identifier::new_unchecked("policy"),
            policy_name,
        ]))
        .build()
}
