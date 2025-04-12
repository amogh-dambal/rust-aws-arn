#[macro_export]
macro_rules! load_test_data {
    ($path: literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/", $path))
    };
}
