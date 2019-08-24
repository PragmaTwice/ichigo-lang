mod syntax;
mod check;
mod eval;
mod print;

extern crate clap;

mod cli;

#[cfg(test)]
mod tests;

fn main() {
    cli::cli::main();
}
