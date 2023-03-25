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

    pub fn define_at_top(&self, name: String, value: ExpressionRes) {
        self.assign_value_to_var(self.envs.len() - 1, name.clone(), value);
    }

    pub fn define_ref_at_top(&self, name: String, value: Rc<RefCell<ExpressionRes>>) {
        self.assign_ref_to_var(self.envs.len() - 1, name.clone(), value);
    }

    pub fn assign_value_to_var(&self, index: usize, name: String, value: ExpressionRes) {
        let rc = self.envs[index].clone();
        let mut ref_mut = rc.try_borrow_mut().unwrap();
        let x = ref_mut.deref_mut();
        x.define_variable(name, value);
    }

    pub fn redefine_value_to_var(&self, index: usize, name: String, value: ExpressionRes) {
        let rc = self.envs[index].clone();
        let mut ref_mut = rc.try_borrow_mut().unwrap();
        let x = ref_mut.deref_mut();
        x.redefine_variable(name, value);
    }

    pub fn assign_ref_to_var(&self, index: usize, name: String, value:  Rc<RefCell<ExpressionRes>>) {
        let rc = self.envs[index].clone();
        let mut ref_mut = rc.try_borrow_mut().unwrap();
        let x = ref_mut.deref_mut();
        x.define_ref(name, value);
    }

    pub fn assign_to_existing(&self, name: String, value: ExpressionRes) {
        let insert = self.get_index(&name);
        self.redefine_value_to_var(insert.unwrap(), name, value);
    }

    pub fn remove_var(&self, name: String ) {
        let option = self.get_index(&name);
        match option {
            None => {}
            Some(index) => {
                self.envs[index].borrow_mut().remove_var(name);
            }
        }
    }

    pub fn lookup_var(&self, name: String) -> Rc<RefCell<ExpressionRes>> {
        for i in (0..self.envs.len()).rev() {
            match self.envs[i].borrow_mut().get_variable(name.clone()) {
                None => { continue; }
                Some(value) => { return value; }
            };
        }
        panic!("missing variable {:?}", name)
    }

    fn get_index(&self, name: &String) -> Option<usize> {
        let mut insert = None;
        for i in (0..self.envs.len()).rev() {
            match self.envs[i].borrow_mut().get_variable(name.clone()) {
                None => {
                    continue;
                }
                Some(found) => {
                    insert = Some(i);
                    break;
                }
            };
        }
        insert
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
    assert_eq!("scoped", rc.borrow().str);
    envs.pop();
    println!("{:#?}", envs);
    let rc2 = envs.lookup_var(String::from("x"));
    assert_eq!("Value", rc2.borrow().str);
}

#[test]
fn remove() {
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
    assert_eq!("scoped", rc.borrow().str);
    envs.remove_var("x".to_string());
    let rc1 = envs.lookup_var(String::from("x"));
    println!("{:#?}", envs);
    let rc2 = envs.lookup_var(String::from("x"));
    assert_eq!("nil", rc2.borrow().str);
}