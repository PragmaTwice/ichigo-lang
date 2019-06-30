use crate::syntax::parser;

#[test]
fn test_example() {
    parser::parse_file("example/hello.ichigo");
}
