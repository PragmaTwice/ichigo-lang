mod syntax;

mod cli;

#[cfg(test)]
mod tests;

fn main() {
    cli::parse_option();
}
