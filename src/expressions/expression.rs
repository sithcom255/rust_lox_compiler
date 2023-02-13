use std::fmt::{Debug, Formatter};
use crate::expressions::visitor::{ExpressionVisitor, Visitor};
use crate::token::{Token, TokenType};

pub trait Expression<T>: Debug {
    fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T;
}


pub struct Expr {
    pub value: String,
    pub equality: Option<Box<dyn Expression<ExpressionRes>>>,
}


impl Expression<ExpressionRes> for Expr {
    fn accept(&self, mut visitor: Box<dyn Visitor<ExpressionRes>>) -> ExpressionRes {
        visitor.execute_for_expr(self)
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = "Expr";
        f.debug_struct(string)
            .field("equality", &self.equality).finish()
    }
}

pub struct Equality {
    pub token: Token,
    pub value: String,

}

impl Expression<ExpressionRes> for Equality {
    fn accept(&self, mut visitor: Box<dyn Visitor<ExpressionRes>>) -> ExpressionRes {
        visitor.execute_for_equality(self)
    }
}

impl Debug for Equality {
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
    pub value: Box<dyn Expression<ExpressionRes>>,
}

impl Debug for GroupingExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GroupingExpr")
            .field("value", &self.value)
            .finish()
    }
}

impl Expression<ExpressionRes> for GroupingExpr {
    fn accept(&self, visitor: Box<dyn Visitor<ExpressionRes>>) -> ExpressionRes {
        visitor.execute_for_grouping(self)
    }
}

pub struct BinaryExpr {
    pub token: Token,
    pub rhs: Box<dyn Expression<ExpressionRes>>,
    pub lhs: Box<dyn Expression<ExpressionRes>>,
}

impl Debug for BinaryExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinaryExpr")
            .field("token", &self.token)
            .field("lhs", &self.lhs)
            .field("rhs", &self.rhs)
            .finish()
    }
}

impl Expression<ExpressionRes> for BinaryExpr {
    fn accept(&self, visitor: Box<dyn Visitor<ExpressionRes>>) -> ExpressionRes {
        visitor.execute_for_binary(self)
    }
}

pub struct UnaryExpr {
    pub token: Token,
    pub rhs: Box<dyn Expression<ExpressionRes>>,
}

impl Debug for UnaryExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnaryExpr")
            .field("token", &self.token)
            .field("rhs", &self.rhs)
            .finish()
    }
}

impl Expression<ExpressionRes> for UnaryExpr {
    fn accept(&self, visitor: Box<dyn Visitor<ExpressionRes>>) -> ExpressionRes {
        visitor.execute_for_unary(self)
    }
}

pub struct LiteralExpr {
    pub token_type: TokenType,
    pub value: String,
}

impl Expression<ExpressionRes> for LiteralExpr {
    fn accept(&self, visitor: Box<dyn Visitor<ExpressionRes>>) -> ExpressionRes {
        visitor.execute_for_literal(self)
    }
}

impl Debug for LiteralExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LiteralExpr")
            .field("token", &self.token_type)
            .field("value", &self.value)
            .finish()
    }
}

#[derive(Debug)]
pub struct ExpressionRes {
    pub type_: ExprResType,
    pub str: String,
    pub number: isize,
    pub boolean: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ExprResType {
    String, Number, Boolean, Nil
}

impl ExpressionRes {
    pub fn from_str(str: String) -> ExpressionRes {
        ExpressionRes {
            type_: ExprResType::String,
            str,
            number: 0,
            boolean: false,
        }
    }

    pub fn from_number(number: isize) -> ExpressionRes {
        ExpressionRes {
            type_: ExprResType::Number,
            str: String::new(),
            number,
            boolean: false,
        }
    }

    pub fn from_bool(boolean: bool) -> ExpressionRes {
        ExpressionRes {
            type_: ExprResType::Boolean,
            str: String::new(),
            number: 0,
            boolean,
        }
    }

    pub fn from_none() -> ExpressionRes {
        ExpressionRes {
            type_: ExprResType::Nil,
            str: "".to_string(),
            number: 0,
            boolean: false,
        }
    }

    pub fn eq_type(&self, other: &ExpressionRes) -> bool {
        return self.type_ == other.type_;
    }
}

#[test]
fn visitor_test() {
    let token = Token {
        token_type: TokenType::Minus,
        value: "".to_string(),
        line: 0,
    };
    let equality = BinaryExpr {
        token,
        rhs: Box::new(LiteralExpr {
            token_type: TokenType::Number,
            value: "10".to_string(),
        }),
        lhs: Box::new(LiteralExpr {
            token_type: TokenType::Number,
            value: "1".to_string(),
        }),
    };
    let expr = Expr {
        value: String::from("here"),
        equality: Some(Box::new(equality)),
    };
    let visitor = ExpressionVisitor {};
    let res = expr.accept(Box::new(visitor));
    println!("{:?}", res)
}