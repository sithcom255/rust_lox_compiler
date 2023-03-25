use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::os::linux::raw::stat;
use std::rc::Rc;

use crate::expressions::expression::ExpressionRes;
use crate::expressions::expression::ExprResType::{Boolean, Identifier};
use crate::expressions::visitor::{ExpressionInterpreter, Visitor};
use crate::program::program::ProgramEnvs;
use crate::program::runtime::Method;
use crate::statements::statement::Statement;

pub trait StmtVisitor {
    fn eval(&mut self, object: &Statement);
}

pub struct StatementInterpreter {
    pub expression_visitor: Rc<dyn Visitor<ExpressionRes>>,
    pub envs: Rc<RefCell<ProgramEnvs>>,
    pub statements: Vec<Box<Statement>>,
}

impl StmtVisitor for StatementInterpreter {
    fn eval(&mut self, object: &Statement) {
        match object {
            Statement::Stmt { expr } => {
                self.expression_visitor.eval((**expr).clone());
            }
            Statement::IfStatement { expr, body, else_body } => {
                let res = self.expression_visitor.eval((*expr).clone());
                if res.type_ != Boolean {
                    panic!("if (.expr.) not evaluatable to bool")
                }
                if res.boolean {
                    self.eval(body);
                } else {
                    match else_body.as_ref() {
                        None => {}
                        Some(value) => { self.eval(value) }
                    };
                }
            }
            Statement::FunStatement { identifier, args, block } => {
                let mut arguments = vec![];
                for arg in args {
                    arguments.push( self.expression_visitor.eval((*arg).clone()));
                }
                let body1 = (**block.as_ref().unwrap()).clone();
                let method = ExpressionRes::from_method(Method::new(identifier.value.clone(),
                                                                    arguments,
                                                                    body1));

                self.envs.try_borrow_mut().unwrap().define_at_top(identifier.value.clone(),
                                                                  method);
            }
            Statement::WhileStatement { expr, body: statements } => {
                let condition = expr.clone();
                let mut res1 =  self.expression_visitor.eval((*condition).clone());

                while res1.boolean {
                    self.eval(statements);
                    res1 = self.expression_visitor.eval((*condition).clone());
                }
            }
            Statement::ForStatement { initiation, condition, increment, body } => {
                match initiation {
                    None => {}
                    Some(value) => { self.eval(value) }
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
                    self.eval(body);
                    match increment {
                        None => {}
                        Some(value) => { self.eval(value) }
                    }
                    match condition {
                        None => { res1 = ExpressionRes::from_bool(false) }
                        Some(value) => {
                            let statement = value.clone();
                            match *statement {
                                Statement::Stmt { expr } => {
                                    res1 =  self.expression_visitor.eval(*expr);
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            Statement::PrintStatement { expr } => {
                let x1 = (*expr).clone();
                let res =  self.expression_visitor.eval(*x1);

                if res.type_ == Identifier {
                    let x = self.lookup_variable(res.str);
                    println!("{}", x.borrow().print())
                } else {
                    println!("{}", res.print())
                }
            }
            Statement::BlockStatement { statements } => {
                {
                    let mut ref_mut = self.envs.try_borrow_mut().unwrap();
                    let envs = ref_mut.deref_mut();
                    envs.push();
                }
                for statement in statements {
                    self.eval(statement);
                }
                let mut ref_mut_post = self.envs.try_borrow_mut().unwrap();
                let envs_after = ref_mut_post.deref_mut();
                envs_after.pop();
            }
            Statement::VarDeclaration { identifier, expr } => {
                let identifier_res =  self.expression_visitor.eval(*(*identifier).clone());
                let expression = *((*expr).clone().unwrap()).clone();
                let content =  self.expression_visitor.eval(expression);
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
            }
        }
    }
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

    pub fn interpret(&mut self, program: Vec<Box<Statement>>) {
        // self.statements = program;
        for statement in program {
            self.eval(&*statement)
        }
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
