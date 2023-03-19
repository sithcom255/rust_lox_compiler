use std::collections::HashMap;
use std::rc::Rc;

use crate::expressions::expression::ExpressionRes;

#[derive(Debug)]
pub struct Environment {
    variables : HashMap<String, Rc<ExpressionRes>>,
    functions : HashMap<String, Rc<Function>>,
    classes: HashMap<String, Rc<Class>>,
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
        self.variables.insert(name,Rc::new(expr));
    }

    pub fn get_variable(&mut self, name: String) -> Option<Rc<ExpressionRes>> {
        let option = self.variables.get(&name);
        match option {
            None => { None}
            Some(value) => {
                let rc = value.clone();
                Some(rc)
            }
        }

    }
}

#[derive(Debug)]
struct Class {
    attributes : HashMap<String, ExpressionRes>,
    functions : HashMap<String, Function>,
}
#[derive(Debug)]
struct Function {

}