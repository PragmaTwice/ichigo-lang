use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct IchigoParser;

fn main() {
    let pairs = IchigoParser::parse(Rule::main, "
        ℕ := { 0    : nat
            succ : nat → nat
            }

        plus = λ x : ℕ. λ { (succ y) : ℕ. succ (plus x y)
                            0        : ℕ. x
                        }
    ").unwrap_or_else(|e| panic!("{}", e));
    
    for pair in pairs {
        println!("{}", pair);
    }

}
