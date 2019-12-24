use crate::check::type_checker::*;
use crate::syntax::ast::*;

#[test]
fn test_example() {
    let (checker, _) = TypeChecker::from_check(vec![
        Bind::Type(
            Ident("ℕ".to_string()),
            Box::new(Type::Sum(vec![
                Instance(
                    Ident("0".to_string()),
                    Box::new(Type::Var(Ident("ℕ".to_string()))),
                ),
                Instance(
                    Ident("1+".to_string()),
                    Box::new(Type::Map(
                        Box::new(Type::Var(Ident("ℕ".to_string()))),
                        Box::new(Type::Var(Ident("ℕ".to_string()))),
                    )),
                ),
            ])),
        ),
        Bind::Expr(
            Ident("+".to_string()),
            Box::new(Expr::Lambda(vec![Pattern {
                param: Box::new(Expr::Typed(
                    Box::new(Expr::Var(Ident("x".to_string()))),
                    Box::new(Type::Var(Ident("ℕ".to_string()))),
                )),
                expr: Box::new(Expr::Lambda(vec![
                    Pattern {
                        param: Box::new(Expr::Apply(
                            Box::new(Expr::Var(Ident("1+".to_string()))),
                            Box::new(Expr::Var(Ident("y".to_string()))),
                        )),
                        expr: Box::new(Expr::Apply(
                            Box::new(Expr::Var(Ident("1+".to_string()))),
                            Box::new(Expr::Apply(
                                Box::new(Expr::Apply(
                                    Box::new(Expr::Var(Ident("+".to_string()))),
                                    Box::new(Expr::Var(Ident("x".to_string()))),
                                )),
                                Box::new(Expr::Var(Ident("y".to_string()))),
                            )),
                        )),
                    },
                    Pattern {
                        param: Box::new(Expr::Var(Ident("0".to_string()))),
                        expr: Box::new(Expr::Var(Ident("x".to_string()))),
                    },
                ])),
            }])),
        ),
        Bind::Type(
            Ident("ℕ𝓁".to_string()),
            Box::new(Type::Sum(vec![
                Instance(
                    Ident("∅".to_string()),
                    Box::new(Type::Var(Ident("ℕ𝓁".to_string()))),
                ),
                Instance(
                    Ident("++".to_string()),
                    Box::new(Type::Map(
                        Box::new(Type::Var(Ident("ℕ".to_string()))),
                        Box::new(Type::Map(
                            Box::new(Type::Var(Ident("ℕ𝓁".to_string()))),
                            Box::new(Type::Var(Ident("ℕ𝓁".to_string()))),
                        )),
                    )),
                ),
            ])),
        ),
        Bind::Expr(
            Ident("take".to_string()),
            Box::new(Expr::Lambda(vec![
                Pattern {
                    param: Box::new(Expr::Apply(
                        Box::new(Expr::Var(Ident("1+".to_string()))),
                        Box::new(Expr::Var(Ident("n".to_string()))),
                    )),
                    expr: Box::new(Expr::Lambda(vec![
                        Pattern {
                            param: Box::new(Expr::Var(Ident("∅".to_string()))),
                            expr: Box::new(Expr::Var(Ident("∅".to_string()))),
                        },
                        Pattern {
                            param: Box::new(Expr::Apply(
                                Box::new(Expr::Apply(
                                    Box::new(Expr::Var(Ident("++".to_string()))),
                                    Box::new(Expr::Var(Ident("x".to_string()))),
                                )),
                                Box::new(Expr::Var(Ident("xs".to_string()))),
                            )),
                            expr: Box::new(Expr::Apply(
                                Box::new(Expr::Apply(
                                    Box::new(Expr::Var(Ident("++".to_string()))),
                                    Box::new(Expr::Var(Ident("x".to_string()))),
                                )),
                                Box::new(Expr::Apply(
                                    Box::new(Expr::Apply(
                                        Box::new(Expr::Var(Ident("take".to_string()))),
                                        Box::new(Expr::Var(Ident("n".to_string()))),
                                    )),
                                    Box::new(Expr::Var(Ident("xs".to_string()))),
                                )),
                            )),
                        },
                    ])),
                },
                Pattern {
                    param: Box::new(Expr::Var(Ident("0".to_string()))),
                    expr: Box::new(Expr::Lambda(vec![Pattern {
                        param: Box::new(Expr::Var(Ident("x".to_string()))),
                        expr: Box::new(Expr::Var(Ident("∅".to_string()))),
                    }])),
                },
            ])),
        ),
    ]);

    assert_eq!(
        checker.symbols,
        vec![
            Symbol {
                id: Ident("0".to_string()),
                optional_type: Some(Type::Var(Ident("ℕ".to_string())))
            },
            Symbol {
                id: Ident("1+".to_string()),
                optional_type: Some(Type::Map(
                    Box::new(Type::Var(Ident("ℕ".to_string()))),
                    Box::new(Type::Var(Ident("ℕ".to_string())))
                ))
            },
            Symbol {
                id: Ident("+".to_string()),
                optional_type: Some(Type::Map(
                    Box::new(Type::Var(Ident("ℕ".to_string()))),
                    Box::new(Type::Map(
                        Box::new(Type::Var(Ident("ℕ".to_string()))),
                        Box::new(Type::Var(Ident("ℕ".to_string())))
                    ))
                ))
            },
            Symbol {
                id: Ident("∅".to_string()),
                optional_type: Some(Type::Var(Ident("ℕ𝓁".to_string())))
            },
            Symbol {
                id: Ident("++".to_string()),
                optional_type: Some(Type::Map(
                    Box::new(Type::Var(Ident("ℕ".to_string()))),
                    Box::new(Type::Map(
                        Box::new(Type::Var(Ident("ℕ𝓁".to_string()))),
                        Box::new(Type::Var(Ident("ℕ𝓁".to_string())))
                    ))
                ))
            },
            Symbol {
                id: Ident("take".to_string()),
                optional_type: Some(Type::Map(
                    Box::new(Type::Var(Ident("ℕ".to_string()))),
                    Box::new(Type::Map(
                        Box::new(Type::Var(Ident("ℕ𝓁".to_string()))),
                        Box::new(Type::Var(Ident("ℕ𝓁".to_string())))
                    ))
                ))
            }
        ]
    );
}
