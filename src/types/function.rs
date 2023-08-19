use crate::parsers::syntax_parser::SyntaxParser;

use super::{Type, variable::Variable};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Function {
    pub name: String,
    pub code: Option<String>,
    pub arguments: Vec<Variable>
}

impl Function {
    pub fn new(name: String, code: Option<String>, arguments: Vec<Variable>) -> Self {
        Self { 
            name, 
            code,
            arguments
        }
    } 

    pub fn find_function<'a>(name: &'a str, list: &'a Vec<Function>) -> Option<&'a Function> {
        for func in list {
            if func.name.eq(name) {
                return Some(func);
            }
        }

        None
    }

    pub fn execute(&self, args: Vec<Variable>, defined_variables: &Vec<Variable>, defined_functions: &Vec<Function>) -> Result<f32, Box<dyn std::error::Error>> {
        if self.arguments.len() > args.len() {
            return Err(format!("Function '{}' requires '{}' argument(s) but was only provided '{}' argument(s)", self.name, self.arguments.len(), args.len()).into());
        }

        if self.code.is_none() {
            return Ok(0.0)
        }

        let value = SyntaxParser::parse_predefined(&SyntaxParser {}, &self.code.to_owned().unwrap(), defined_variables, defined_functions)?;
        let value: f32 = match value {
            Some(value) => value.parse()?,
            None => return Err(format!("Function '{}' returned no value", self.name).into())
        };
        
        Ok(value)
    }
}

impl Type for Function {}