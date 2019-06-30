use super::parser::*;
use std::env;

pub fn parse_option() {
    let args: Vec<_> = env::args().collect();

    let mut arg_iter = args.iter();
    
    let _program_path = arg_iter.next();

    match arg_iter.next() {
        Some(x) => println!("{:#?}", parse_file(x)),
        None => println!("no file path.")
    }
}
