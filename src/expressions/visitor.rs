use crate::expressions::expression::{Comparison, ExpressionRes, Equality, Expr, LiteralExpr, GroupingExpr, UnaryExpr, BinaryExpr};
use crate::token::TokenType;


pub trait Visitor<T> {
    fn execute_for_expr(&mut self, object: &Expr) -> T;
    fn execute_for_equality(&mut self, object: &Equality) -> T;
    fn execute_for_comparison(&self, object: &Comparison) -> T;
    fn execute_for_grouping(&self, object: &GroupingExpr) -> T;
    fn execute_for_binary(&self, object: &BinaryExpr) -> T;
    fn execute_for_unary(&self, object: &UnaryExpr) -> T;
    fn execute_for_literal(&self, object: &LiteralExpr) -> T;
}

#[derive(PartialEq, Copy, Clone)]
pub struct HelloWorldVisitor {}

impl HelloWorldVisitor {
    fn execute_for_equality_(object: &Equality) {
        println!("Hello-world Equality {:?}", object.value);
    }
}

impl Visitor<ExpressionRes> for HelloWorldVisitor {
    fn execute_for_expr(&mut self, object: &Expr) -> ExpressionRes {
        let expression = object.equality.as_ref().unwrap();
        expression.accept(Box::new(*self))
    }


    fn execute_for_equality(&mut self, object: &Equality) -> ExpressionRes {
        println!("Hello-world Equality {:?}", object.value);
        ExpressionRes::from_str(String::from(""))
    }

    fn execute_for_comparison(&self, object: &Comparison) -> ExpressionRes {
        println!("Hello-world Comparison {:?}", object.value);
        ExpressionRes::from_str(String::from(""))
    }

    fn execute_for_grouping(&self, object: &GroupingExpr) -> ExpressionRes {
        println!("Hello-world term {:?}", object.value);
        ExpressionRes::from_str(String::from(""))
    }

    fn execute_for_binary(&self, object: &BinaryExpr) -> ExpressionRes {
        let rhs = object.rhs.as_ref();
        let rhs_res = rhs.accept(Box::new(*self));
        let lhs = object.lhs.as_ref();
        let lhs_res = lhs.accept(Box::new(*self));

        match object.token.token_type {
            TokenType::Minus => ExpressionRes::from_number(
                lhs_res.number - rhs_res.number),
            TokenType::Slash => ExpressionRes::from_number(
                lhs_res.number / rhs_res.number),
            TokenType::Star => ExpressionRes::from_number(
                lhs_res.number * rhs_res.number),
            _ => ExpressionRes::from_none()
        }
    }

    fn execute_for_unary(&self, object: &UnaryExpr) -> ExpressionRes {
        println!("Hello-world unary {:?}", object);
        ExpressionRes::from_str(String::from(""))
    }

    fn execute_for_literal(&self, object: &LiteralExpr) -> ExpressionRes {
        let token_type = object.token_type.clone();
        let string = object.value.clone();
        match token_type {
            TokenType::String => ExpressionRes::from_str(string),
            TokenType::Number => ExpressionRes::from_number(str::parse::<isize>(&string).unwrap()),
            TokenType::False | TokenType::True => ExpressionRes::from_bool(str::parse::<bool>(&string).unwrap()),
            _ => ExpressionRes {
                token_type: TokenType::While,
                str: "".to_string(),
                number: 0,
                boolean: false,
            }
        }
    }
}