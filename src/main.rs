use crate::expressions::visitor::ExpressionVisitor;
use crate::parser::Parser;
use crate::statements::statement::Statement;
use crate::statements::stmt_visitor::StatementVisitor;
use crate::token::Scanner;

mod expressions;
mod log;
mod parser;
mod token;
#[cfg(test)]
mod token_test;
mod statements;

fn main() {
    let program = "print \"hello world\" ;\
     print 1 + 2 ;\
     print false + false
     EOF";

    let program = get_program(program);

    for statement in program {
        statement.accept(Box::new(StatementVisitor::new(ExpressionVisitor {})))
    }
}

fn get_program(program: &str) -> Vec<Box<dyn Statement>> {
    let vec = Scanner::new().tokenize_string(String::from(program));

    let mut parser = Parser::new(vec);
    let program = parser.program();
    program
}
