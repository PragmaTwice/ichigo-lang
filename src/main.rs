#[macro_use]
extern crate maplit;
#[macro_use]
extern crate lazy_static;

mod check;
mod cli;
mod eval;
mod preprocess;
mod print;
mod syntax;

#[cfg(test)]
mod tests;

fn main() {
    cli::cli::main();
}
