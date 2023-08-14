use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Self {
            token_type
        }
    }
}

pub fn find_variable(tokens: &Vec<Token>, query: String) -> Option<(String, Option<f32>)> {
    for token in tokens {
        match token.to_owned().token_type {
            TokenType::Variable { name, value } => {
                if name.eq_ignore_ascii_case(&query) {
                    return Some((name, value));
                }
            }, 
            _ => {}
        }
    }

    None
}
