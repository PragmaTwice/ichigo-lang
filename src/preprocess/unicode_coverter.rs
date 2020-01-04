use super::unicode_symbols::UNICODE_SYMBOLS;
use regex::Regex;

pub fn convert(input: &str) -> Result<String, String> {
    lazy_static! {
        static ref UNICODE_SYMBOL_MATCHER: Regex = Regex::new(r"\\([A-Za-z0-9_^+\-]*)").unwrap();
    }

    let mut current_pos = 0usize;
    let mut current_str = String::new();

    for cap in UNICODE_SYMBOL_MATCHER.captures_iter(input) {
        if cap[1].is_empty() {
            return Err(format!("a unicode symbol name is expected after `\\`"));
        }

        match UNICODE_SYMBOLS.get(&cap[1]) {
            Some(symbol) => {
                current_str += &input[current_pos..cap.get(0).unwrap().start()];
                current_str += symbol;
                current_pos = cap.get(0).unwrap().end();
            }
            None => return Err(format!("unfound unicode symbol name: `{}`", &cap[0])),
        }
    }

    current_str += &input[current_pos..];

    Ok(current_str)
}
