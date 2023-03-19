#![allow(warnings, unused)]

use std::ops::Deref;
use std::rc::Rc;

use crate::expressions::expression::{BinaryExpr, Expr, Expression, ExpressionRes, LiteralExpr, Logical, UnaryExpr};
use crate::expressions::visitor::{ExpressionInterpreter, Visitor};
use crate::token::{Token, TokenType};
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
    let visitor = ExpressionInterpreter::new();
    let rc = Rc::new(visitor) as Rc<dyn Visitor<ExpressionRes>>;
    let x1 = rc.as_ref();
    assert!(expr.accept(Rc::new(x1).into()).boolean);
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
    let visitor = ExpressionInterpreter::new();


    let rc = Rc::new(visitor) as Rc<dyn Visitor<ExpressionRes>>;
    let x1 = rc.as_ref();
    assert_eq!(expr.accept(Rc::new(x1)).str, "hello world");
}


#[test]
fn visitor_test() {
    let token = Token {
        token_type: TokenType::Minus,
        value: "".to_string(),
        line: 0,
    };
    let equality = BinaryExpr {
        token,
        rhs: Box::new(LiteralExpr {
            token_type: TokenType::Number,
            value: "10".to_string(),
        }),
        lhs: Box::new(LiteralExpr {
            token_type: TokenType::Number,
            value: "1".to_string(),
        }),
    };
    let expr = Expr {
        value: String::from("here"),
        equality: Some(Box::new(equality)),
    };
    let x1 = get_visitor();
    let res = expr.accept(Rc::new(&*x1));
    println!("{:?}", res)
}

#[test]
fn logical_test() {
    let visitor1 = get_visitor();
    let visitor = Rc::new(&*visitor1);

    let logical_false = Logical {
        token: Token::new(TokenType::And, "".to_string(), 0),
        rhs: Box::new(get_false_literal()),
        lhs: Box::new(get_true_literal()),
    };

    let res = logical_false.accept(Rc::clone(&visitor));
    assert!(!res.boolean);

    let logical_true = Logical {
        token: Token::new(TokenType::And, "".to_string(), 0),
        rhs: Box::new(get_true_literal()),
        lhs: Box::new(get_true_literal()),
    };

    let res = logical_true.accept(Rc::clone(&visitor));
    println!("{:#?}", res);
    assert!(res.boolean);

    let logical = Logical {
        token: Token::new(TokenType::Or, "".to_string(), 0),
        rhs: Box::new(logical_false),
        lhs: Box::new(logical_true),
    };
    let res = logical.accept(Rc::clone(&visitor));
    assert!(res.boolean);
}



fn get_false_literal() -> LiteralExpr {
    LiteralExpr { token_type: TokenType::False, value: "".to_string() }
}

fn get_visitor() -> Box<dyn Visitor<ExpressionRes>> {
    let visitor = ExpressionInterpreter::new();
    Box::new(visitor) as Box<dyn Visitor<ExpressionRes>>
}

fn get_true_literal() -> LiteralExpr {
    LiteralExpr { token_type: TokenType::True, value: "".to_string() }
}
