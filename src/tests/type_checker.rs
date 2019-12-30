use crate::check::type_checker::*;
use crate::syntax::ast::*;
use crate::syntax::parser;

#[test]
fn test_example() {
    let (checker, typed_ast) =
        TypeChecker::from_check(parser::parse_file("example/hello.ichigo").unwrap());

    assert_eq!(
        typed_ast.unwrap(),
        parser::parse_file("example/hello.typed.ichigo").unwrap()
    );

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

    assert_eq!(
        checker.types,
        vec![Ident("ℕ".to_string()), Ident("ℕ𝓁".to_string())]
            .into_iter()
            .collect()
    );
}
