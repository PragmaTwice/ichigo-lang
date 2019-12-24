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
    let mut untyped_ast = ast::Main::new();
    let mut typed_ast = ast::Main::new();

    for  input_file in input_files {
        println!("processing '{}'...\n", input_file);

        let optional_ast = parser::parse_file(input_file);
        analysis_ast(optional_ast, &mut checker, &mut untyped_ast, &mut typed_ast, &matches);
    }

    if matches.is_present("interactive_mode") {
        println!("turning to interactive mode...");


        loop {
            print!("\n{}", "> ".green());
            let _ = stdout().flush();

            let mut input_string = String::new();
            stdin().read_to_string(&mut input_string).expect("did not enter a correct string");
            
            let trimed_input_string = input_string.trim();
            if trimed_input_string.starts_with(":") {
                let input_command = (&trimed_input_string[1..]).split_whitespace().collect::<Vec<_>>();
                match input_command[..] {
                    ["exit"] | ["quit"] => break,
                    ["print", printed] => match printed {
                        "typed-ast" => println!("{:6} : {:?}\n", "typed ast".yellow(), typed_ast),
                        "ast" => println!("{:6} : {:?}\n", "ast".yellow(), untyped_ast),
                        "typed-code" => println!("{:6} : \n{}\n", "typed code".yellow(), printer::print(typed_ast.clone())),
                        "code" => println!("{:6} : \n{}\n", "code".yellow(), printer::print(untyped_ast.clone())),
                        "symbols" => println!("{:6} : {:?}\n", "symbols".yellow(), checker.symbols),
                        _ => println!("{} : {}\n", "command error".red(), "only `(typed-)ast`, `(typed-)code`, `symbols` is functional")
                    },
                    ["clear"] => {
                        checker = type_checker::TypeChecker::new();
                        untyped_ast = ast::Main::new();
                        typed_ast = ast::Main::new();
                    },
                    ["load", filename] => {
                        let optional_ast = parser::parse_file(filename);
                        analysis_ast(optional_ast, &mut checker, &mut untyped_ast, &mut typed_ast, &matches);
                    }
                    ["help"] => println!("{} : {}\n", "command info".yellow(), "`exit`, `quit`, `clear`, `print <something>`, `load <filename>` is expected, give a try"),
                    [] => println!("{} : {}\n", "command error".red(), "a command follow `:` is expected but not provided"),
                    _ => println!("{} : {}\n", "command error".red(), "the given command is not found, try `help`")
                }
                continue;
            }
            
            let optional_ast = parser::parse_str(input_string.as_str());
            analysis_ast(optional_ast, &mut checker, &mut untyped_ast, &mut typed_ast, &matches);
        }

        println!("bye.");
    }
    
}

fn analysis_ast(optional_ast : parser::ParseResult<ast::Main>, 
               checker : &mut type_checker::TypeChecker,
               untyped_ast : &mut ast::Main,
               typed_ast : &mut ast::Main,
               matches: &ArgMatches) {
    match optional_ast {
        Ok(ast_part) => {
            if matches.is_present("print_untyped_ast") {
                println!("{:6} : {:?}\n", "untyped".yellow(), ast_part);
                if matches.is_present("print_ast_with_generated_code") {
                    println!("{:6} : \n{}\n", "code".yellow(), printer::print(ast_part.clone()));
                }
            }
            let typed_result = checker.check(ast_part.clone());
            match typed_result {
                Ok(mut typed_ast_part) => {
                    untyped_ast.append(&mut ast_part.clone());
                    typed_ast.append(&mut typed_ast_part);
                    if matches.is_present("print_typed_ast") {
                        println!("{:6} : {:?}\n", "typed".yellow(), typed_ast);
                        if matches.is_present("print_ast_with_generated_code") {
                            println!("{:6} : \n{}\n", "code".yellow(), printer::print(typed_ast.clone()));
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
