use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;
use std::fs;

use super::ast::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct IchigoParser;

pub fn parse_file(filename: &str) {
    let contents = fs::read_to_string(filename)
        .expect("something went wrong while reading the file");

    parse_str(contents.as_str());
}

pub fn parse_str(input: &str) {
    let pair = IchigoParser::parse(Rule::main, input)
        .unwrap_or_else(|e| panic!("{}", e))
        .next()
        .unwrap();

    parse_main(pair);
}

fn parse_main(pair: Pair<Rule>) -> Vec<Bind> {
    match pair.as_rule() {
        Rule::main => pair
            .into_inner()
            .map(|inner_pair| parse_bind(inner_pair))
            .collect(),
        _ => unreachable!()
    }
}

fn parse_bind(pair: Pair<Rule>) -> Bind {
    match pair.as_rule() {
        Rule::bind => {
            let mut inner = pair.into_inner();
            let ident = Ident { 
                name : inner.next().unwrap().as_str().to_string()
            };
            let binded = inner.next().unwrap();

            match binded.as_rule() {
                Rule::expr => Bind::Expr(ident, Box::new(parse_expr(binded))),
                Rule::type_ => Bind::Type(ident, Box::new(parse_type(binded))),

                _ => unreachable!()
            }
        },
        _ => unreachable!()
    }
}

fn parse_expr(pair: Pair<Rule>) -> Expr {
    unreachable!()
}

fn parse_type(pair: Pair<Rule>) -> Type {
    unreachable!()
}