#[derive(Debug, Clone)]
pub enum TokenType {
    Variable {
        name: String,
        value: Option<f32>
    },

    Expression {
        value: f32
    },

    Unknown {

    }
}

impl TokenType {
    pub fn new_variable(name: String, value: Option<f32>) -> Self {
        Self::Variable { name, value }
    }

    pub fn new_expression(value: f32) -> Self {
        Self::Expression { value }
    }

    pub fn unknown() -> Self {
        Self::Unknown {  }
    }
}
