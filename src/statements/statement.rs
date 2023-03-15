use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::expressions::expression::{Expression, ExpressionRes};
use crate::statements::stmt_visitor::StmtVisitor;

// pub trait Statement: Debug {
//     fn accept(&self, visitor: Rc<RefCell<&dyn StmtVisitor>>);
// }

#[derive(Debug)]
pub enum Statement {
    Stmt {
        expr: Box<dyn Expression<ExpressionRes>>,
    },
    PrintStatement {
        expr: Box<dyn Expression<ExpressionRes>>,
    },
    VarDeclaration {
        identifier: Box<dyn Expression<ExpressionRes>>,
        expr: Option<Box<dyn Expression<ExpressionRes>>>,
    },
}