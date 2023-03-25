use std::collections::LinkedList;
use std::fmt::Debug;

use crate::expressions::expression::{Expression, ExpressionRes};
use crate::token::Token;

#[derive(Debug)]
pub enum Statement {
    Stmt {
         expr: Box<dyn Expression<ExpressionRes>>,
    },
    IfStatement {
        expr: Box<dyn Expression<ExpressionRes>>,
        body: Box<Statement>,
        else_body: Option<Box<Statement>>,
    },
    FunStatement {
        identifier: Token,
        args: Vec<Box<dyn Expression<ExpressionRes>>>,
        block: Option<Box<Statement>>,
    },
    WhileStatement {
        expr: Box<dyn Expression<ExpressionRes>>,
        body: Box<Statement>,
    },
    ForStatement {
        initiation: Option<Box<Statement>>,
        condition: Option<Box<Statement>>,
        increment: Option<Box<Statement>>,
        body: Box<Statement>
    },
    PrintStatement {
        expr: Box<dyn Expression<ExpressionRes>>,
    },
    BlockStatement {
        statements: LinkedList<Box<Statement>>,
    },
    VarDeclaration {
        identifier: Box<dyn Expression<ExpressionRes>>,
        expr: Option<Box<dyn Expression<ExpressionRes>>>,
    },
}