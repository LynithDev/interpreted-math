use std::error::Error;

use crate::{parsers::Parser, preced, is_decimal};

pub struct InfixParser;

impl Parser for InfixParser {
    fn parse(input: &str) -> Result<Option<String>, Box<dyn Error>> {
        let mut stack = Vec::<char>::new();
        let mut postfix = String::new();

        for symbol in input.chars() {

            match symbol {
                ' ' | '_' => {
                    continue;
                }

                // Digit
                symbol if symbol.is_digit(10) || is_decimal(&symbol) => {
                    postfix.push(symbol);
                },

                // Opening parantheses or power of
                '(' | '^' => {
                    stack.push(symbol);
                }

                // Closing paranthesis
                ')' => {
                    while !stack.is_empty() && !stack.last().unwrap().eq(&'(') {
                        postfix.push(stack.pop().unwrap());
                    }

                    stack.pop();
                }

                // Other
                _ => {
                    while !stack.is_empty() && preced(&symbol) <= preced(stack.last().unwrap()) {
                        postfix.push(stack.pop().unwrap());
                    }

                    stack.push(symbol);
                    postfix.push(' ');
                }
            }
        }

        while !stack.is_empty() {
            postfix.push(' ');
            postfix.push(stack.pop().unwrap());
        }

        Ok(Some(postfix))
    }
}
