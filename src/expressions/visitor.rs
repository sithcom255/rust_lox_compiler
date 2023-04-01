use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::rc::Rc;
use crate::env::environment::Environment;

use crate::expressions::expression::{Expression, ExpressionRes, ExprResType};
use crate::expressions::expression::ExprResType::{Function, Identifier, Nil};
use crate::program::program::ProgramEnvs;
use crate::statements::stmt_visitor::StatementInterpreter;
use crate::token::TokenType;

pub trait Visitor<T> {
    fn eval(&self, expression: Expression) -> T;
}

#[derive(Clone)]
pub struct ExpressionInterpreter {
    pub envs: Rc<RefCell<ProgramEnvs>>,
}

impl Debug for ExpressionInterpreter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExpressionInterpreter debug")
            .finish()
    }
}

impl ExpressionInterpreter {
    pub fn new() -> ExpressionInterpreter {
        ExpressionInterpreter {
            envs: Rc::new(RefCell::new(ProgramEnvs::new()))
        }
    }
    pub fn new_with_envs(envs: Rc<RefCell<ProgramEnvs>>) -> ExpressionInterpreter {
        ExpressionInterpreter {
            envs
        }
    }
}

impl Visitor<ExpressionRes> for ExpressionInterpreter {
    fn eval(& self, expression: Expression) -> ExpressionRes {
        match expression {
            Expression::Expr { value, equality } => {
                match equality {
                    None => { ExpressionRes::from_none() }
                    Some(value) => {
                        self.eval(*value)
                    }
                }
            }
            Expression::Equality { token, value } => {
                println!("Hello-world Equality {:?}", &value);
                ExpressionRes::from_str(String::from(""))
            }
            Expression::Comparison { token_type, value } => {
                println!("Hello-world Equality {:?}", &value);
                ExpressionRes::from_str(String::from(""))
            }
            Expression::GroupingExpr { value } => {
                self.eval(*value)
            }
            Expression::BinaryExpr { token, rhs, lhs } => {
                let mut rhs_res = self.eval(*rhs);
                let mut lhs_res = self.eval(*lhs);

                if rhs_res.type_ == Identifier {
                    let rc = self.envs.borrow().lookup_var(rhs_res.str.to_string());
                    rhs_res = ExpressionRes::copy(rc.borrow().deref());
                }

                if lhs_res.type_ == Identifier {
                    let rc = self.envs.borrow().lookup_var(lhs_res.str.to_string());
                    lhs_res = ExpressionRes::copy(rc.borrow().deref());
                }

                if lhs_res.type_ == ExprResType::Number && lhs_res.eq_type(&rhs_res) {
                    match token.token_type {
                        TokenType::Greater => ExpressionRes::from_bool(
                            lhs_res.number > rhs_res.number),
                        TokenType::GreaterEqual => ExpressionRes::from_bool(
                            lhs_res.number >= rhs_res.number),
                        TokenType::Less => ExpressionRes::from_bool(
                            lhs_res.number < rhs_res.number),
                        TokenType::LessEqual => ExpressionRes::from_bool(
                            lhs_res.number <= rhs_res.number),
                        TokenType::EqualEqual => ExpressionRes::from_bool(
                            lhs_res.number == rhs_res.number),
                        TokenType::Minus => ExpressionRes::from_number(
                            lhs_res.number - rhs_res.number),
                        TokenType::Slash => ExpressionRes::from_number(
                            lhs_res.number / rhs_res.number),
                        TokenType::Star => ExpressionRes::from_number(
                            lhs_res.number * rhs_res.number),
                        TokenType::Plus => ExpressionRes::from_number(
                            lhs_res.number + rhs_res.number),
                        TokenType::Percent => ExpressionRes::from_number(
                            (lhs_res.number).rem_euclid(rhs_res.number)
                        ),
                        _ => ExpressionRes::from_none()
                    }
                } else if lhs_res.type_ == ExprResType::String && lhs_res.eq_type(&rhs_res) {
                    match token.token_type {
                        TokenType::Plus => ExpressionRes::from_str(
                            lhs_res.str.to_string() + &*rhs_res.str),
                        TokenType::EqualEqual => ExpressionRes::from_bool(
                            lhs_res.str.to_string() == rhs_res.str.to_string()),
                        _ => ExpressionRes::from_none(),
                    }
                } else {
                    println!("There has been an error in a binary operation");
                    ExpressionRes::from_none()
                }
            }
            Expression::UnaryExpr { token, rhs } => {
                let rhs_res = self.eval(*rhs);
                match (rhs_res.type_, token.token_type) {
                    (ExprResType::Number, TokenType::Minus) => ExpressionRes::from_number(-(rhs_res.number)),
                    (ExprResType::Boolean, TokenType::Bang) => ExpressionRes::from_bool(!(rhs_res.boolean)),
                    _ => ExpressionRes::from_none()
                }
            }
            Expression::LiteralExpr { token_type, value } => {
                match token_type {
                    TokenType::String => ExpressionRes::from_str(value.clone()),
                    TokenType::Number => ExpressionRes::from_number(str::parse::<isize>(&value).unwrap()),
                    TokenType::False => ExpressionRes::from_bool(false),
                    TokenType::True => ExpressionRes::from_bool(true),
                    _ => ExpressionRes::from_none()
                }
            }
            Expression::VariableExpr { token_type, value } => {
                match token_type {
                    TokenType::Nil => ExpressionRes::from_variable(String::from("nil")),
                    _ => ExpressionRes::from_variable(value.clone())
                }
            }
            Expression::Assignment { identifier, value } => {
                let assignee = self.eval(*identifier);
                let value = self.eval(*value);
                match value.type_ {
                    Identifier => {
                        let rc = self.envs.borrow().lookup_var(value.str.clone());
                        rc.replace(ExpressionRes::copy(&value));
                        return ExpressionRes::copy(&value);
                    }
                    Function => {
                        value
                    }
                    Nil => {
                        self.envs.borrow().remove_var(assignee.str);
                        ExpressionRes::from_none()
                    }
                    _ => {
                        let res = ExpressionRes::copy(&value);
                        self.envs.borrow().assign_to_existing(assignee.str.to_string(), value);
                        res
                    }
                }
            }
            Expression::Logical { token, rhs, lhs } => {
                let mut rhs_res = self.eval(*rhs);
                let mut lhs_res = self.eval(*lhs);

                if rhs_res.type_ == Identifier {
                    let rc = self.envs.borrow().lookup_var(rhs_res.str.to_string());
                    rhs_res = ExpressionRes::copy(rc.borrow().deref());
                }

                if lhs_res.type_ == Identifier {
                    let rc1 = self.envs.borrow().lookup_var(lhs_res.str.to_string());
                    lhs_res = ExpressionRes::copy(rc1.borrow().deref());
                }

                if lhs_res.type_ == ExprResType::Boolean && lhs_res.eq_type(&rhs_res) {
                    match token.token_type {
                        TokenType::And => {
                            ExpressionRes::from_bool(lhs_res.boolean && rhs_res.boolean)
                        }
                        TokenType::Or => {
                            ExpressionRes::from_bool(lhs_res.boolean || rhs_res.boolean)
                        }
                        _ => {
                            panic!("cannot evaluate logical expression for {:#?} {:#?}", &lhs_res, &rhs_res)
                        }
                    }
                } else {
                    panic!("cannot evaluate logical expression for {:#?} {:#?}", &lhs_res, &rhs_res)
                }
            }
            Expression::Call { identifier, args } => {
                let method_name = self.eval(*identifier);
                let mut arguments = Environment::new();
                {

                    let mut ref_mut = self.envs.borrow_mut();
                    ref_mut.push();
                }
                let method = self.envs.borrow().lookup_var(method_name.str.clone());

                let argument_names = method.as_ref().borrow().get_params();
                if argument_names.len() != args.len() {
                    println!("{{ wow, so weird, this look like passed args are not same as declared definition }}")
                }
                let mut i: usize = 0;
                for arg in args {

                    let mut res = self.eval(*arg);
                    if res.type_ == ExprResType::Identifier {
                        let rc2 = self.envs.borrow_mut().lookup_var(res.str.clone());
                        res = ExpressionRes::copy(rc2.borrow_mut().deref());
                    }

                    self.envs.borrow_mut().define_at_top(argument_names[i].clone(), res);
                    i = i + 1;
                }

                &method.as_ref().borrow().get_method().call(StatementInterpreter::new_default(),
                self.envs.clone());
                self.envs.borrow_mut().pop();

                // om
                ExpressionRes::from_none()
            }
        }
    }
}