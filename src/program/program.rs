use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::ops::DerefMut;
use std::rc::Rc;

use crate::env::environment::Environment;
use crate::expressions::expression::ExpressionRes;

pub struct ProgramEnvs {
    envs: Vec<Rc<RefCell<Environment>>>,
}

impl Debug for ProgramEnvs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProgramEnvs debug")
            .field("vec", &self.envs)
            .finish()
    }
}

impl ProgramEnvs {
    pub fn new() -> ProgramEnvs {
        ProgramEnvs {
            envs: vec![Rc::new(RefCell::new(Environment::new()))],
        }
    }

    pub fn push(&mut self) {
        self.envs.push(Rc::from(RefCell::new(Environment::new())));
    }

    pub fn pop(&mut self) -> Option<Rc<RefCell<Environment>>> {
        self.envs.pop()
    }

    pub fn assign_value_to_var(&self, index: usize, name: String, value: ExpressionRes) {
        let rc = self.envs[index].clone();
        let mut ref_mut = rc.try_borrow_mut().unwrap();
        let x = ref_mut.deref_mut();
        x.define_variable(name, value);
    }

    pub fn define_at_top(&self, name: String, value: ExpressionRes) {
        self.assign_value_to_var(self.envs.len() - 1, name, value)
    }

    pub fn assign_to_existing(&self, name: String, value: ExpressionRes) {
        let mut insert = None;
        for i in (0..self.envs.len()).rev() {
            match self.envs[i].borrow_mut().get_variable(name.clone()) {
                None => { continue; }
                Some(found) => {
                    insert = Some(i);
                    break; }
            };
        }
        self.assign_value_to_var(insert.unwrap(), name, value);
    }

    pub fn lookup_var(&self, name: String) -> Rc<ExpressionRes> {
        for i in (0..self.envs.len()).rev() {
            match self.envs[i].borrow_mut().get_variable(name.clone()) {
                None => { continue; }
                Some(value) => { return value; }
            };
        }
        panic!("missing variable {:?}", name)
    }
}

#[test]
fn assign_get() {
    let mut envs = ProgramEnvs::new();
    envs.assign_value_to_var(0,
                             String::from("x"),
                             ExpressionRes::from_str(String::from("Value")));

    let rc = envs.lookup_var(String::from("x"));
    let rc2 = envs.lookup_var(String::from("x"));
    print!("{:?}, {rc2:?}", rc);
}

#[test]
fn push_scoped() {
    let mut envs = ProgramEnvs::new();
    envs.define_at_top(
        String::from("x"),
        ExpressionRes::from_str(String::from("Value")));
    envs.push();
    envs.define_at_top(
        String::from("x"),
        ExpressionRes::from_str(String::from("scoped")));

    println!("{:#?}", envs);
    let rc = envs.lookup_var(String::from("x"));
    assert_eq!("scoped", rc.str);
    envs.pop();
    println!("{:#?}", envs);
    let rc2 = envs.lookup_var(String::from("x"));
    assert_eq!("Value", rc2.str);
}