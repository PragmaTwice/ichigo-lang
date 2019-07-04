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
            println!("untyped : {:?}", ast);
            println!();
            let typed_ast = type_checker::check(ast);
            println!("typed   : {:?}", typed_ast);
        },
        None => println!("no file path.")
    }
}
