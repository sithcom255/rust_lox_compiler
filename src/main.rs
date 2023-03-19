#![allow(warnings, unused)]
extern crate core;

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
#[cfg(test)]
mod parser_tests;


fn main() {
    let program = get_statement(get_for_loop());
    // debug(&program);
    let mut interpreter = StatementInterpreter::new_default();
    interpreter.interpret(program);
}

fn debug(p: &Vec<Box<Statement>>) {
    for statement in p {
        println!("{:#?}", statement)
    }
}

fn get_statement(program: String) -> Vec<Box<Statement>> {
    let vec = Scanner::new().tokenize_string(String::from(program));

    let mut parser = Parser::new(vec);
    let program = parser.program();
    program
}

fn multiple_assigments() -> String {
    "var x;
     var y = x = 2;
     print y;
     EOF;".to_string()
}

fn get_scoped() -> String {
    "var x = 1;
     var y = 2;
     var z;
     {
        var y = 3;
        z = x + y;
        print z;
     }
     print z;
     EOF;".to_string()
}

fn get_if_else() -> String {
    "if (true) {
        print \"true\";
     } else {
        print \"false\";
     }
     EOF;".to_string()
}

fn get_boolean() -> String {
    "if (true and false or true) {
        print \"true\";
     } else {
        print \"false\";
     }
     EOF;".to_string()
}

fn get_while() -> String {
    "while(true) {
    print \" hello world\";
    }
     EOF;".to_string()
}

fn get_fizzbuzz() -> String {
    "var x = 0;
    while(x < 100) {
        if (x % 3 == 0) {
        print \"fizz\";
        }
        if (x % 5 == 0) {
        print \"buzz\";
        }
        if (x % 5 == 0 and x % 3 == 0) {
        print \"fizzbuzz\";
        }
        x = x + 1;
    }
    EOF;".to_string()
}

fn get_for_loop() -> String {
    "for (var x = 0; x < 100; x = x +1) {
        if (x % 3 == 0) {
        print \"fizz\";
        }
        if (x % 5 == 0) {
        print \"buzz\";
        }
        if (x % 5 == 0 and x % 3 == 0) {
        print \"fizzbuzz\";
        }
    }
    EOF;".to_string()
}

fn get_modulo() -> String {
    "var x = 3;
        if (x % 3 == 0) {
        print \"fizz\";
        }
    EOF;".to_string()
}