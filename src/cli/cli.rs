use crate::syntax::{parser, ast};
use crate::check::type_checker;
use crate::print::printer;

use colored::*;
use clap::App;

pub fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input_files: Vec<_> = matches.values_of("INPUT").unwrap().collect();

    let mut checker = type_checker::TypeChecker::new();
    let mut typed_ast = ast::Main::new();

    for  input_file in input_files {
        println!("processing '{}'\n", input_file);

        let optional_ast = parser::parse_file(input_file);

        match optional_ast {
            Ok(ast_part) => {
                if matches.is_present("print_untyped_ast") {
                    println!("{:6} : {:?}\n", "untyped".yellow(), ast_part);
                    if matches.is_present("print_ast_with_generated_code") {
                        println!("{:6} : {}\n", "code".yellow(), printer::print(ast_part.clone()));
                    }
                }
                let typed_result = checker.check(ast_part);
                match typed_result {
                    Ok(mut typed_ast_part) => {
                        typed_ast.append(&mut typed_ast_part);
                        if matches.is_present("print_typed_ast") {
                            println!("{:6} : {:?}\n", "typed".yellow(), typed_ast);
                            if matches.is_present("print_ast_with_generated_code") {
                                println!("{:6} : {}\n", "code".yellow(), printer::print(typed_ast.clone()));
                            }
                        }
                    },
                    Err(e) => println!("{} : {}\n", "type error".red(), e)
                }
                println!("{:6} : {:?}\n", "symbols".yellow(), checker.symbols);
            },
            Err(e) => println!("{} : {}\n", "parse error".red(), e)
        }
    }

    if matches.is_present("interactive_mode") {

    }
    
}
