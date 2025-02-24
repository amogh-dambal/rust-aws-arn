use aws_arn::{AccountId, AccountIdentifier, IdentifierLike, ResourceName};
use proptest::prelude::*;
use rstest::rstest;
use std::str::FromStr;

#[rstest]
#[case::iam_managed_policies(
    "arn:aws:iam::aws:policy/ReadOnlyAccess",
    Some(AccountIdentifier::Service(aws_arn::Identifier::new_unchecked("aws")))
)]
#[case::bedrock("arn:aws:states:::aws-sdk:bedrock:[apiAction]", None)]
fn test_account_identifier_service(
    #[case] arn: &str,
    #[case] expected_account_identifier: Option<AccountIdentifier>,
) {
    let resource_name = arn.parse::<ResourceName>();
    assert!(
        resource_name.is_ok(),
        "expected ARN: {:?} to parse successfully, instead: {:?}",
        arn,
        resource_name
    );

    let resource_name = resource_name.unwrap();
    assert_eq!(
        expected_account_identifier, resource_name.account_id,
        "expected {:?} but got {:?}",
        expected_account_identifier, resource_name.account_id
    );
}

#[rstest]
#[case::plain("012345678912", false, false, true)]
#[case::wildcard("*", true, true, false)]
fn test_account_id_api(
    #[case] account_id: &str,
    #[case] is_any: bool,
    #[case] has_wildcards: bool,
    #[case] is_plain: bool,
) {
    let account = AccountId::from_str(account_id);
    assert!(account.is_ok());
    let account = account.unwrap();
    assert!(!account.is_empty());
    assert_eq!(is_any, account.is_any());
    assert_eq!(has_wildcards, account.has_wildcards());
    assert_eq!(is_plain, account.is_plain());
}

#[rstest]
#[case::standard("012345678912")]
#[case::wildcard("*")]
fn test_account_id_valid(#[case] val: &str) {
    assert!(AccountId::is_valid(val))
}

#[rstest]
#[case::empty("")]
#[case::too_few_digits("123456789")]
#[case::whitespace(" ")]
#[case::tab("\t")]
#[case::carriage_return("\r")]
#[case::newline("\n")]
#[case::char("a")]
#[case::some_str("a a")]
#[case::colon(":")]
#[case::slash("/")]
fn test_account_id_is_not_valid(#[case] val: &str) {
    assert!(!AccountId::is_valid(val));
}

proptest! {
    #[test]
    fn proptest_account_id_char_doesnt_crash(s in "\\PC") {
        let _ = AccountId::from_str(&s);
    }

   #[test]
   fn proptest_account_id_valid_values(s in r"[0-9]{12}") {
       println!("valid_values {:?}", s);
       assert!(AccountId::from_str(&s).is_ok());
   }
}
