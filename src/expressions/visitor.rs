use crate::expressions::expression::{Comparison, Equality, Expr, LiteralExpr, GroupingExpr, UnaryExpr, BinaryExpr};

pub trait Visitor {
    fn execute_for_expr(&self, object: &Expr);
    fn execute_for_equality(&self, object: &Equality);
    fn execute_for_comparison(&self, object: &Comparison);
    fn execute_for_grouping(&self, object: &GroupingExpr);
    fn execute_for_binary(&self, object: &BinaryExpr);
    fn execute_for_unary(&self, object: &UnaryExpr);
    fn execute_for_literal(&self, object: &LiteralExpr);
}

pub struct HelloWorldVisitor {}

impl Visitor for HelloWorldVisitor {

    fn execute_for_expr(&self, object: &Expr) {
        println!("Hello-world Expr {:?}", object.value);
    }

    fn execute_for_equality(&self, object: &Equality) {
        println!("Hello-world Equality {:?}", object.value);
    }

    fn execute_for_comparison(&self, object: &Comparison) {
        println!("Hello-world Comparison {:?}", object.value);
    }

    fn execute_for_grouping(&self, object: &GroupingExpr) {
        println!("Hello-world term {:?}", object.value);
    }

    fn execute_for_binary(&self, object: &BinaryExpr) {
        println!("Hello-world unary {:?}", object);
    }

    fn execute_for_unary(&self, object: &UnaryExpr) {
        println!("Hello-world unary {:?}", object);
    }

    fn execute_for_literal(&self, object: &LiteralExpr) {
        println!("Hello-world primary {:?}", object.value);
    }
}