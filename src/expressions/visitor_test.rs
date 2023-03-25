#![allow(warnings, unused)]

use std::ops::Deref;
use std::ptr::eq;
use std::rc::Rc;

use crate::expressions::expression::{Expression, ExpressionRes};
use crate::expressions::expression::Expression::{BinaryExpr, Expr, LiteralExpr, Logical, UnaryExpr};
use crate::expressions::visitor::{ExpressionInterpreter, Visitor};
use crate::parser::Parser;
use crate::statements::statement::Statement;
use crate::token::{Scanner, Token, TokenType};
use crate::token::TokenType::And;

#[test]
fn unary_bang() {
    let expr = UnaryExpr {
        token: Token {
            token_type: TokenType::Bang,
            value: "".to_string(),
            line: 0,
        },
        rhs: Box::new(LiteralExpr { token_type: TokenType::False, value: "".to_string() }),
    };
    let mut visitor = ExpressionInterpreter::new();
    assert!(visitor.eval(expr).boolean);
}

#[test]
fn string_binary_plus() {
    let expr = BinaryExpr {
        token: Token {
            token_type: TokenType::Plus,
            value: "".to_string(),
            line: 0,
        },
        lhs: Box::new(LiteralExpr { token_type: TokenType::String, value: "hello ".to_string() }),
        rhs: Box::new(LiteralExpr { token_type: TokenType::String, value: "world".to_string() }),
    };
    let mut visitor = ExpressionInterpreter::new();
    assert_eq!(visitor.eval(expr).str, "hello world");
}


#[test]
fn visitor_test() {
    let token = Token {
        token_type: TokenType::Minus,
        value: "".to_string(),
        line: 0,
    };
    let equality =  Box::new(BinaryExpr {
        token,
        rhs: Box::new(LiteralExpr {
            token_type: TokenType::Number,
            value: "10".to_string(),
        }),
        lhs:  Box::new(LiteralExpr {
            token_type: TokenType::Number,
            value: "1".to_string(),
        }),
    });
    let expr = Expr {
        value: String::from("here"),
        equality: Some(equality.clone()),
    };

    println!("{:?}", get_visitor().eval(*equality))
}

#[test]
fn logical_test() {
  let logical_false = Box::new(Logical {
        token: Token::new(TokenType::And, "".to_string(), 0),
        rhs: get_false_literal(),
        lhs: get_true_literal(),
    });

    assert!(get_visitor().eval(*logical_false.clone()).boolean);

    let logical_true = Box::new(Logical {
        token: Token::new(TokenType::And, "".to_string(), 0),
        rhs: get_true_literal(),
        lhs: get_true_literal()
    });


    println!("{:#?}", get_visitor().eval(*logical_true.clone()));
    assert!(get_visitor().eval(*logical_true.clone()).boolean);

    let logical = Logical {
        token: Token::new(TokenType::Or, "".to_string(), 0),
        rhs: logical_false,
        lhs: logical_true,
    };
    assert!(get_visitor().eval(logical).boolean);
}

#[test]
fn variable_propagation() {
    let string = "var x  = 1;\
         var y = x;\
         print y;\
         EOF;".to_string();
    let mut parser = get_statements(string);
    let x1 = get_visitor();
    // let res = expr.accept(Rc::new(&*x1));
}

fn get_statements(statement: String) -> Vec<Box<Statement>> {
    let vec = Scanner::new().tokenize_string(statement.to_string());
    Parser::new(vec).program()
}



fn get_false_literal() ->  Box<Expression>{
    Box::new(LiteralExpr { token_type: TokenType::False, value: "".to_string() })
}

fn get_visitor() -> ExpressionInterpreter {
   ExpressionInterpreter::new()
}

fn get_true_literal() ->  Box<Expression> {
    Box::new(LiteralExpr { token_type: TokenType::True, value: "".to_string() })
}
