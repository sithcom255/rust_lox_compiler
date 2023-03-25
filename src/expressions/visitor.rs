use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::rc::Rc;

use crate::expressions::expression::{Assignment, BinaryExpr, Call, Comparison, Equality, Expr, ExpressionRes, ExprResType, GroupingExpr, LiteralExpr, Logical, UnaryExpr, VariableExpr};
use crate::expressions::expression::ExprResType::{Function, Identifier, Nil};
use crate::program::program::ProgramEnvs;
use crate::token::TokenType;

pub trait Visitor<T> {
    fn execute_for_expr(&self, object: &Expr) -> T;
    fn execute_for_equality(&self, object: &Equality) -> T;
    fn execute_for_comparison(&self, object: &Comparison) -> T;
    fn execute_for_grouping(&self, object: &GroupingExpr) -> T;
    fn execute_for_binary(&self, object: &BinaryExpr) -> T;
    fn execute_for_unary(&self, object: &UnaryExpr) -> T;
    fn execute_for_literal(&self, object: &LiteralExpr) -> T;
    fn execute_for_variable(&self, object: &VariableExpr) -> T;
    fn execute_for_assignment(&self, object: &Assignment) -> T;
    fn execute_for_logical(&self, object: &Logical) -> T;
    fn execute_for_call(&self, object: &Call) -> T;
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
    fn execute_for_expr(&self, object: &Expr) -> ExpressionRes {
        let expression = object.equality.as_ref().unwrap();
        expression.accept(Rc::new(self))
    }


    fn execute_for_equality(&self, object: &Equality) -> ExpressionRes {
        println!("Hello-world Equality {:?}", object.value);
        ExpressionRes::from_str(String::from(""))
    }

    fn execute_for_comparison(&self, object: &Comparison) -> ExpressionRes {
        println!("Hello-world Comparison {:?}", object.value);
        ExpressionRes::from_str(String::from(""))
    }

    fn execute_for_grouping(&self, object: &GroupingExpr) -> ExpressionRes {
        let expression = object.value.as_ref();
        expression.accept(Rc::new(self))
    }

    fn execute_for_binary(&self, object: &BinaryExpr) -> ExpressionRes {
        let mut rhs_res = Rc::new(object.rhs.as_ref().accept(Rc::new(self)));
        let mut lhs_res = Rc::new(object.lhs.as_ref().accept(Rc::new(self)));

        if rhs_res.type_ == Identifier {
            let rc = self.envs.borrow().lookup_var(rhs_res.str.to_string());
            rhs_res = Rc::new(ExpressionRes::copy(rc.borrow().deref()));
        }

        if lhs_res.type_ == Identifier {
            let rc = self.envs.borrow().lookup_var(rhs_res.str.to_string());
            rhs_res = Rc::new(ExpressionRes::copy(rc.borrow().deref()));
        }


        if lhs_res.type_ == ExprResType::Number && lhs_res.eq_type(&rhs_res) {
            match object.token.token_type {
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
            match object.token.token_type {
                TokenType::Plus => ExpressionRes::from_str(
                    lhs_res.str.to_string() + &*rhs_res.str),
                _ => ExpressionRes::from_none(),
            }
        } else {
            println!("There has been an error in a binary operation");
            ExpressionRes::from_none()
        }
    }

    fn execute_for_unary(&self, object: &UnaryExpr) -> ExpressionRes {
        let rhs = object.rhs.as_ref();
        let rhs_res = rhs.accept(Rc::new(self));

        match (rhs_res.type_, object.token.token_type) {
            (ExprResType::Number, TokenType::Minus) => ExpressionRes::from_number(-(rhs_res.number)),
            (ExprResType::Boolean, TokenType::Bang) => ExpressionRes::from_bool(!(rhs_res.boolean)),
            _ => ExpressionRes::from_none()
        }
    }

    fn execute_for_literal(&self, object: &LiteralExpr) -> ExpressionRes {
        let token_type = object.token_type.clone();
        let string = object.value.clone();
        match token_type {
            TokenType::String => ExpressionRes::from_str(string),
            TokenType::Number => ExpressionRes::from_number(str::parse::<isize>(&string).unwrap()),
            TokenType::False => ExpressionRes::from_bool(false),
            TokenType::True => ExpressionRes::from_bool(true),
            _ => ExpressionRes::from_none()
        }
    }

    fn execute_for_variable(&self, object: &VariableExpr) -> ExpressionRes {
        match object.token_type {
            TokenType::Nil => ExpressionRes::from_variable(String::from("nil")),
            _ => ExpressionRes::from_variable(object.value.clone())
        }
    }

    fn execute_for_assignment(&self, object: &Assignment) -> ExpressionRes {
        let x = object.identifier.as_ref().accept(Rc::new(self));
        let value = object.value.as_ref().accept(Rc::new(self));
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
                self.envs.borrow().remove_var(x.str);
                ExpressionRes::from_none()
            }
            _ => {
                let res = ExpressionRes::copy(&value);
                self.envs.borrow().assign_to_existing(x.str.to_string(), value);
                res
            }
        }
    }
    fn execute_for_logical(&self, object: &Logical) -> ExpressionRes {
        let mut rhs_res = Rc::new(object.rhs.as_ref().accept(Rc::new(self)));
        let mut lhs_res = Rc::new(object.lhs.as_ref().accept(Rc::new(self)));

        if rhs_res.type_ == Identifier {
            let rc = self.envs.borrow().lookup_var(rhs_res.str.to_string());
            rhs_res = Rc::new(ExpressionRes::copy(rc.borrow().deref()));
        }

        if lhs_res.type_ == Identifier {
            let rc1 = self.envs.borrow().lookup_var(lhs_res.str.to_string());
            lhs_res = Rc::new(ExpressionRes::copy(rc1.borrow().deref()));
        }

        if lhs_res.type_ == ExprResType::Boolean && lhs_res.eq_type(&rhs_res) {
            if object.token.token_type == TokenType::And {
                ExpressionRes::from_bool(lhs_res.boolean && rhs_res.boolean)
            } else if object.token.token_type == TokenType::Or {
                ExpressionRes::from_bool(lhs_res.boolean || rhs_res.boolean)
            } else {
                panic!("cannot evaluate logical expression for {:#?} {:#?}", &lhs_res, &rhs_res)
            }
        } else {
            panic!("cannot evaluate logical expression for {:#?} {:#?}", &lhs_res, &rhs_res)
        }
    }

    fn execute_for_call(&self, object: &Call) -> ExpressionRes {
        let identifier = object.identifier.accept(Rc::new(self));
        let mut arguments = vec![];
        for arg in &object.args {
            let mut res = arg.accept(Rc::new(self));
            if (res.type_ == ExprResType::Identifier ) {

            }
            arguments.push(res)
        }
        let method = self.envs.borrow_mut().lookup_var(identifier.str.clone());


        ExpressionRes::from_none()
    }
}