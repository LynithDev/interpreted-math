use crate::parsers::{infix_parser::InfixParser, Parser};

use super::{Evaluator, postfix_evaluator::PostfixEvaluator};

pub struct CodeEvaluator {

}

impl CodeEvaluator {

}

impl Evaluator for CodeEvaluator {
    fn eval(expr: String) -> Result<f32, Box<dyn std::error::Error>> {
        let value = match infix_evaluation(expr.as_str()) {
            Ok(n) => n,
            Err(e) => return Err(e)
        };

        Ok(value)
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
