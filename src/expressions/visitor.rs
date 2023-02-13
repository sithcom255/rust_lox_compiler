use crate::expressions::expression::{Comparison, ExpressionRes, Equality, Expr, LiteralExpr, GroupingExpr, UnaryExpr, BinaryExpr, ExprResType, Expression};
use crate::token::{Token, TokenType};


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
pub struct ExpressionVisitor {}

impl ExpressionVisitor {
    fn execute_for_equality_(object: &Equality) {
        println!("Hello-world Equality {:?}", object.value);
    }
}

impl Visitor<ExpressionRes> for ExpressionVisitor {
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
        let expression = object.value.as_ref();
        expression.accept(Box::new(*self))
    }

    fn execute_for_binary(&self, object: &BinaryExpr) -> ExpressionRes {
        let rhs_res = object.rhs.as_ref().accept(Box::new(*self));
        let lhs_res = object.lhs.as_ref().accept(Box::new(*self));

        if lhs_res.type_ == ExprResType::Number && lhs_res.eq_type(&rhs_res) {
            match object.token.token_type {
                TokenType::Minus => ExpressionRes::from_number(
                    lhs_res.number - rhs_res.number),
                TokenType::Slash => ExpressionRes::from_number(
                    lhs_res.number / rhs_res.number),
                TokenType::Star => ExpressionRes::from_number(
                    lhs_res.number * rhs_res.number),
                TokenType::Plus => ExpressionRes::from_number(
                    lhs_res.number + rhs_res.number),
                _ => ExpressionRes::from_none()
            }
        } else if lhs_res.type_ == ExprResType::String && lhs_res.eq_type(&rhs_res) {
            match object.token.token_type {
                TokenType::Plus => ExpressionRes::from_str(
                    lhs_res.str + &*rhs_res.str),
                _ => ExpressionRes::from_none(),
            }
        } else {
            ExpressionRes::from_none()
        }
    }

    fn execute_for_unary(&self, object: &UnaryExpr) -> ExpressionRes {
        let rhs = object.rhs.as_ref();
        let rhs_res = rhs.accept(Box::new(*self));

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
}

#[test]
fn unary_bang() {
    let expr = UnaryExpr {
        token: Token {
            token_type: TokenType::Bang,
            value: "".to_string(),
            line: 0,
        },
        rhs: Box::new(LiteralExpr { token_type: TokenType::False, value: "".to_string() }),
    };
    let visitor = ExpressionVisitor {};
    assert!(expr.accept(Box::new(visitor)).boolean);
}

#[test]
fn string_binary_plus() {
    let expr = BinaryExpr {
        token: Token {
            token_type: TokenType::Plus,
            value: "".to_string(),
            line: 0,
        },
        lhs: Box::new(LiteralExpr { token_type: TokenType::String, value: "hello ".to_string() }),
        rhs: Box::new(LiteralExpr { token_type: TokenType::String, value: "world".to_string() }),
    };
    let visitor = ExpressionVisitor {};
    assert_eq!(expr.accept(Box::new(visitor)).str, "hello world");
}