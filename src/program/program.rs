use core::panicking::panic;
use std::cell::RefCell;
use std::rc::Rc;
use crate::env::environment::Environment;

pub struct ProgramEnvs {
    envs: Vec<Option<Rc<RefCell<Environment>>>>,
}

impl ProgramEnvs {
    fn new() -> ProgramEnvs {
        ProgramEnvs {
            envs: vec![None, 10],
        }
    }

    fn push(&mut self) {
        self.envs.push(Some(Rc::from(RefCell::new(Environment::new()))));
    }

    fn pop(&mut self) {
        self.envs.pop();
    }
}