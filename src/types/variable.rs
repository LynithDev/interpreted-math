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

    pub fn find_variable<'a>(name: &'a str, list: &'a Vec<Variable>) -> Option<&'a Variable> {
        for var in list {
            if var.name.eq(name) {
                return Some(var);
            }
        }

        None
    }
}

impl Type for Variable {}