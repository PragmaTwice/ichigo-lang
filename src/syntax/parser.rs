use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use super::ast::*;

#[derive(Parser)]
#[grammar = "syntax/grammar.pest"]
struct IchigoParser;

pub type ParseResult<Node> = Result<Node, String>;

pub fn parse_str(input: &str) -> ParseResult<Main> {
    match IchigoParser::parse(Rule::main, input) {
        Ok(o) => Ok(parse_main(o.clone().next().unwrap())),
        Err(e) => Err(e.to_string()),
    }
}

fn parse_main(pair: Pair<Rule>) -> Main {
    match pair.as_rule() {
        Rule::main => pair
            .into_inner()
            .filter_map(|inner_pair| parse_bind(inner_pair))
            .collect(),

        _ => unreachable!(),
    }
}

fn parse_ident(pair: Pair<Rule>) -> Ident {
    match pair.as_rule() {
        Rule::ident => Ident(pair.as_str().to_owned()),

        _ => unreachable!(),
    }
}

fn parse_bind(pair: Pair<Rule>) -> Option<Bind> {
    match pair.as_rule() {
        Rule::bind => {
            let mut inner = pair.into_inner();
            let ident = parse_ident(inner.next().unwrap());
            let binded = inner.next().unwrap();

            Some(match binded.as_rule() {
                Rule::expr => Bind::Expr(ident, Box::new(parse_expr(binded))),
                Rule::sum => Bind::Type(ident, Box::new(parse_sum(binded))),

                _ => unreachable!(),
            })
        }

        Rule::EOI => None,

        _ => unreachable!(),
    }
}

fn parse_expr(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::expr => {
            let inner = pair.into_inner().peek().unwrap();
            match inner.as_rule() {
                Rule::lambda => parse_lambda(inner),
                Rule::apply => parse_apply(inner),
                Rule::typed => parse_typed(inner),
                Rule::atom => parse_expr(inner),

                _ => unreachable!(),
            }
        }
        Rule::atom => {
            let inner = pair.into_inner().peek().unwrap();
            match inner.as_rule() {
                Rule::expr => parse_expr(inner),
                Rule::ident => Expr::Var(parse_ident(inner)),

                _ => unreachable!(),
            }
        }

        _ => unreachable!(),
    }
}

fn parse_lambda(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::lambda => {
            let inner = pair.into_inner().peek().unwrap();
            match inner.as_rule() {
                Rule::patterns => {
                    let mut patterns = Vec::new();
                    for pattern in inner.into_inner() {
                        match pattern.as_rule() {
                            Rule::pattern => {
                                let mut innererer = pattern.into_inner();
                                let param = innererer.next().unwrap();
                                let expr = innererer.next().unwrap();

                                patterns.push(Pattern {
                                    param: Box::new(parse_expr(param)),
                                    expr: Box::new(parse_expr(expr)),
                                });
                            }

                            _ => unreachable!(),
                        }
                    }

                    Expr::Lambda(patterns)
                }

                _ => unreachable!(),
            }
        }

        _ => unreachable!(),
    }
}

fn parse_apply(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::apply => {
            let mut inner = pair.into_inner();
            let first = inner.next().unwrap();

            inner.fold(parse_expr(first), |acc, x| {
                Expr::Apply(Box::new(acc), Box::new(parse_expr(x)))
            })
        }

        _ => unreachable!(),
    }
}

fn parse_typed(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::typed => {
            let mut inner = pair.into_inner();
            let expr = inner.next().unwrap();
            let type_ = inner.next().unwrap();

            Expr::Typed(Box::new(parse_expr(expr)), Box::new(parse_type(type_)))
        }

        _ => unreachable!(),
    }
}

fn parse_type(pair: Pair<Rule>) -> Type {
    match pair.as_rule() {
        Rule::type_ => {
            let inner = pair.into_inner().peek().unwrap();
            match inner.as_rule() {
                Rule::map => parse_map(inner),
                Rule::type_atom => parse_type(inner),

                _ => unreachable!(),
            }
        }
        Rule::type_atom => {
            let inner = pair.into_inner().peek().unwrap();
            match inner.as_rule() {
                Rule::ident => Type::Var(parse_ident(inner)),
                Rule::type_ => parse_type(inner),

                _ => unreachable!(),
            }
        }

        _ => unreachable!(),
    }
}

fn parse_map(pair: Pair<Rule>) -> Type {
    match pair.as_rule() {
        Rule::map => {
            let mut inner = pair.into_inner();
            let first = inner.next().unwrap();
            let second = inner.next();
            match second {
                Some(x) => Type::Map(Box::new(parse_type(first)), Box::new(parse_map(x))),
                None => parse_type(first),
            }
        }

        _ => unreachable!(),
    }
}

fn parse_sum(pair: Pair<Rule>) -> Type {
    match pair.as_rule() {
        Rule::sum => {
            let mut instances = Vec::new();
            pair.into_inner()
                .peek()
                .unwrap()
                .into_inner()
                .for_each(|p| {
                    match p.as_rule() {
                        Rule::instance => {
                            let mut inner = p.into_inner();
                            let ident = inner.next().unwrap();
                            let type_ = inner.next().unwrap();

                            instances
                                .push(Instance(parse_ident(ident), Box::new(parse_type(type_))));
                        }

                        _ => unreachable!(),
                    };
                });
            Type::Sum(instances)
        }

        _ => unreachable!(),
    }
}
