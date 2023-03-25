use std::fmt::{Debug, Formatter};
use std::ops::Add;
use std::rc::Rc;

use crate::expressions::visitor::{ExpressionInterpreter, Visitor};
use crate::program::runtime::Method;
use crate::token::{Token, TokenType};

pub enum Expression {
    Expr {
        value: String,
        equality: Option<Box<Expression>>,
    },
    Equality {
        token: Token,
        value: String,

    },
    Comparison {
        token_type: TokenType,
        value: String,
    },
    GroupingExpr {
        value: Box<Expression>,
    },
    BinaryExpr {
        token: Token,
        rhs: Box<Expression>,
        lhs: Box<Expression>,
    },
}


pub trait Expression<T>: Debug {
    fn accept(&self, visitor: Rc<&dyn Visitor<T>>) -> T;
}


pub struct Expr {
    pub value: String,
    pub equality: Option<Box<dyn Expression<ExpressionRes>>>,
}


impl Expression<ExpressionRes> for Expr {
    fn accept(&self, visitor: Rc<&dyn Visitor<ExpressionRes>>) -> ExpressionRes {
        visitor.execute_for_expr(self)
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Expr")
            .field("equality", &self.equality).finish()
    }
}

pub struct Equality {
    pub token: Token,
    pub value: String,

}

impl Expression<ExpressionRes> for Equality {
    fn accept(&self, visitor: Rc<&dyn Visitor<ExpressionRes>>) -> ExpressionRes {
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
    fn accept(&self, visitor: Rc<&dyn Visitor<ExpressionRes>>) -> ExpressionRes {
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
    fn accept(&self, visitor: Rc<&dyn Visitor<ExpressionRes>>) -> ExpressionRes {
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
    fn accept(&self, visitor: Rc<&dyn Visitor<ExpressionRes>>) -> ExpressionRes {
        visitor.execute_for_unary(self)
    }
}

pub struct LiteralExpr {
    pub token_type: TokenType,
    pub value: String,
}

impl Expression<ExpressionRes> for LiteralExpr {
    fn accept(&self, visitor: Rc<&dyn Visitor<ExpressionRes>>) -> ExpressionRes {
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

pub struct VariableExpr {
    pub token_type: TokenType,
    pub value: String,
}

impl Expression<ExpressionRes> for VariableExpr {
    fn accept(&self, visitor: Rc<&dyn Visitor<ExpressionRes>>) -> ExpressionRes {
        visitor.execute_for_variable(self)
    }
}

impl Debug for VariableExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VariableExpr")
            .field("token", &self.token_type)
            .field("value", &self.value)
            .finish()
    }
}

pub struct Assignment {
    pub identifier: Box<dyn Expression<ExpressionRes>>,
    pub value: Box<dyn Expression<ExpressionRes>>,
}

impl Expression<ExpressionRes> for Assignment {
    fn accept(&self, visitor: Rc<&dyn Visitor<ExpressionRes>>) -> ExpressionRes {
        visitor.execute_for_assignment(self)
    }
}

impl Debug for Assignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Assignment")
            .field("identifier", &self.identifier)
            .field("value", &self.value)
            .finish()
    }
}

pub struct Logical {
    pub token: Token,
    pub rhs: Box<dyn Expression<ExpressionRes>>,
    pub lhs: Box<dyn Expression<ExpressionRes>>,
}

impl Debug for Logical {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinaryExpr")
            .field("token", &self.token)
            .field("lhs", &self.lhs)
            .field("rhs", &self.rhs)
            .finish()
    }
}

impl Expression<ExpressionRes> for Logical {
    fn accept(&self, visitor: Rc<&dyn Visitor<ExpressionRes>>) -> ExpressionRes {
        visitor.execute_for_logical(self)
    }
}


pub struct Call {
    pub identifier: Box<dyn Expression<ExpressionRes>>,
    pub args: Vec<Box<dyn Expression<ExpressionRes>>>,
}

impl Debug for Call {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Call")
            .field("identifier", &self.identifier)
            .field("args", &self.args)
            .finish()
    }
}

impl Expression<ExpressionRes> for Call {
    fn accept(&self, visitor: Rc<&dyn Visitor<ExpressionRes>>) -> ExpressionRes {
        visitor.execute_for_call(self)
    }
}


#[derive(Debug)]
pub struct ExpressionRes {
    pub type_: ExprResType,
    pub str: String,
    pub number: isize,
    pub boolean: bool,
    pub method: Option<Method>,
}

impl ExpressionRes {
    pub fn copy(p: &ExpressionRes) -> ExpressionRes {
        ExpressionRes {
            type_: p.type_.clone(),
            str: p.str.clone(),
            number: p.number.clone(),
            boolean: p.boolean.clone(),
            method: None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ExprResType {
    String,
    Number,
    Boolean,
    Identifier,
    Function,
    Nil,
}

impl ExpressionRes {
    pub fn from_str(str: String) -> ExpressionRes {
        ExpressionRes {
            type_: ExprResType::String,
            str,
            number: 0,
            boolean: false,
            method: None,
        }
    }

    pub fn from_number(number: isize) -> ExpressionRes {
        ExpressionRes {
            type_: ExprResType::Number,
            str: String::new(),
            number,
            boolean: false,
            method: None,
        }
    }

    pub fn from_bool(boolean: bool) -> ExpressionRes {
        ExpressionRes {
            type_: ExprResType::Boolean,
            str: String::new(),
            number: 0,
            boolean,
            method: None,
        }
    }

    pub fn from_variable(str: String) -> ExpressionRes {
        ExpressionRes {
            type_: ExprResType::Identifier,
            str,
            number: 0,
            boolean: false,
            method: None,
        }
    }

    pub fn from_method(method: Method) -> ExpressionRes {
        ExpressionRes {
            type_: ExprResType::Function,
            str: "fun".to_string().add(&method.name.clone()),
            number: 0,
            boolean: false,
            method: Some(method),
        }
    }

    pub fn from_none() -> ExpressionRes {
        ExpressionRes {
            type_: ExprResType::Nil,
            str: "nil".to_string(),
            number: 0,
            boolean: false,
            method: None,
        }
    }

    pub fn eq_type(&self, other: &ExpressionRes) -> bool {
        self.type_ == other.type_
    }

    pub fn print(&self) -> String {
        match self.type_ {
            ExprResType::String => self.str.clone(),
            ExprResType::Number => self.number.to_string(),
            ExprResType::Boolean => if self.boolean { String::from("true") } else { String::from("false") },
            ExprResType::Nil => String::from("nil"),
            ExprResType::Identifier => self.str.clone(),
            ExprResType::Function => { self.str.clone() }
        }
    }
}