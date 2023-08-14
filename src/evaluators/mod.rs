use std::error::Error;

pub mod postfix_evaluator;

pub trait Evaluator {
    fn eval(expr: String) -> Result<f32, Box<dyn Error>>;
}