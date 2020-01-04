use crate::syntax::ast::*;
use crate::syntax::parser;

use std::fs;

#[test]
fn test_example() {
    assert_eq!(
        parser::parse_str(fs::read_to_string("example/hello.ichigo").unwrap().as_str()),
        Ok(vec![
            Bind::Type(
                Ident("ℕ".to_string(),),
                Box::new(Type::Sum(vec![
                    Instance(
                        Ident("0".to_string(),),
                        Box::new(Type::Var(Ident("ℕ".to_string(),),)),
                    ),
                    Instance(
                        Ident("1+".to_string(),),
                        Box::new(Type::Map(
                            Box::new(Type::Var(Ident("ℕ".to_string(),),)),
                            Box::new(Type::Var(Ident("ℕ".to_string(),),)),
                        )),
                    ),
                ],)),
            ),
            Bind::Expr(
                Ident("+".to_string(),),
                Box::new(Expr::Lambda(vec![Pattern {
                    param: Box::new(Expr::Typed(
                        Box::new(Expr::Var(Ident("x".to_string(),),)),
                        Box::new(Type::Var(Ident("ℕ".to_string(),)))
                    )),
                    expr: Box::new(Expr::Lambda(vec![
                        Pattern {
                            param: Box::new(Expr::Apply(
                                Box::new(Expr::Var(Ident("1+".to_string(),),)),
                                Box::new(Expr::Var(Ident("y".to_string(),),),),
                            )),
                            expr: Box::new(Expr::Apply(
                                Box::new(Expr::Var(Ident("1+".to_string(),),)),
                                Box::new(Expr::Apply(
                                    Box::new(Expr::Apply(
                                        Box::new(Expr::Var(Ident("+".to_string(),),)),
                                        Box::new(Expr::Var(Ident("x".to_string(),),)),
                                    )),
                                    Box::new(Expr::Var(Ident("y".to_string(),),)),
                                )),
                            )),
                        },
                        Pattern {
                            param: Box::new(Expr::Var(Ident("0".to_string()))),
                            expr: Box::new(Expr::Var(Ident("x".to_string())))
                        }
                    ])),
                },],)),
            ),
            Bind::Type(
                Ident("ℕ𝓁".to_string(),),
                Box::new(Type::Sum(vec![
                    Instance(
                        Ident("∅".to_string(),),
                        Box::new(Type::Var(Ident("ℕ𝓁".to_string(),),)),
                    ),
                    Instance(
                        Ident("++".to_string(),),
                        Box::new(Type::Map(
                            Box::new(Type::Var(Ident("ℕ".to_string(),),)),
                            Box::new(Type::Map(
                                Box::new(Type::Var(Ident("ℕ𝓁".to_string(),),)),
                                Box::new(Type::Var(Ident("ℕ𝓁".to_string(),),)),
                            )),
                        )),
                    ),
                ],)),
            ),
            Bind::Expr(
                Ident("take".to_string(),),
                Box::new(Expr::Lambda(vec![
                    Pattern {
                        param: Box::new(Expr::Apply(
                            Box::new(Expr::Var(Ident("1+".to_string()))),
                            Box::new(Expr::Var(Ident("n".to_string())))
                        )),
                        expr: Box::new(Expr::Lambda(vec![
                            Pattern {
                                param: Box::new(Expr::Var(Ident("∅".to_string()))),
                                expr: Box::new(Expr::Var(Ident("∅".to_string())))
                            },
                            Pattern {
                                param: Box::new(Expr::Apply(
                                    Box::new(Expr::Apply(
                                        Box::new(Expr::Var(Ident("++".to_string()))),
                                        Box::new(Expr::Var(Ident("x".to_string())))
                                    )),
                                    Box::new(Expr::Var(Ident("xs".to_string())))
                                )),
                                expr: Box::new(Expr::Apply(
                                    Box::new(Expr::Apply(
                                        Box::new(Expr::Var(Ident("++".to_string()))),
                                        Box::new(Expr::Var(Ident("x".to_string())))
                                    )),
                                    Box::new(Expr::Apply(
                                        Box::new(Expr::Apply(
                                            Box::new(Expr::Var(Ident("take".to_string()))),
                                            Box::new(Expr::Var(Ident("n".to_string())))
                                        )),
                                        Box::new(Expr::Var(Ident("xs".to_string())))
                                    ))
                                ))
                            },
                        ],))
                    },
                    Pattern {
                        param: Box::new(Expr::Var(Ident("0".to_string(),),),),
                        expr: Box::new(Expr::Lambda(vec![Pattern {
                            param: Box::new(Expr::Var(Ident("x".to_string(),),)),
                            expr: Box::new(Expr::Var(Ident("∅".to_string(),),)),
                        },],)),
                    }
                ],)),
            ),
        ])
    );
}
