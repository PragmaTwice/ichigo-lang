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

fn parse_main(pair: Pair<Rule>) -> Main {
    match pair.as_rule() {
        Rule::main => pair
            .into_inner()
            .map(|inner_pair| parse_bind(inner_pair))
            .collect(),
        
        _ => unreachable!()
    }
}

fn parse_ident(pair: Pair<Rule>) -> Ident {
    match pair.as_rule() {
        Rule::ident => Ident {
            name : pair.as_str().to_owned()
        },

        _ => unreachable!()
    }
}

fn parse_bind(pair: Pair<Rule>) -> Bind {
    match pair.as_rule() {
        Rule::bind => {
            let mut inner = pair.into_inner();
            let ident = parse_ident(inner.next().unwrap());
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

    match pair.as_rule() {
        Rule::type_ => {   
            let inner = pair.into_inner().peek().unwrap();
            match inner.as_rule() {
                Rule::sum => parse_sum(inner),
                Rule::map => parse_map(inner),
                Rule::tatom => parse_type(inner),

                _ => unreachable!()
            }
        },
        Rule::tatom => {   
            let inner = pair.into_inner().peek().unwrap();
            match inner.as_rule() {
                Rule::ident => Type::Var(parse_ident(inner)),
                Rule::type_ => parse_type(inner),

                _ => unreachable!()
            }
        },

        _ => unreachable!()
    }
}

fn parse_map(pair: Pair<Rule>) -> Type {
    match pair.as_rule() {
        Rule::map => {
            let mut inner = pair.into_inner();
            let first = inner.next().unwrap();
            
            inner.fold(parse_type(first), |acc, x| Type::Map(Box::new(acc), Box::new(parse_type(x))))
        },

        _ => unreachable!()
    }
}

fn parse_sum(pair: Pair<Rule>) -> Type {
    match pair.as_rule() {
        Rule::sum => {
            let mut instances = Vec::new();
            pair.into_inner().peek().unwrap().into_inner().for_each(|p| {
                match p.as_rule() {
                    Rule::instance => {
                        let mut inner = p.into_inner();
                        let ident = inner.next().unwrap();
                        let type_ = inner.next().unwrap();

                        instances.push(Instance{
                            ins: parse_ident(ident),
                            type_: Box::new(parse_type(type_))
                        });
                    },

                    _ => unreachable!()
                };
            });
            Type::Sum(instances)
        },

        _ => unreachable!()
    }
}
