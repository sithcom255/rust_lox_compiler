use std::collections::HashMap;
use crate::expressions::expression::ExpressionRes;

pub struct Environment {
    variables : HashMap<String, ExpressionRes>,
    functions : HashMap<String, Function>,
    classes: HashMap<String, Class>,
}

impl Environment {
    pub fn new() -> Environment  {
        Environment {
            variables: Default::default(),
            functions: Default::default(),
            classes: Default::default(),
        }
    }

    pub fn define_variable(&mut self, name: String, expr : ExpressionRes) {
        self.variables[&name] = expr;
    }

    pub fn get_variable(&mut self, name: &str) -> Option<&ExpressionRes> {
        self.variables.get(name)
    }
}


struct Class {
    attributes : HashMap<String, ExpressionRes>,
    functions : HashMap<String, Function>,
}

struct Function {

}