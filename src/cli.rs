use super::syntax::parser;
use super::check::type_checker;
use std::env;
use colored::*;

pub fn parse_option() {
    let args: Vec<_> = env::args().collect();

    let mut arg_iter = args.iter();
    
    let _program_path = arg_iter.next();

    match arg_iter.next() {
        Some(x) => {
            let ast = parser::parse_file(x);
            match ast {
                Ok(o) => {
                    println!("{:6} : {:?}\n", "untyped".yellow(), o);

                    let (checker, typed_ast) = type_checker::TypeChecker::check(o);
                    match typed_ast {
                        Ok(o) => println!("{:6} : {:?}\n", "typed".yellow(), o),
                        Err(e) => println!("{} : {}", "type error".red(), e)
                    }
                    println!("{:6} : {:?}\n", "symbols".yellow(), checker.symbols);
                    println!("{:6} : {:?}\n", "types".yellow(), checker.types);
                },
                Err(e) => println!("{} : {}", "parse error".red(), e)
            };
            
        },
        None => println!("{} : {}", "io error".red(), "no file path")
    }
}
