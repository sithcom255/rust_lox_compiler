use std::fmt::{Debug, Formatter};
use crate::expressions::visitor::{HelloWorldVisitor, Visitor};
use crate::token::{Token, TokenType};

pub trait Expression: Debug {
    fn accept(&self, visitor: Box<dyn Visitor>);
}


pub struct Expr {
    pub value: String,
    pub equality: Option<Box<dyn Expression>>,
}


impl Expression for Expr {
    fn accept(&self, visitor: Box<dyn Visitor>) {
        visitor.execute_for_expr(self);
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = "Expr";
        f.debug_struct(string)
           .field("equality", &self.equality).finish()
    }
}

pub struct Equality<'a> {
    pub token: &'a Token,
    pub value: String,

}

impl Expression for Equality<'_> {
    fn accept(&self, visitor: Box<dyn Visitor>) {
        visitor.execute_for_equality(self)
    }
}

impl Debug for Equality<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Equality")
            .field("token", &self.token)
            .finish()
    }
}

pub struct Comparison {
    pub token_type: TokenType,
    pub value: String,
}

pub struct GroupingExpr {
    pub value: Box<dyn Expression>
}

impl Debug for GroupingExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GroupingExpr")
            .field("value", &self.value)
            .finish()
    }
}

impl Expression for GroupingExpr {
    fn accept(&self, visitor: Box<dyn Visitor>) {
        visitor.execute_for_grouping(self);
    }
}

pub struct BinaryExpr {
    pub token:  Token,
    pub rhs: Box<dyn Expression>,
    pub lhs: Box<dyn Expression>,
}

impl Debug for BinaryExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinaryExpr")
            .field("token", &self.token)
            .field("rhs", &self.lhs)
            .field("rhs", &self.rhs)
            .finish()
    }
}

impl Expression for BinaryExpr {
    fn accept(&self, visitor: Box<dyn Visitor>) {
        visitor.execute_for_binary(self);
    }
}

pub struct UnaryExpr {
    pub token: Token,
    pub lhs: Box<dyn Expression>,
}

impl Debug for UnaryExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnaryExpr")
            .field("token", &self.token)
            .field("rhs", &self.lhs)
            .finish()
    }
}

impl Expression for UnaryExpr {
    fn accept(&self, visitor: Box<dyn Visitor>) {
        visitor.execute_for_unary(self);
    }
}

pub struct LiteralExpr {
    pub token: Token,
    pub value: String,
}

impl Expression for LiteralExpr {
    fn accept(&self, visitor: Box<dyn Visitor>) {
        visitor.execute_for_literal(self);
    }
}

impl Debug for LiteralExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LiteralExpr")
            .field("token", &self.token)
            .field("value", &self.value)
            .finish()
    }
}

#[test]
fn visitor_test() {
    let mut expr = Expr {
        value: String::from("here"),
        equality: None,
    };
    let visitor = HelloWorldVisitor {};
    let x = expr.accept(Box::new(visitor));
    println!("{:?}", expr)
}