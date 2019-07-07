use super::syntax::parser;
use super::check::type_checker;
use std::env;

pub fn parse_option() {
    let args: Vec<_> = env::args().collect();

    let mut arg_iter = args.iter();
    
    let _program_path = arg_iter.next();

    match arg_iter.next() {
        Some(x) => {
            let ast = parser::parse_file(x);
            match ast {
                Ok(o) => {
                    println!("untyped   : {:?}", o);
                    println!();
                    let (_, typed_ast) = type_checker::TypeChecker::check(o);
                    match typed_ast {
                        Ok(o) => println!("typed     : {:?}", o),
                        Err(e) => println!("type error: {}", e)
                    }
                },
                Err(e) => println!("parse error: {}", e)
            };
            
        },
        None => println!("no file path.")
    }
}
