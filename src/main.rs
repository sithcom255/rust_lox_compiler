#![allow(warnings, unused)]
extern crate core;

use log::{LevelFilter, SetLoggerError, trace};
use simple_logger::SimpleLogger;
use crate::parser::Parser;
use crate::statements::statement::Statement;
use crate::statements::stmt_visitor::StatementInterpreter;
use crate::token::{Scanner, Token};

mod expressions;
mod parser;
mod token;
#[cfg(test)]
mod token_test;
mod statements;
mod env;
mod program;
#[cfg(test)]
mod parser_tests;
mod resolver_visitor;

fn get_class() -> String  {
    "class Hello {
        world() {
         print \"Hello world\";
        }
    }
    var x = Hello();

    // print x;
    x.world();

    ".to_string()
}

fn main() {
    init();
    let program = get_statement(get_class());
    debug(&program);
    let mut interpreter = StatementInterpreter::new_default();
    interpreter.interpret(program);
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(SimpleLogger::new()))
        .map(|()| log::set_max_level(LevelFilter::Debug))
}

fn debug(p: &Vec<Box<Statement>>) {
    for statement in p {
        trace!("{:#?}", statement)
    }
}
fn debug_token(p: &Vec<Token>) {
    for statement in p {
        println!("{:#?}", statement)
    }
}

fn get_statement(program: String) -> Vec<Box<Statement>> {
    let vec = Scanner::new().tokenize_string(String::from(program));
    // debug_token(&vec);
    let mut parser = Parser::new(vec);
    let program = parser.program();
    program
}

fn return_fn() -> String {
    "fun hello() {
       return \"Hello world\";
    }
    print hello();
    EOF;
    ".to_string()
}

fn multiple_assigments() -> String {
    "var x;
     var y = x = 2;
     print y;
     EOF;".to_string()
}

fn value_propagation() -> String {
    "var x =1;
     var y = x + 2;
     x = 2;
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


fn string_comparison() -> String {
    "var param = \"ok\"

        if (param == \"ok\") {
        print \"ok\";
        }

    EOF;
    ".to_string()

}

fn easy_function() -> String {
    "fun hello(param) {
        var paramOut =\" nono\";
        if (param == \"ok\") {
            hello(\"notOK\");
        }
        print \" hello this: \" + param + paramOut;
    }

    var paramOut = \"closedOver\";
    hello(\"ok\");
    EOF;
    ".to_string()
}
