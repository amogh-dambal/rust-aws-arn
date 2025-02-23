use aws_arn::{AccountIdentifier, Identifier, Region, ResourceIdentifier, ResourceName, Service};
use std::str::FromStr;

#[test]
fn test_s3_bucket() {
    let arn: ResourceName = ResourceName::builder()
        .service(Service::S3)
        .resource(ResourceIdentifier::from_str("my-bucket").unwrap())
        .build();
    assert_eq!(arn.to_string(), "arn:aws:s3:::my-bucket");
}

#[test]
fn test_lambda_layer() {
    let arn: ResourceName = ResourceName::builder()
        .service(Service::Lambda)
        .resource(ResourceIdentifier::from_qualified_id(&[
            Identifier::from_str("layer").unwrap(),
            Identifier::from_str("my-layer").unwrap(),
            Identifier::from_str(&3.to_string()).unwrap(),
        ]))
        .in_region(Region::UsEast2)
        .owned_by(AccountIdentifier::from_str("123456789012").unwrap())
        .build();

    assert_eq!(
        arn.to_string(),
        "arn:aws:lambda:us-east-2:123456789012:layer:my-layer:3"
    );
}
