#[macro_use]
extern crate maplit;
#[macro_use]
extern crate lazy_static;

mod syntax;
mod check;
mod eval;
mod print;
mod preprocess;
mod cli;

#[cfg(test)]
mod tests;

fn main() {
    cli::cli::main();
}
