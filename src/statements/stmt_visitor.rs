use crate::env::environment::Environment;
use crate::expressions::expression::ExprResType::Variable;
use crate::expressions::visitor::{ExpressionInterpreter};
use crate::statements::statement::{PrintStatement, Statement, Stmt, VarDeclaration};
use crate::token::TokenType;


pub trait StmtVisitor {
    fn execute_statement(&self, object: &Stmt);
    fn execute_print_statement(&self, object: &PrintStatement);
    fn execute_var_statement(&mut self, object: &VarDeclaration);
}

pub struct StatementInterpreter {
    pub expression_visitor: ExpressionInterpreter,
    pub environment: Environment,
}

impl StmtVisitor for StatementInterpreter {
    fn execute_statement(&self, object: &Stmt) {
        object.expr.accept(Box::new(&self.expression_visitor));
    }

    fn execute_print_statement(&self, object: &PrintStatement) {
        let res = object.expr.accept(Box::new(&self.expression_visitor));
        println!("{}", res.print())
    }

    fn execute_var_statement(&mut self, object: &VarDeclaration) {
        let identifier_res = object.identifier.as_ref().accept(Box::new(&self.expression_visitor));
        if identifier_res.type_ == Variable {
            let res = object.expr.as_ref().unwrap().accept(Box::new(&self.expression_visitor));
            self.environment.define_variable(identifier_res.str, res);
        };
        println!("{:?}", self.environment.get_variable("hello"))
    }
}

impl StatementInterpreter {
    pub fn new(expression_visitor: ExpressionInterpreter) -> StatementInterpreter {
        StatementInterpreter { expression_visitor, environment : Environment::new() }
    }

    pub fn interpret(&self, program: Vec<Box<dyn Statement>>) {
        for statement in program {
            statement.accept(Box::new(self))
        }
    }
}
