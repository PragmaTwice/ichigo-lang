use crate::preprocess::unicode_coverter;

use std::fs;

#[test]
fn test_example() {
    assert_eq!(
        unicode_coverter::convert(
            fs::read_to_string("example/hello.ascii.ichigo")
                .unwrap()
                .as_str()
        )
        .unwrap(),
        fs::read_to_string("example/hello.ichigo").unwrap().as_str()
    )
}
