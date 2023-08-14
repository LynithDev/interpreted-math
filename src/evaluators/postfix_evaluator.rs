use std::error::Error;

use crate::{eval_operator, is_decimal};

use super::Evaluator;

pub struct PostfixEvaluator;

impl Evaluator for PostfixEvaluator {
    fn eval(expr: String) -> Result<f32, Box<dyn Error>> {
        let mut stack: Vec<f32> = Vec::new();

        for (i, symbol) in expr.chars().enumerate() {
            if symbol.eq(&' ') {
                continue;
            }

            let prev_exists = i > 0 && expr.chars().nth(i - 1).is_some();
            let prev_decimal = prev_exists && is_decimal(&expr.chars().nth(i - 1).unwrap());
            let prev_digit = prev_exists && expr.chars().nth(i - 1).unwrap().is_digit(10);

            match symbol {
                symbol if is_decimal(&symbol) => continue,

                symbol if symbol.is_digit(10) => {
                    let mut digit = symbol.to_string().parse::<f32>().unwrap();
                    if prev_decimal {
                        let prev_number = stack.pop();
                        digit = (digit / 10.0) + prev_number.unwrap_or_else(|| 0.0);
                    }
                    
                    if prev_digit {
                        let prev_number = stack.pop();
                        digit = format!("{}{}", prev_number.unwrap_or_else(|| 0.0), digit).parse::<f32>().unwrap_or_else(|_| digit);
                    }

                    stack.push(digit);
                }

                _ => {
                    let b = stack.pop();
                    let a = stack.pop();

                    if a.is_some() && b.is_some() {
                        stack.push(eval_operator(a.unwrap(), symbol, b.unwrap()).unwrap());
                    }
                }
            }

        }

        match stack.last() {
            Some(stack) => Ok(stack.to_owned()),
            None => Err("Could not parse expression".into())
        }
    }
}
