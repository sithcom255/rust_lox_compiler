use std::collections::LinkedList;
use std::fmt::Debug;

use crate::expressions::expression::{Expression, ExpressionRes};
use crate::token::Token;

#[derive(Debug,  Clone)]
pub enum Statement {
    Stmt {
         expr: Box<Expression>,
    },
    IfStatement {
        expr: Expression,
        body: Box<Statement>,
        else_body: Option<Box<Statement>>,
    },
    FunStatement {
        identifier: Token,
        args: Vec<Expression>,
        block: Option<Box<Statement>>,
    },
    WhileStatement {
        expr: Box<Expression>,
        body: Box<Statement>,
    },
    ForStatement {
        initiation: Option<Box<Statement>>,
        condition: Option<Box<Statement>>,
        increment: Option<Box<Statement>>,
        body: Box<Statement>
    },
    PrintStatement {
        expr: Box<Expression>,
    },
    BlockStatement {
        statements: LinkedList<Box<Statement>>,
    },
    VarDeclaration {
        identifier: Box<Expression>,
        expr: Option<Box<Expression>>,
    },
    ReturnStatement {
        expr: Option<Box<Expression>>,
    },
}