use aws_arn::ResourceName;
use std::str::FromStr;

#[macro_use]
mod utils;

const EXAMPLES: &str = load_test_data!("examples.txt");
const EXAMPLES_WITH_SERVICES: &str = load_test_data!("examples-with-services.txt");

#[test]
fn test_examples_from_file() {
    for (line, arn_str) in EXAMPLES.lines().enumerate() {
        if !arn_str.starts_with("#") {
            println!("{:0>4}: {}", line + 1, arn_str);
            let parsed = ResourceName::from_str(arn_str);
            println!("{:#?}", parsed);
            assert!(parsed.is_ok());
        } else {
            println!("{:0>4}: IGNORE {}", line + 1, &arn_str[1..]);
        }
    }

    for (line, arn_str) in EXAMPLES_WITH_SERVICES.lines().enumerate() {
        if !arn_str.starts_with("#") {
            println!("{:0>4}: {}", line + 1, arn_str);
            let parsed = ResourceName::from_str(arn_str);
            println!("{:#?}", parsed);
            assert!(parsed.is_ok());
        } else {
            println!("{:0>4}: IGNORE {}", line + 1, &arn_str[1..]);
        }
    }
}
