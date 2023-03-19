use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::os::linux::raw::stat;
use std::rc::Rc;

use crate::expressions::expression::ExpressionRes;
use crate::expressions::expression::ExprResType::{Boolean, Variable};
use crate::expressions::visitor::{ExpressionInterpreter, Visitor};
use crate::program::program::ProgramEnvs;
use crate::statements::statement::Statement;

pub trait StmtVisitor {
    fn eval(&mut self, object: &Statement);
}

pub struct StatementInterpreter {
    pub expression_visitor: Rc<dyn Visitor<ExpressionRes>>,
    pub envs: Rc<RefCell<ProgramEnvs>>,
}

impl StmtVisitor for StatementInterpreter {
    fn eval(&mut self, object: &Statement) {
        match object {
            Statement::Stmt { expr } => {
                expr.accept(Rc::new(self.expression_visitor.as_ref()));
            }
            Statement::IfStatement { expr, body, else_body } => {
                let res = expr.accept(Rc::new(self.expression_visitor.as_ref()));
                if res.type_ != Boolean {
                    panic!("if (.expr.) not evaluatable to bool")
                }
                if res.boolean {
                    self.eval(body);
                } else {
                    match else_body.as_ref() {
                        None => { }
                        Some(value) => { self.eval(value)}
                    };

                }
            }
            Statement::WhileStatement { expr, body: statements } => {
                let condition = expr.clone().deref();
                let mut res1 = condition.accept(Rc::new(self.expression_visitor.as_ref()));

                while res1.boolean {
                    self.eval(statements);
                    res1 = condition.accept(Rc::new(self.expression_visitor.as_ref()))
                }
            }
            Statement::ForStatement { initiation, condition, increment, body } => {

            }
            Statement::PrintStatement { expr } => {
                let res = expr.accept(Rc::new(self.expression_visitor.as_ref()));
                if res.type_ == Variable {
                    let x = self.lookup_variable(res.str);
                    println!("{}", x.print())
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
                let identifier_res = identifier.as_ref().accept(Rc::new(self.expression_visitor.as_ref()));
                let content = expr.as_ref().unwrap().accept(Rc::new(self.expression_visitor.as_ref()));
                if identifier_res.type_ == Variable {
                    let mut ref_mut = self.envs.try_borrow_mut().unwrap();
                    let envs = ref_mut.deref_mut();
                    envs.define_at_top(identifier_res.str, content);
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
        }
    }

    pub fn new(expression_visitor: ExpressionInterpreter) -> StatementInterpreter {
        StatementInterpreter {
            expression_visitor: Rc::new(expression_visitor),
            envs: Rc::new(RefCell::new(ProgramEnvs::new())),
        }
    }

    pub fn interpret(&mut self, program: Vec<Box<Statement>>) {
        for statement in program {
            self.eval(&*statement)
        }
    }

    pub fn lookup_variable(&self, name: String) -> Rc<ExpressionRes> {
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
    assert_eq!(rc.str, "test")
}

#[test]
fn test_null() {
    let res = ExpressionRes::from_str(String::from("test"));
    let interpreter = StatementInterpreter::new_default();
    interpreter.insert_variable(String::from("test"), res);
    let rc = interpreter.lookup_variable(String::from("test"));
    assert_eq!(rc.str, "test")
}
