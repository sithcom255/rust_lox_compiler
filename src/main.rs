use crate::expressions::visitor::ExpressionVisitor;
use crate::parser::Parser;
use crate::token::Scanner;

mod expressions;
mod log;
mod parser;
mod token;

fn main() {
    let mut scanner = Scanner::new();
    let vec = scanner.tokenize_string(String::from(" \"hello \" + \"world\""));
    let mut parser = Parser::new(vec);
    let option = parser.expression();
    let x = ExpressionVisitor {};
    let res = option.unwrap().accept(Box::new(x));
    println!("{:?}", res)
}
