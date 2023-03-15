use std::cell::RefCell;
use std::ops::{ DerefMut};
use std::rc::Rc;

use crate::env::environment::Environment;
use crate::expressions::expression::ExpressionRes;

pub struct ProgramEnvs {
    envs: Vec<Rc<RefCell<Environment>>>,
}

impl ProgramEnvs {
    pub fn new() -> ProgramEnvs {
        ProgramEnvs {
            envs: vec![ Rc::new(RefCell::new(Environment::new()))],
        }
    }

    pub fn push(&mut self) {
        self.envs.push(Rc::from(RefCell::new(Environment::new())));
    }

    pub fn pop(&mut self) {
        self.envs.pop();
    }

    pub fn assign_value_to_var(&self, index: usize, name: String, value: ExpressionRes) {
        let rc = self.envs[index].clone();
        let mut ref_mut = rc.try_borrow_mut().unwrap();
        let x = ref_mut.deref_mut();
        println!("defined variable {name:?}");
        x.define_variable(name, value);
    }
}