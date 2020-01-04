use crate::check::type_checker;
use crate::preprocess::unicode_coverter;
use crate::print::printer;
use crate::syntax::{ast, parser};

use clap::*;
use colored::*;
use std::fs;
use std::io::*;

pub fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input_files: Vec<_> = matches.values_of("INPUT").unwrap_or_default().collect();

    let mut checker = type_checker::TypeChecker::new();
    let mut untyped_ast = ast::Main::new();
    let mut typed_ast = ast::Main::new();

    for input_file in input_files {
        println!("processing '{}'...\n", input_file);

        match fs::read_to_string(input_file) {
            Ok(o) => analysis_code(
                o.as_str(),
                &mut checker,
                &mut untyped_ast,
                &mut typed_ast,
                &matches,
            ),
            Err(e) => println!("{} : {}\n", "io error".red(), e),
        }
    }

    if matches.is_present("interactive_mode") {
        println!("turning to interactive mode...");
        println!("{} : {}\n", "hint".yellow(), "command starting with `:` or ichigo-lang code is expected, both of which should be end by an EOF");

        loop {
            print!("\n{}", "> ".green());
            let _ = stdout().flush();

            let mut input_string = String::new();
            stdin()
                .read_to_string(&mut input_string)
                .expect("did not enter a correct string");

            let trimed_input_string = input_string.trim();
            if trimed_input_string.starts_with(":") {
                let input_command = (&trimed_input_string[1..])
                    .split_whitespace()
                    .collect::<Vec<_>>();
                match input_command[..] {
                    ["exit"] | ["quit"] => break,
                    ["print", printed] => match printed {
                        "typed-ast" => println!("{:?}\n", typed_ast),
                        "ast" => println!("{:?}\n", untyped_ast),
                        "typed-code" => println!("{}\n", printer::print(typed_ast.clone())),
                        "code" => println!("{}\n", printer::print(untyped_ast.clone())),
                        "symbols" => println!("{:?}\n", checker.symbols),
                        "types" => println!("{:?}\n", checker.types),
                        _ => println!("{} : {}\n", "command error".red(), "the provided argument to print is unknown, try `help print`")
                    },
                    ["clear"] => {
                        checker = type_checker::TypeChecker::new();
                        untyped_ast = ast::Main::new();
                        typed_ast = ast::Main::new();
                    },
                    ["load", filename] => {
                        match fs::read_to_string(filename) {
                            Ok(o) => analysis_code(o.as_str(), &mut checker, &mut untyped_ast, &mut typed_ast, &matches),
                            Err(e) => println!("{} : {}\n", "io error".red(), e)
                        }
                    }
                    ["help"] => println!("{} : {}\n", "command info".yellow(), "`exit` (or `quit`), `clear`, `print <something>`, `load <filename>` is expected, give a try with `help <command>`"),
                    ["help", command] => match command {
                        "exit" | "quit" => println!("{} : {}\n", "command info".yellow(), "to exit the REPL"),
                        "print" => println!("{} : {}\n", "command info".yellow(), "to print some valuable information, as which `(typed-)ast`, `(typed-)code`, `symbols`, `types` is expected"),
                        "clear" => println!("{} : {}\n", "command info".yellow(), "to clear all input, including input file"),
                        "load" => println!("{} : {}\n", "command info".yellow(), "to load a ichigo-lang code file according to a filename"),
                        _ => println!("{} : {}\n", "command error".red(), "the given command is not found")
                    },
                    [] => println!("{} : {}\n", "command error".red(), "a command follow `:` is expected but not provided, try `help`"),
                    _ => println!("{} : {}\n", "command error".red(), "the given command is not found, try `help`")
                }
                continue;
            }

            analysis_code(
                input_string.as_str(),
                &mut checker,
                &mut untyped_ast,
                &mut typed_ast,
                &matches,
            );
        }

        println!("bye.");
    }
}

fn analysis_code(
    code: &str,
    checker: &mut type_checker::TypeChecker,
    untyped_ast: &mut ast::Main,
    typed_ast: &mut ast::Main,
    matches: &ArgMatches,
) {
    match unicode_coverter::convert(code) {
        Ok(preprocessed_code) => match parser::parse_str(preprocessed_code.as_str()) {
            Ok(ast_part) => {
                if matches.is_present("print_untyped_ast") {
                    println!("{:6} : {:?}\n", "untyped".yellow(), ast_part);
                    if matches.is_present("print_ast_with_generated_code") {
                        println!(
                            "{:6} : \n{}\n",
                            "code".yellow(),
                            printer::print(ast_part.clone())
                        );
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
                                println!(
                                    "{:6} : \n{}\n",
                                    "code".yellow(),
                                    printer::print(typed_ast.clone())
                                );
                            }
                        }
                    }
                    Err(e) => println!("{} : {}\n", "type error".red(), e),
                }
                if matches.is_present("print_symbols") {
                    println!("{:6} : {:?}\n", "symbols".yellow(), checker.symbols);
                }
                if matches.is_present("print_types") {
                    println!("{:6} : {:?}\n", "types".yellow(), checker.types);
                }
            }
            Err(e) => println!("{} : {}\n", "parse error".red(), e),
        },
        Err(e) => println!("{} : {}\n", "preprocess error".red(), e),
    }
}
