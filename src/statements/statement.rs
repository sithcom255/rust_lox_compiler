use std::fmt::{Debug, Formatter};
use crate::expressions::expression::{Expression, ExpressionRes};
use crate::statements::stmt_visitor::StmtVisitor;

pub trait Statement: Debug {
    fn accept(&self, visitor: Box<dyn StmtVisitor>);
}

// statements are supposed to capture side effects, what kind of side effects are there?
// mutating state of block, mutation of object, print statement, assignement, return statement
// function definition, function invocation
pub struct Stmt {
    pub expr: Box<dyn Expression<ExpressionRes>>,
}

impl Stmt {
    pub fn new(expr: Box<dyn Expression<ExpressionRes>> ) -> Stmt {
        Stmt { expr }
    }
}

impl Debug for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stmt").finish()
    }
}

impl Statement for Stmt {
    fn accept(&self, visitor: Box<dyn StmtVisitor>) {
        visitor.execute_statement(self)
    }
}

pub struct PrintStatement {
    pub expr: Box<dyn Expression<ExpressionRes>>,
}

impl PrintStatement {
    pub fn new(expr: Box<dyn Expression<ExpressionRes>> ) -> PrintStatement {
        PrintStatement { expr }
    }
}

impl Debug for PrintStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrintStatement")
            .field("expression",&self.expr ).finish()
    }
}

impl Statement for PrintStatement {
    fn accept(&self, visitor: Box<dyn StmtVisitor>) {
        visitor.execute_print_statement(self)
    }
}

pub struct VarDeclaration {
    pub expr: Box<dyn Expression<ExpressionRes>>,
    pub identifier: String,
}

impl Debug for VarDeclaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VarStatement")
            .field("identifier", &self.identifier)
            .field("expression",&self.expr ).finish()
    }
}

impl Statement for VarDeclaration {
    fn accept(&self, visitor: Box<dyn StmtVisitor>) {
        visitor.execute_var_statement(self)
    }
}