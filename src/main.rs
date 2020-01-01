mod syntax;
mod check;
mod eval;
mod print;

#[macro_use]
extern crate maplit;

mod cli;

#[cfg(test)]
mod tests;

fn main() {
    cli::cli::main();
}
