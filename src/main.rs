extern crate core;

use std::borrow::Borrow;
use crate::expressions::visitor::ExpressionInterpreter;
use crate::parser::Parser;
use crate::statements::statement::Statement;
use crate::statements::stmt_visitor::StatementInterpreter;
use crate::token::Scanner;

mod expressions;
mod log;
mod parser;
mod token;
#[cfg(test)]
mod token_test;
mod statements;
mod env;
mod program;

fn main() {
    let program = "print \"hello world\" ;\
     print 1 + 2 ;\
     print false + false;
     var x=1;
     EOF";

    let program = get_program(program);
    for statements in &program {
        println!("{:?}",statements)
    }
    let mut interpreter = StatementInterpreter::new(ExpressionInterpreter {});
    interpreter.interpret(program);
}

fn get_program(program: &str) -> Vec<Box<Statement>> {
    let vec = Scanner::new().tokenize_string(String::from(program));

    let mut parser = Parser::new(vec);
    let program = parser.program();
    program
}
