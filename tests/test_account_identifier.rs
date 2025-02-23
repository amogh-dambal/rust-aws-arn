use aws_arn::{AccountIdentifier, IdentifierLike};
use proptest::prelude::*;
use rstest::rstest;
use std::str::FromStr;

#[rstest]
#[case::plain("012345678912", false, false, true)]
#[case::wildcard("*", true, true, false)]
fn test_account_identifier_api(
    #[case] account_id: &str,
    #[case] is_any: bool,
    #[case] has_wildcards: bool,
    #[case] is_plain: bool,
) {
    let account = AccountIdentifier::from_str(account_id);
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
fn test_account_identifier_valid(#[case] val: &str) {
    assert!(AccountIdentifier::is_valid(val))
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
fn test_account_identifier_is_not_valid(#[case] val: &str) {
    assert!(!AccountIdentifier::is_valid(val));
}

proptest! {
    #[test]
    fn proptest_account_identifier_char_doesnt_crash(s in "\\PC") {
        let _ = AccountIdentifier::from_str(&s);
    }

   #[test]
   fn proptest_account_identifier_valid_values(s in r"[0-9]{12}") {
       println!("valid_values {:?}", s);
       assert!(AccountIdentifier::from_str(&s).is_ok());
   }
}
