use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::iter::Rev;
use std::ops::DerefMut;
use std::rc::Rc;


use log::{trace,info, warn, error};
use crate::env::environment::Environment;
use crate::expressions::expression::ExpressionRes;

pub struct ProgramEnvs {
    top: Rc<RefCell<Environment>>,
}

impl Debug for ProgramEnvs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProgramEnvs debug")
            .field("vec", &self.top)
            .finish()
    }
}

impl ProgramEnvs {
    pub fn new() -> ProgramEnvs {
        ProgramEnvs {
            top: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn new_with_env(env: Rc<RefCell<Environment>>) -> ProgramEnvs {
        ProgramEnvs {
            top: env,
        }
    }

    pub fn get_top(&self) ->Rc<RefCell<Environment>> {
        self.top.clone()
    }

    pub fn push(&mut self) {
        self.top = Rc::new(RefCell::new(Environment::new_with_enclosing(self.top.clone())));
    }

    pub fn pop(&mut self) -> Option<Rc<RefCell<Environment>>> {
        let rc = self.top.borrow_mut().enclosing.clone().unwrap();
        self.top = rc;
        Some(self.top.clone())
    }

    pub fn define_at_top(&self, name: String, value: ExpressionRes) {
        self.top.borrow_mut().define_variable(name.clone(), value);
    }

    pub fn define_ref_at_top(&self, name: String, value: Rc<RefCell<ExpressionRes>>) {
        self.top.borrow_mut().define_ref(name.clone(), value);
    }

    pub fn assign_value_to_var(&self, name: String, value: ExpressionRes) {
        match self.get_env(&name) {
            None => {}
            Some(rc) => {
                let mut ref_mut = rc.borrow_mut();
                let x = ref_mut.deref_mut();
                x.define_variable(name, value);
            }
        };
    }

    pub fn assign_ref_to_var(&self, name: String, value: Rc<RefCell<ExpressionRes>>) {
        match self.get_env(&name) {
            None => {}
            Some(rc) => {
                let mut ref_mut = rc.borrow_mut();
                let x = ref_mut.deref_mut();
                x.define_ref(name, value);
            }
        };
    }

    pub fn assign_to_existing(&self, name: String, value: ExpressionRes) {
        match self.get_env(&name) {
            None => {}
            Some(rc) => {
                let mut ref_mut = rc.borrow_mut();
                let x = ref_mut.deref_mut();
                x.redefine_variable(name, value);
            }
        };
    }

    pub fn remove_var(&self, name: String) {
        match self.get_env(&name) {
            None => {}
            Some(env) => {
                env.borrow_mut().remove_var(name);
            }
        }
    }

    pub fn lookup_var(&self, name: String) -> Rc<RefCell<ExpressionRes>> {
        match self.get_env(&name.clone()) {
            None => { panic!("Missing variable {:?} \n state is {:#?}", name, &self.top); }
            Some(value) => { return value.borrow_mut().get_variable(name.clone()).unwrap(); }
        };
    }

    fn get_env(&self, name: &String) -> Option<Rc<RefCell<Environment>>> {
        trace!("Resolving get_env {} ", &name);
        let mut current = self.top.clone();
        loop {
            let mut next = None;
            match current.borrow_mut().get_variable(name.clone()) {
                Some(..) => {
                    return Some(current.clone());
                }
                None => {}
            }
            if let Some(ref mut env) = current.borrow_mut().enclosing {
                match env.borrow_mut().get_variable(name.clone()) {
                    None => {
                        next = Some(env.clone());
                    }
                    Some(val) => {
                        return Some(env.clone());
                    }
                }
            }
            if next.is_none() {
                return None;
            }
            current = next.unwrap();
        }
    }
}

#[test]
fn assign_get() {
    let mut envs = ProgramEnvs::new();
    envs.define_at_top(String::from("x"),
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