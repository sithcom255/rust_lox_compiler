use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::os::linux::raw::stat;
use std::rc::Rc;

use log::{trace,info, warn, error};


use crate::env::environment::Environment;
use crate::expressions::expression::ExpressionRes;
use crate::expressions::expression::ExprResType::{Boolean, Identifier};
use crate::expressions::visitor::{ExpressionInterpreter, Visitor};
use crate::program::program::ProgramEnvs;
use crate::program::runtime::Method;
use crate::resolver_visitor::resolver::{CaptureResolver, Resolve};
use crate::statements::statement::Statement;
use crate::statements::stmt_visitor::StatementRes::{Expr, Void};

pub trait StmtVisitor {
    fn eval(&mut self, object: &Statement) -> Result<StatementRes, String>;
}

pub struct StatementInterpreter {
    pub expression_visitor: Rc<dyn Visitor<ExpressionRes>>,
    pub envs: Rc<RefCell<ProgramEnvs>>,
    pub statements: Vec<Box<Statement>>,
}

impl StmtVisitor for StatementInterpreter {
    fn eval(&mut self, object: &Statement) -> Result<StatementRes, String> {
        match object {
            Statement::Stmt { expr } => {
                trace!("Entering {} ", "Stmt");
                self.expression_visitor.eval((**expr).clone());
                Ok(Void)
            }
            Statement::IfStatement { expr, body, else_body } => {
                trace!("Entering {} ", "IfStatement");
                let res = self.expression_visitor.eval((*expr).clone());
                if res.type_ != Boolean {
                    panic!("if (.expr.) not evaluatable to bool")
                }
                if res.boolean {
                    match self.eval(body) {
                        Ok(Void) => {}
                        Ok(Expr { res }) => { return Ok(Expr { res }); }
                        Err(_) => {}
                    };
                } else {
                    match else_body.as_ref() {
                        None => {}
                        Some(value) => {
                            match self.eval(value) {
                                Ok(Void) => {}
                                Ok(Expr { res }) => { return Ok(Expr { res }); }
                                Err(_) => {}
                            };
                        }
                    };
                }
                Ok(Void)
            }
            Statement::FunStatement { identifier, args, block } => {
                trace!("Entering {} ", "FunStatement");
                let mut arguments = vec![];
                for arg in args {
                    arguments.push(self.expression_visitor.eval((*arg).clone()));
                }
                let body1 = (**block.as_ref().unwrap()).clone();
                let mut resolver = CaptureResolver::new_with_environment(self.envs.clone(), &arguments);
                resolver.resolve_statement(&body1);
                let map = resolver.get_captured();
                let mut environment = Environment::new_with_enclosing(self.envs.borrow().get_top());
                for (key, value) in map.into_iter() {
                    environment.define_ref(key, value)
                }
                let method = ExpressionRes::from_method(Method::new(identifier.value.clone(),
                                                                    arguments,
                                                                    body1, environment));


                // figure out the what variables does the method contain
                self.envs.try_borrow_mut().unwrap().define_at_top(identifier.value.clone(),
                                                                  method);
                return Ok(Void);
            }
            Statement::WhileStatement { expr, body: statements } => {
                trace!("Entering {} ", "WhileStatement");
                let condition = expr.clone();
                let mut res1 = self.expression_visitor.eval((*condition).clone());

                while res1.boolean {
                    match self.eval(statements) {
                        Ok(Void) => {}
                        Ok(Expr { res }) => { return Ok(Expr { res }); }
                        Err(_) => {}
                    };
                    res1 = self.expression_visitor.eval((*condition).clone());
                }
                Ok(Void)
            }
            Statement::ForStatement { initiation, condition, increment, body } => {
                trace!("Entering {} ", "ForStatement");
                match initiation {
                    None => {}
                    Some(value) => {
                        self.eval(value);
                    }
                }

                let mut res1 = ExpressionRes::from_bool(true);

                match condition {
                    None => { res1 = ExpressionRes::from_bool(false) }
                    Some(value) => {
                        let statement = value.clone();
                        match *statement {
                            Statement::Stmt { expr } => {
                                res1 = self.expression_visitor.eval(*expr);
                            }
                            _ => {}
                        }
                    }
                }

                while res1.boolean {
                    match self.eval(body) {
                        Ok(Void) => {}
                        Ok(Expr { res }) => { return Ok(Expr { res }); }
                        Err(_) => {}
                    };
                    match increment {
                        None => {}
                        Some(value) => { self.eval(value); }
                    }
                    match condition {
                        None => { res1 = ExpressionRes::from_bool(false) }
                        Some(value) => {
                            let statement = value.clone();
                            match *statement {
                                Statement::Stmt { expr } => {
                                    res1 = self.expression_visitor.eval(*expr);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                return Ok(Void);
            }
            Statement::PrintStatement { expr } => {
                trace!("Entering {} ", "PrintStatement");
                let x1 = (*expr).clone();
                let res = self.expression_visitor.eval(*x1);

                if res.type_ == Identifier {
                    let x = self.lookup_variable(res.str);
                    println!("{}", x.borrow().print())
                } else {
                    println!("{}", res.print())
                }
                return Ok(Void);
            }
            Statement::BlockStatement { statements } => {
                trace!("Entering {} ", "BlockStatement");
                {
                    let mut ref_mut = self.envs.try_borrow_mut().unwrap();
                    let envs = ref_mut.deref_mut();
                    envs.push();
                }
                for statement in statements {
                    if matches!(**statement, Statement::ReturnStatement{..} ) {
                        match self.eval(statement) {
                            Ok(Void) => {}
                            Ok(Expr { res }) => { return Ok(Expr { res }); }
                            Err(_) => {}
                        };
                    } else {
                        self.eval(statement);
                    }
                }
                let mut ref_mut_post = self.envs.try_borrow_mut().unwrap();
                let envs_after = ref_mut_post.deref_mut();
                envs_after.pop();
                return Ok(Void);
            }
            Statement::VarDeclaration { identifier, expr } => {
                trace!("Entering {} ", "VarDeclaration");
                let identifier_res = self.expression_visitor.eval(*(*identifier).clone());
                let expression = *((*expr).clone().unwrap()).clone();
                let content = self.expression_visitor.eval(expression);
                if content.type_ == Identifier {
                    let mut ref_mut = self.envs.try_borrow_mut().unwrap();
                    let envs = ref_mut.deref_mut();
                    let value = envs.lookup_var(content.str);
                    envs.define_ref_at_top(identifier_res.str, value.clone())
                } else {
                    let mut ref_mut = self.envs.try_borrow_mut().unwrap();
                    let envs = ref_mut.deref_mut();
                    envs.define_at_top(identifier_res.str, ExpressionRes::copy(&content))
                };
                return Ok(Void);
            }
            Statement::ReturnStatement { expr } => {
                trace!("Entering {} ", "ReturnStatement");
                let option = (*expr).clone().unwrap();
                let res = self.expression_visitor.eval(*option);
                return Ok(Expr { res });
            }
        }
    }
}

pub enum StatementRes {
    Void,
    Expr {
        res: ExpressionRes
    },
}

impl StatementInterpreter {
    pub fn new_default() -> StatementInterpreter {
        let envs = Rc::new(RefCell::new(ProgramEnvs::new()));
        let expression_visitor = Rc::new(ExpressionInterpreter::new_with_envs(envs.clone()));
        StatementInterpreter {
            expression_visitor,
            envs,
            statements: vec![],
        }
    }

    pub fn new(expression_visitor: ExpressionInterpreter) -> StatementInterpreter {
        StatementInterpreter {
            expression_visitor: Rc::new(expression_visitor),
            envs: Rc::new(RefCell::new(ProgramEnvs::new())),
            statements: vec![],
        }
    }
    pub fn new_with_envs(envs: Rc<RefCell<ProgramEnvs>>) -> StatementInterpreter {
        StatementInterpreter {
            expression_visitor: Rc::new(ExpressionInterpreter::new_with_envs(envs.clone())),
            envs,
            statements: vec![],
        }
    }

    pub fn interpret(&mut self, program: Vec<Box<Statement>>) -> Result<StatementRes, String> {
        // self.statements = program;
        for statement in program {
            match self.eval(&*statement) {
                Ok(Void) => { continue; }
                Ok(Expr { res }) => { return Ok(Expr { res }); }
                Err(err) => { println!("{}", err) }
            };
        }
        Ok(Void)
    }

    pub fn lookup_variable(&self, name: String) -> Rc<RefCell<ExpressionRes>> {
        self.envs.borrow().lookup_var(name)
    }

    pub fn insert_variable(&self, name: String, expr: ExpressionRes) {
        let mut ref_mut = self.envs.try_borrow_mut().unwrap();
        let envs = ref_mut.deref_mut();
        envs.define_at_top(name, expr);
    }
}


#[test]
fn test_lookup() {
    let res = ExpressionRes::from_str(String::from("test"));
    let interpreter = StatementInterpreter::new_default();
    interpreter.insert_variable(String::from("test"), res);
    let rc = interpreter.lookup_variable(String::from("test"));
    assert_eq!(rc.borrow().str, "test");
}

#[test]
fn test_null() {
    let res = ExpressionRes::from_str(String::from("test"));
    let interpreter = StatementInterpreter::new_default();
    interpreter.insert_variable(String::from("test"), res);
    let rc = interpreter.lookup_variable(String::from("test"));
    assert_eq!(rc.borrow().str, "test");
}
