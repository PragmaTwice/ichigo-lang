use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;
use std::fs;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct IchigoParser;

pub fn parse_file(filename: &str) {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    parse_str(contents.as_str());
}

pub fn parse_str(input: &str) {
    let pair = IchigoParser::parse(Rule::main, input)
        .unwrap_or_else(|e| panic!("{}", e))
        .next()
        .unwrap();

    parse_value(pair);
}

fn parse_value(pair: Pair<Rule>) {
    match pair.as_rule() {
        Rule::main => pair.into_inner().map(|inner_pair| parse_value(inner_pair)).collect(),
        Rule::bind => println!("bind: {}", pair.as_str()),
        _ => ()
    }
}
