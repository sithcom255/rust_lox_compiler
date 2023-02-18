use crate::expressions::visitor::{ExpressionVisitor};
use crate::statements::statement::{PrintStatement, Stmt};


pub trait StmtVisitor {
    fn execute_for_statement(&self, object: &Stmt);
    fn execute_print_statement(&self, object: &PrintStatement);
}

pub struct StatementVisitor {
    pub expression_visitor: ExpressionVisitor,
}

impl StatementVisitor {
    pub(crate) fn new(expression_visitor: ExpressionVisitor) -> StatementVisitor {
        StatementVisitor { expression_visitor }
    }
}

impl StmtVisitor for StatementVisitor {
    fn execute_for_statement(&self, object: &Stmt) {
       object.expr.accept(Box::new(self.expression_visitor));
    }

    fn execute_print_statement(&self, object: &PrintStatement) {
        let res = object.expr.accept(Box::new(self.expression_visitor));
        println!("{}", res.print())
    }
}