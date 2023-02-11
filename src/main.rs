use crate::token::Scanner;

mod expressions;
mod log;
mod parser;
mod token;

fn main() {
    let mut scanner = Scanner::new();
    scanner.tokenize_file(&"helloo");
}
