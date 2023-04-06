use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::rc::Rc;

use crate::env::environment::Environment;
use crate::expressions::expression::ExpressionRes;
use crate::program::program::ProgramEnvs;
use crate::statements::statement::Statement;
use crate::statements::stmt_visitor::{StatementInterpreter, StatementRes, StmtVisitor};

#[derive(Debug,  Clone)]
pub struct Method {
    pub name: String,
    pub args: Vec<ExpressionRes>,
    pub body: Statement,
    pub captured_env: Rc<RefCell<Environment>>,
}

impl Method {
    pub fn new(name: String, args: Vec<ExpressionRes>, body: Statement, captured: Environment) -> Method {
        Method {
            name,
            args,
            body,
            captured_env: Rc::new(RefCell::new(captured)),
        }
    }

    pub fn prepare_for_call(&self, env: Environment) -> Method {
        Method {
            name: self.name.clone(),
            args: self.args.clone(),
            body: self.body.clone(),
            captured_env: Rc::new(RefCell::new((env))),
        }
    }

    // this should be just the params right now, not the captured stuff
    pub fn call(&self, arguments: Rc<RefCell<ProgramEnvs>>) -> Result<StatementRes, String> {
        let mut interpreter = StatementInterpreter::new_with_envs(
            arguments);
        return interpreter.eval(&self.body);
    }
}

#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub args: Vec<ExpressionRes>,
    pub methods: Vec<Rc<Method>>,
}

impl Class {
    pub fn new(name: String) -> Class {
        Class {
            name,
            args: vec![],
            methods: vec![],
        }
    }
    pub fn new_class(name: String, args: Vec<ExpressionRes>, methods: Vec<Rc<Method>>) -> Class {
        Class {
            name,
            args,
            methods,
        }
    }

    pub fn add_method(&mut self, name: String, args: Vec<ExpressionRes>, body: Statement) {
        let method = Method::new(name, args, body, Environment::new());
        self.methods.push(Rc::new(method));
    }

    // this should be just the params right now, not the captured stuff
    pub fn call(&self, env: Rc<RefCell<Environment>>, class: Rc<Class>) -> Result<Instance, String> {

        Ok(Instance { class, env })
    }
}

#[derive(Debug)]
pub struct Instance {
    pub class: Rc<Class>,
    pub env: Rc<RefCell<Environment>>,
}

impl Instance {
    fn new(class: Rc<Class>) -> Instance {
        Instance {
            class,
            env: Rc::new(RefCell::new(Environment::new()))
        }
    }

    fn get_function_def(&self, method: String) -> Rc<Method> {
        for x in &self.class.deref().methods  {
            if x.name == method {
                let rc = x.clone();
                return rc;
            }
        }
        panic!("did not find the method def")
    }

    fn get_child_env(&self) -> Environment {
        Environment::new_with_enclosing(self.env.clone())
    }

    fn call(&mut self, method: String, arguments: Environment) -> Result<StatementRes, String> {
        let envs = ProgramEnvs::new_with_env(Rc::new(RefCell::new(arguments)));
        let program_envs = Rc::new(RefCell::new(envs));
        for x in &self.class.deref().methods  {
            if x.name == method {
                return x.call(program_envs.clone())
            }
        }
        panic!("did not find the method def")
    }
}