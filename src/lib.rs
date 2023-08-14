pub mod parsers;
pub mod evaluators;
pub mod token;
pub mod token_type;

pub fn preced(symbol: &char) -> u8 {
    match symbol {
        '^' => 3,
        '/' | '*' => 2,
        '+' | '-' => 1,
        _ => 0
    }
}

pub fn is_operator(symbol: &char) -> bool {
    match symbol {
        '+' | '-' | '/' | '*' => true,
        _ => false
    }
}

pub fn eval_operator(a: f32, op: char, b: f32) -> Option<f32> {
    match op {
        '+' => Some(a + b),
        '-' => Some(a - b),
        '*' => Some(a * b),
        '/' => Some(a / b),
        '^' => Some(a.powf(b)),
        _ => None
    }
}

pub fn is_decimal(symbol: &char) -> bool {
    match symbol {
        ',' | '.' => true,
        _ => false,
    }
}
