mod syntax;
mod check;
mod eval;
mod print;

mod cli;

#[cfg(test)]
mod tests;

fn main() {
    cli::parse_option();
}
