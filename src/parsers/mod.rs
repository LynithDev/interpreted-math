pub mod infix_parser;
pub mod syntax_parser;

use std::error::Error;

pub trait Parser {
    fn parse(input: &str) -> Result<Option<String>, Box<dyn Error>>;
}
