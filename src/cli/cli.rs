use crate::syntax::{parser, ast};
use crate::check::type_checker;
use crate::print::printer;

use colored::*;
use clap::*;
use std::io::*;

pub fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input_files: Vec<_> = matches.values_of("INPUT").unwrap_or_default().collect();

    let mut checker = type_checker::TypeChecker::new();
    let mut typed_ast = ast::Main::new();

    for  input_file in input_files {
        println!("processing '{}'...\n", input_file);

        let optional_ast = parser::parse_file(input_file);

        analysis_ast(optional_ast, &mut checker, &mut typed_ast, &matches);
    }

    if matches.is_present("interactive_mode") {
        println!("turning to interactive mode...");


        loop {
            print!("\n{}", "> ".green());
            let _ = stdout().flush();

            let mut input_string = String::new();
            stdin().read_to_string(&mut input_string).expect("did not enter a correct string");
            
            if input_string == "exit" {
                break;
            }

            let optional_ast = parser::parse_str(input_string.as_str());
            analysis_ast(optional_ast, &mut checker, &mut typed_ast, &matches);
        }

        println!("bye.");
    }
    
}

fn analysis_ast(optional_ast : parser::ParseResult<ast::Main>, 
               checker : &mut type_checker::TypeChecker,
               typed_ast : &mut ast::Main,
               matches: &ArgMatches) {
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
