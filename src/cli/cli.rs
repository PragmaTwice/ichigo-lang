use crate::check::type_checker;
use crate::preprocess::unicode_coverter;
use crate::print::printer;
use crate::syntax::{ast, parser};

use colored::*;
use std::fs::*;
use std::io::*;
use structopt::*;

/// A simple CLI for ichigo-lang interpreter
#[derive(StructOpt)]
#[structopt(
    name = "ichigo-lang-cli",
    version = "1.0",
    author = "Pragma Twice <i@twice.moe>"
)]
struct Opt {
    /// sets some ichigo-lang files for input
    #[structopt(name = "INPUT")]
    input_files: Vec<String>,

    /// turns to interactive mode (REPL), ':help' for some useful information
    #[structopt(short, long)]
    interactive: bool,

    /// prints untyped AST for each input code segment
    #[structopt(long)]
    print_ast: bool,
    /// prints typed AST for each input code segment
    #[structopt(long)]
    print_typed_ast: bool,
    /// prints generated code for each printed AST
    #[structopt(long)]
    print_ast_with_generated_code: bool,
    /// prints defined symbols for each input code segment
    #[structopt(long)]
    print_symbols: bool,
    /// prints defined types for each input code segment
    #[structopt(long)]
    print_types: bool,
}

pub struct CLI {
    checker: type_checker::TypeChecker,
    untyped_ast: ast::Main,
    typed_ast: ast::Main,
    options: Opt,
}

impl CLI {
    pub fn new() -> CLI {
        CLI {
            checker: type_checker::TypeChecker::new(),
            untyped_ast: ast::Main::new(),
            typed_ast: ast::Main::new(),
            options: Opt::from_args(),
        }
    }

    pub fn main(&mut self) {
        for input_file in self.options.input_files.clone() {
            println!("processing '{}'...\n", input_file);

            self.analysis_code(input_file.as_str())
        }

        if self.options.interactive {
            self.interactive_loop()
        }
    }

    fn interactive_loop(&mut self) {
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
                        "typed-ast" => println!("{:?}\n", self.typed_ast),
                        "ast" => println!("{:?}\n", self.untyped_ast),
                        "typed-code" => println!("{}\n", printer::print(self.typed_ast.clone())),
                        "code" => println!("{}\n", printer::print(self.untyped_ast.clone())),
                        "symbols" => println!("{:?}\n", self.checker.symbols),
                        "types" => println!("{:?}\n", self.checker.types),
                        _ => println!("{} : {}\n", "command error".red(), "the provided argument to print is unknown, try `help print`")
                    },
                    ["clear"] => {
                        self.checker = type_checker::TypeChecker::new();
                        self.untyped_ast = ast::Main::new();
                        self.typed_ast = ast::Main::new();
                    },
                    ["load", filename] => {
                        self.analysis_file(filename)
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

            self.analysis_code(input_string.as_str());
        }

        println!("bye.");
    }

    fn analysis_code(&mut self, code: &str) {
        match unicode_coverter::convert(code) {
            Ok(preprocessed_code) => match parser::parse_str(preprocessed_code.as_str()) {
                Ok(ast_part) => {
                    if self.options.print_ast {
                        println!("{:6} : {:?}\n", "untyped".yellow(), ast_part);
                        if self.options.print_ast_with_generated_code {
                            println!(
                                "{:6} : \n{}\n",
                                "code".yellow(),
                                printer::print(ast_part.clone())
                            );
                        }
                    }
                    let typed_result = self.checker.check(ast_part.clone());
                    match typed_result {
                        Ok(mut typed_ast_part) => {
                            self.untyped_ast.append(&mut ast_part.clone());
                            self.typed_ast.append(&mut typed_ast_part);
                            if self.options.print_typed_ast {
                                println!("{:6} : {:?}\n", "typed".yellow(), self.typed_ast);
                                if self.options.print_ast_with_generated_code {
                                    println!(
                                        "{:6} : \n{}\n",
                                        "code".yellow(),
                                        printer::print(self.typed_ast.clone())
                                    );
                                }
                            }
                        }
                        Err(e) => println!("{} : {}\n", "type error".red(), e),
                    }
                    if self.options.print_symbols {
                        println!("{:6} : {:?}\n", "symbols".yellow(), self.checker.symbols);
                    }
                    if self.options.print_types {
                        println!("{:6} : {:?}\n", "types".yellow(), self.checker.types);
                    }
                }
                Err(e) => println!("{} : {}\n", "parse error".red(), e),
            },
            Err(e) => println!("{} : {}\n", "preprocess error".red(), e),
        }
    }

    fn analysis_file(&mut self, filename: &str) {
        match read_to_string(filename) {
            Ok(o) => self.analysis_code(o.as_str()),
            Err(e) => println!("{} : {}\n", "io error".red(), e),
        }
    }
}
