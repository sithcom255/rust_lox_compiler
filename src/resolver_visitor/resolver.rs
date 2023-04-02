use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::env::environment::Environment;
use crate::expressions::expression::{Expression, ExpressionRes};
use crate::program::program::ProgramEnvs;
use crate::statements::statement::Statement;

pub struct CaptureResolver<'a> {
    captured: HashMap<String, Rc<RefCell<ExpressionRes>>>,
    parameters: &'a Vec<ExpressionRes>,
    local_vars: Vec<String>,
    env: Rc<RefCell<ProgramEnvs>>,
}

pub trait Resolve {
     fn resolve_statement(&mut self, statement: &Statement);
    fn resolve_expression(&mut self, expression: &Expression);
}

impl Resolve for CaptureResolver<'_> {
    fn resolve_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Stmt { expr } => {}
            Statement::IfStatement { expr, body, else_body } => {
                self.resolve_expression(expr);
                self.resolve_statement(body);
                match else_body {
                    None => {}
                    Some(ref val)=> {
                        self.resolve_statement(&*val)

                    }
                }
            }
            Statement::FunStatement { identifier, args, block } => {}
            Statement::WhileStatement { expr, body } => {
                self.resolve_expression(expr);
                self.resolve_statement(body);
            }
            Statement::ForStatement {
                initiation, condition,
                increment, body
            } => {
                self.resolve_statement(&*initiation.clone().unwrap());
                self.resolve_statement(&*condition.clone().unwrap());
                self.resolve_statement(&*body.clone());
            }
            Statement::PrintStatement { expr } => {
                self.resolve_expression(expr);
            }
            Statement::BlockStatement { statements } => {
                for statement in statements {
                    self.resolve_statement(statement)
                }
            }
            Statement::VarDeclaration { identifier, expr } => {
                if let Expression::VariableExpr{ token_type, ref value } = **identifier {
                    self.local_vars.push(value.clone().to_string())
                }
                self.resolve_expression(&*expr.clone().unwrap());
            }
            Statement::ReturnStatement { expr } => {
                self.resolve_expression(expr.as_ref().unwrap().deref());
            }
        }
    }

    fn resolve_expression(&mut self, expression: &Expression) {
        match expression {
            Expression::Expr { value, equality } => {
                self.resolve_expression(&*equality.clone().unwrap())
            }
            Expression::Equality { token, value } => {}
            Expression::Comparison { token_type, value } => {}
            Expression::GroupingExpr { value } => {
                self.resolve_expression(&*value.clone())
            }
            Expression::BinaryExpr { token, rhs, lhs } => {
                self.resolve_expression(&*rhs.clone());
                self.resolve_expression(&*lhs.clone());
            }
            Expression::UnaryExpr { token, rhs } => {
                self.resolve_expression(&*rhs.clone());
            }
            Expression::LiteralExpr { token_type, value } => {}
            Expression::VariableExpr { token_type, value } => {
                if value != "EOF" && !self.local_vars.contains(value) {
                    println!("{:#?}", &self.local_vars);
                    let mut resolve = true;
                    for parameter in self.parameters {
                        if parameter.str == value.to_string() {
                            resolve = false;
                        }
                    }
                    if resolve {
                        match self.env.borrow_mut().lookup_var(value.clone()) {
                           reference => {
                                self.captured.insert(value.clone(), reference);
                            }
                        }
                    }
                }
            }
            Expression::Assignment { identifier, value } => {
                if let Expression::VariableExpr{ token_type, ref value } = **identifier {
                    self.local_vars.push(value.clone().to_string())
                }
                self.resolve_expression(&*value.clone());
            }
            Expression::Logical { token, rhs, lhs } => {
                self.resolve_expression(&*rhs.clone());
                self.resolve_expression(&*lhs.clone());
            }
            Expression::Call { identifier, args } => {}
        }
    }
}

impl CaptureResolver<'_> {
    pub fn new_with_environment(env: Rc<RefCell<ProgramEnvs>>, parameters: &Vec<ExpressionRes>) ->  CaptureResolver  {
            CaptureResolver {
                captured: Default::default(),
                parameters,
                local_vars: vec![],
                env,
            }
    }

    pub fn get_captured(mut self) -> HashMap<String, Rc<RefCell<ExpressionRes>>> {
        let CaptureResolver { captured , ..} = self;
          return captured;
    }

}