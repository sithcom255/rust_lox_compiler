use crate::expressions::visitor::{ExpressionInterpreter};
use crate::statements::statement::{PrintStatement, Stmt, VarDeclaration};


pub trait StmtVisitor {
    fn execute_statement(&self, object: &Stmt);
    fn execute_print_statement(&self, object: &PrintStatement);
    fn execute_var_statement(&self, object: &VarDeclaration);
}

pub struct StatementInterpreter {
    pub expression_visitor: ExpressionInterpreter,
}

impl StatementInterpreter {
    pub fn new(expression_visitor: ExpressionInterpreter) -> StatementInterpreter {
        StatementInterpreter { expression_visitor }
    }
}

impl StmtVisitor for StatementInterpreter {
    fn execute_statement(&self, object: &Stmt) {
       object.expr.accept(Box::new(self.expression_visitor));
    }

    fn execute_print_statement(&self, object: &PrintStatement) {
        let res = object.expr.accept(Box::new(self.expression_visitor));
        println!("{}", res.print())
    }

    fn execute_var_statement(&self, object: &VarDeclaration) {
        let res = object.expr.accept(Box::new(self.expression_visitor));
    }
}