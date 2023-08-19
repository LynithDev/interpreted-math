use super::Type;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Variable {
    pub name: String,
    pub value: Option<f32>
}

impl Variable {
    pub fn new(name: String, value: Option<f32>) -> Self {
        Self { name, value }
    }

    pub fn empty(name: String) -> Self {
        Variable::new(name, None)
    }

    pub fn find_variable(name: &str, list: &Vec<Variable>) -> Option<Variable> {
        for var in list {
            if var.name.eq(name) {
                return Some(var.to_owned());
            }
        }

        None
    }
}

impl Type for Variable {}