use crate::{evaluators::{postfix_evaluator::PostfixEvaluator, Evaluator}, is_operator, types::variable::Variable};

use super::{Parser, infix_parser::InfixParser};

pub struct SyntaxParser;

impl SyntaxParser {
    pub fn new() -> Self {
        Self
    }
    
    fn parse(&self, input: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let mut variables: Vec<Variable> = Vec::new();

        let mut expression_stack: String = String::new();
        let mut string_stack: String = String::new();

        let mut naming: bool = false;
        let mut full_variable_name: bool = false;

        #[allow(unused_labels)]
        'line_loop: for (line_number, line) in input.lines().enumerate() {
            let line = line.trim_start();

            if line.starts_with("#") {
                continue;
            }

            'symbol_loop: for (symbol_index, symbol) in line.chars().enumerate() {
                match symbol {
                    '#' => break 'symbol_loop, // Comments

                    ' ' => continue,

                    '[' => {
                        full_variable_name = true;
                        continue;
                    }

                    symbol if symbol.is_ascii_alphabetic() || symbol.eq(&']') => {

                        string_stack.push(symbol);
                        
                        if full_variable_name && !symbol.eq(&']') {
                            continue;
                        }
                        
                        if !naming {
                            if symbol.eq(&']') {
                                full_variable_name = false;
                                string_stack.pop();
                            }
                            
                            match Variable::find_variable(string_stack.as_str(), &variables) {
                                Some(var) => {
                                    string_stack.truncate(string_stack.len() - var.name.len());
                                    
                                    if symbol_index != 0 
                                    && line.chars().nth(symbol_index - 1).is_some()
                                    && line.chars().nth(symbol_index - 1).unwrap().is_ascii_alphabetic()
                                    && !naming
                                    && string_stack.is_empty()
                                    && !expression_stack.is_empty()
                                    && !expression_stack.ends_with(|c| is_operator(&c)) {
                                        expression_stack.push('*');
                                    }
                                    
                                    expression_stack.push_str(var.value.unwrap_or_else(|| 0.0).to_string().as_str());
                                    string_stack = String::new();
                                },
                                None => {}
                            }
                        }

                        continue;
                    }
    
                    '=' | '\n' => { // Termination for variables
                        if naming && !string_stack.is_empty() {
                            // Remove duplicate variables
                            for (i, var) in variables.to_owned().iter().enumerate() {
                                if var.name.eq(string_stack.to_owned().as_str()) {
                                    variables.remove(i);
                                }
                            }

                            variables.push(Variable::new(string_stack.to_owned(), None));
                            naming = false;
                            string_stack = String::new();
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
                    Err(e) => return Err(format!("Line {}: {}", line_number + 1, e).into())
                };

                if !line.starts_with("$") {
                    return Ok(Some(value.to_string()));
                } else {
                    let var = variables.pop();

                    if let Some(var) = var {
                        variables.push(Variable::new(var.name, Some(value)))
                    }
                }

                expression_stack = String::new();
            }

            if !string_stack.is_empty() {
                println!("\u{001b}[0;33mWarning: Line {}: Unknown letters '{}' \u{001b}[0;39m(If it's part of a long variable name, try surrounding it in [])\u{001b}[0;m", line_number + 1, string_stack);
            }
        }

        // if std::env::args().len() > 2 && std::env::args().nth(2).eq(&Some("true".to_string())) {
        //     println!("{:#?}", tokens);
        // }
        
        // for token in tokens {
        //     match token.token_type {
        //         TokenType::Expression { value } => {
        //             return Ok(Some(value.to_string()));
        //         },
        //         _ => {}
        //     }
        // }

        

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