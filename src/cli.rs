use super::syntax::parser;
use super::check::type_checker;
use super::print::printer;
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
                    println!("{:6} : {}\n", "code".yellow(), printer::print(o.clone()));

                    let (checker, typed_ast) = type_checker::TypeChecker::from_check(o);
                    match typed_ast {
                        Ok(o) => {
                            println!("{:6} : {:?}\n", "typed".yellow(), o);
                            println!("{:6} : {}\n", "code".yellow(), printer::print(o.clone()));
                        },
                        Err(e) => println!("{} : {}\n", "type error".red(), e)
                    }
                    println!("{:6} : {:?}\n", "symbols".yellow(), checker.symbols);
                },
                Err(e) => println!("{} : {}\n", "parse error".red(), e)
            };
            
        },
        None => println!("{} : {}\n", "io error".red(), "no file path")
    }
}
