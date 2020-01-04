mod check;
mod cli;
mod eval;
mod preprocess;
mod print;
mod syntax;

#[cfg(test)]
mod tests;

use cli::cli::CLI;

fn main() {
    CLI::new().main()
}
