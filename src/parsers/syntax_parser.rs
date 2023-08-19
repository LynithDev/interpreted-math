use crate::{evaluators::{Evaluator, code_evaluator::CodeEvaluator}, is_operator, types::{variable::Variable, function::Function}};

use super::Parser;

pub struct SyntaxParser;

impl SyntaxParser {
    pub fn new() -> Self {
        Self
    }
    
    pub fn parse_predefined(&self, input: &str, variables: &Vec<Variable>, functions: &Vec<Function>) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let mut variables: Vec<Variable> = variables.clone();
        let mut functions: Vec<Function> = functions.clone();

        let mut expression_stack: String = String::new();
        let mut string_stack: String = String::new();

        let mut naming: bool = false;
        let mut full_name: bool = false;

        let mut function_mode: bool = false;
        let mut function_body_mode: bool = false;

        #[allow(unused_labels)]
        'line_loop: for (line_number, line) in input.lines().enumerate() {
            let line = line.trim_start();

            if function_body_mode {
                string_stack.push('\n');
            }

            if line.starts_with("#") {
                continue;
            }

            'symbol_loop: for (symbol_index, symbol) in line.chars().enumerate() {
                if function_body_mode {

                    if symbol.eq(&'}') {
                        function_body_mode = false;
                        function_mode = false;

                        let func = functions.pop();

                        if let Some(func) = func {
                            functions.push(Function::new(func.name, Some(string_stack.trim().to_owned()), Vec::new()));
                            string_stack = String::new();
                        }
                        continue 'line_loop;
                    }

                    string_stack.push(symbol);
                    continue;
                }

                match symbol {
                    '#' => break 'symbol_loop, // Comments

                    ' ' => continue,

                    '{' => {
                        function_body_mode = true;
                    },

                    '[' => {
                        full_name = true;
                        continue;
                    }

                    symbol if symbol.eq(&')') && function_mode && !full_name => {
                        continue;
                    }

                    symbol if symbol.is_ascii_alphabetic() || symbol.eq(&']') || symbol.eq(&'(') || (function_mode && symbol.eq(&')')) => {

                        if naming && symbol.eq(&'(') {
                            function_mode = true;
                            naming = false;

                            functions.push(Function::new(string_stack.clone(), None, Vec::new()));
                            string_stack = String::new();
                            
                            continue;
                        }

                        string_stack.push(symbol);

                        if full_name {
                            // Calling functions
                            if symbol.eq(&'(') {
                                function_mode = true;
                                string_stack.pop();
                                continue;
                            }

                            if function_mode && symbol.eq(&')') {
                                string_stack.pop();
                                let func = Function::find_function(string_stack.as_str(), &functions);
                                
                                match func {
                                    Some(func) => {
                                        match func.execute(Vec::new(), &variables, &functions) {
                                            Ok(res) => expression_stack.push_str(res.to_string().as_str()),
                                            Err(e) => return Err(e) 
                                        }
                                        function_mode = false;
                                    },
                                    None => return Err(format!("No function with name '{}' was found", string_stack).into())
                                }

                                string_stack = String::new();
                                continue;
                            }

                            // Waiting for termination to get the full name of the searched variable
                            if !symbol.eq(&']') {
                                continue;
                            }
                        }
                        
                        if !naming {
                            if symbol.eq(&']') {
                                full_name = false;
                                string_stack.pop();
                            }
                            
                            match Variable::find_variable(string_stack.to_owned().as_str(), &variables) {
                                Some(var) => {
                                    string_stack.truncate(string_stack.len() - var.name.len());
                                    
                                    // Implicit variable multiplication
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
    
                    '=' | '\n' => { // Termination for variables and functions
                        if naming && !string_stack.is_empty() {
                            
                            if function_mode {
                                // Remove duplicate functions
                                for (i, func) in functions.to_owned().iter().enumerate() {
                                    if func.name.eq(string_stack.to_owned().as_str()) {
                                        functions.remove(i);
                                    }
                                }

                                functions.push(Function::new(string_stack.to_owned(), None, Vec::new()));
                            } else {
                                // Remove duplicate variables
                                for (i, var) in variables.to_owned().iter().enumerate() {
                                    if var.name.eq(string_stack.to_owned().as_str()) {
                                        variables.remove(i);
                                    }
                                }

                                variables.push(Variable::new(string_stack.to_owned(), None));
                            }

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

            if !function_mode || !function_body_mode {
                if !expression_stack.is_empty() {
                    let value = match CodeEvaluator::eval(expression_stack) {
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

        }

        Ok(None)
    }
}

impl Parser for SyntaxParser {

    fn parse(input: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let parser = Self::new();
        parser.parse_predefined(input, &Vec::new(), &Vec::new())
    }

}