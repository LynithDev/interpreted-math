use std::process::exit;

use crate::{token::{Token, find_variable}, token_type::TokenType, evaluators::{postfix_evaluator::PostfixEvaluator, Evaluator}};

use super::{Parser, infix_parser::InfixParser};

pub struct SyntaxParser;

impl SyntaxParser {
    pub fn new() -> Self {
        Self
    }
    
    fn parse(&self, input: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let mut tokens: Vec<Token> = Vec::new();

        let mut token: Option<Token> = None;

        let mut expression_stack: String = String::new();
        let mut string_stack: String = String::new();

        let mut naming: bool = false;

        for line in input.lines() {
            let line = line.trim_start();

            if line.starts_with("#") {
                continue;
            }

            for symbol in line.chars() {
                match symbol {
                    '#' => continue, // Comments

                    ' ' => continue,

                    symbol if symbol.is_ascii_alphabetic() => {
                        string_stack.push(symbol);
                        
                        match find_variable(&tokens, string_stack.to_owned()) {
                            Some(var) => {
                                string_stack.truncate(string_stack.len() - var.0.len());
                                expression_stack.push_str(var.1.unwrap_or_else(|| 0.0).to_string().as_str());
                                string_stack = String::new();
                            },
                            None => {}
                        }

                        continue;
                    }
    
                    '=' | '\n' => { // Termination for variables
                        if naming && !string_stack.is_empty() {
                            token = Some(Token::new(TokenType::new_variable(string_stack.to_owned(), None)));
                            naming = false;
                            string_stack = String::new();
                            tokens.push(token.unwrap().to_owned());
                            continue;
                        }
                    }
    
                    '$' => {
                        naming = true;
                        continue;
                    }
    
                    _ => {
                        expression_stack.push(symbol);
                    }
                }
            }

            if !expression_stack.is_empty() {
                let value = match infix_evaluation(&expression_stack) {
                    Ok(n) => n,
                    Err(e) => return Err(e)
                };

                if !line.starts_with("$") {
                    tokens.push(Token::new(
                        TokenType::Expression { 
                            value
                        }
                    ));
                } else {
                    let token = tokens.pop();

                    if let Some(token) = token.to_owned() {
                        match token.token_type {
                            TokenType::Variable { name, value: _ } => {
                                tokens.push(Token::new(
                                    TokenType::Variable { 
                                        name, 
                                        value: Some(value)
                                    }
                                ));
                            },
                            _ => {}
                        }
                    }
                }

                expression_stack = String::new();
            }

        }

        println!("{:#?}", tokens);
        
        Ok(None)
    }
}

fn infix_evaluation(input: &str) -> Result<f32, Box<dyn std::error::Error>> {
    let postfix = match InfixParser::parse(input) {
        Ok(postfix) => postfix.unwrap(),
        Err(e) => return Err(e)
    };

    match PostfixEvaluator::eval(postfix) {
        Ok(floet) => Ok(floet),
        Err(e) => return Err(e),
    }
}

impl Parser for SyntaxParser {

    fn parse(input: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let parser = Self::new();
        parser.parse(input)
    }

}