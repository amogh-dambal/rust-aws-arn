use std::str::FromStr;

use aws_arn::{
    AccountId, IdentifierLike, Partition, Region, ResourceIdentifier, ResourceName, Service,
};

fn parse_and_compare(test_arn: &str, expected: ResourceName) {
    let result = ResourceName::from_str(test_arn);
    assert!(result.is_ok(), "expected OK but received {:?}", result);
    let arn = result.unwrap();
    assert_eq!(arn, expected);
}

#[test]
fn test_valid_arn_to_string() {
    let arn = ResourceName {
        partition: Partition::Aws,
        service: Service::S3,
        region: None,
        account_id: None,
        resource: ResourceIdentifier::new_unchecked("mythings/athing"),
    };
    assert_eq!(arn.to_string(), "arn:aws:s3:::mythings/athing");
}

#[test]
fn test_valid_arn_to_string_wild() {
    let arn = ResourceName {
        partition: Partition::Aws,
        service: Service::S3,
        region: None,
        account_id: None,
        resource: ResourceIdentifier::new_unchecked("mythings/*"),
    };
    assert_eq!(arn.to_string(), "arn:aws:s3:::mythings/*");
}

#[test]
fn test_valid_arn_to_string_wild_more() {
    let arn = ResourceName {
        partition: Partition::Aws,
        service: Service::S3,
        region: None,
        account_id: None,
        resource: ResourceIdentifier::new_unchecked("mything?/?thing"),
    };
    assert_eq!(arn.to_string(), "arn:aws:s3:::mything?/?thing");
}

#[test]
fn test_arn_from_valid_str() {
    parse_and_compare(
        "arn:aws:s3:us-east-1:123456789012:job/23476",
        ResourceName {
            partition: Partition::Aws,
            service: Service::S3,
            region: Some(Region::UsEast1),
            account_id: Some(AccountId::new_unchecked("123456789012").into()),
            resource: ResourceIdentifier::new_unchecked("job/23476"),
        },
    );
}

#[test]
fn test_github_issues_2() {
    let result = ResourceName::from_str(
        "arn:aws:cloudwatch:us-west-2:123456789012:alarm:Production:LB:High4xx",
    );
    assert!(result.is_ok());
    let arn = result.unwrap();
    assert_eq!(arn.partition, Partition::Aws);
    assert_eq!(arn.service, Service::CloudWatch);
    assert_eq!(arn.region, Some(Region::UsWest2));
    assert_eq!(
        arn.account_id,
        Some(AccountId::new_unchecked("123456789012").into())
    );
    assert_eq!(
        arn.resource,
        ResourceIdentifier::new_unchecked("alarm:Production:LB:High4xx")
    );
    assert!(arn.resource.contains_qualified());
}
