use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::expressions::expression::ExpressionRes;

#[derive(Debug)]
pub struct Environment {
    variables: HashMap<String, Rc<RefCell<ExpressionRes>>>,
    classes: HashMap<String, Rc<Class>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            variables: Default::default(),
            classes: Default::default(),
        }
    }

    pub fn define_variable(&mut self, name: String, expr: ExpressionRes) {
        self.variables.insert(name, Rc::new(RefCell::new(expr)));
    }
    pub fn redefine_variable(&mut self, name: String, expr: ExpressionRes) {
        let option = self.variables.get(&name).unwrap();
        let mut ref_mut = option.replace(expr);
    }


    pub fn define_ref(&mut self, name: String, expr: Rc<RefCell<ExpressionRes>>) {
        self.variables.insert(name, expr);
    }

    pub fn get_variable(&mut self, name: String) -> Option<Rc<RefCell<ExpressionRes>>> {
        let option = self.variables.get(&name);
        match option {
            None => { None }
            Some(value) => {
                let rc = value.clone();
                Some(rc)
            }
        }
    }

    pub fn remove_var(&mut self, name: String) {
        self.variables.remove(&*name);
        self.variables.insert(name, Rc::new(RefCell::new(ExpressionRes::from_none())));
    }
}

#[derive(Debug)]
struct Class {
    attributes: HashMap<String, ExpressionRes>,
    functions: HashMap<String, Function>,
}

#[derive(Debug)]
struct Function {}