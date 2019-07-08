mod syntax;
mod check;
mod eval;

mod cli;

#[cfg(test)]
mod tests;

fn main() {
    cli::parse_option();
}
