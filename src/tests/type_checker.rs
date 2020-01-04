use crate::check::type_checker::*;
use crate::syntax::ast::*;
use crate::syntax::parser;

use std::fs;

#[test]
fn test_example() {
    let (checker, typed_ast) = TypeChecker::from_check(
        parser::parse_str(fs::read_to_string("example/hello.ichigo").unwrap().as_str()).unwrap(),
    );

    assert_eq!(
        typed_ast.unwrap(),
        parser::parse_str(
            fs::read_to_string("example/hello.typed.ichigo")
                .unwrap()
                .as_str()
        )
        .unwrap()
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
        hashset! {Ident("ℕ".to_string()), Ident("ℕ𝓁".to_string())}
    );
}
