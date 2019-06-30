mod parser;
mod ast;

fn main() {
    parser::parse_file("example/hello.ichigo");
}
