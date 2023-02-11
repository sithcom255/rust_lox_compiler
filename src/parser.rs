use std::ops::Deref;
use std::ptr::eq;
use crate::token::{Token, TokenType};
use crate::expressions::expression;
use crate::expressions::expression::{BinaryExpr, Comparison, Equality, Expr, Expression, LiteralExpr};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
    size: usize,

}

impl Parser {
    fn expression(&mut self) -> Option<Box<dyn Expression>> {
        let equality = self.equality();

        let expr = Expr {
            value: "".to_string(),
            equality: Some(equality.unwrap()),
        };
        Some(Box::new(expr))
    }

    fn equality(&mut self) -> Option<Box<dyn Expression>> {
        let mut lhs = self.comparison().unwrap();

        while self.current < self.size && match self.tokens[self.current].token_type {
            TokenType::BangEqual |
            TokenType::EqualEqual => true,
            _ => false,
        } {
            let token = self.tokens[self.current].clone();
            self.advance();
            let rhs = self.comparison().unwrap();
            lhs = Box::new(BinaryExpr { token, rhs, lhs });
        }
        Some(lhs)
    }

    fn comparison(&mut self) -> Option<Box<dyn Expression>> {
        let mut lhs = self.term().unwrap();
        self.advance();
        while self.current < self.size && match self.tokens[self.current].token_type {
            TokenType::Greater |
            TokenType::GreaterEqual |
            TokenType::Less |
            TokenType::LessEqual => true,
            _ => false,
        } {
            let rhs = self.comparison().unwrap();
            let token = self.tokens[self.current].clone();
            lhs = Box::new(BinaryExpr { token, rhs, lhs });
            self.advance();
        };
        Some(lhs)
    }

    fn term(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(LiteralExpr {
            token: Token {
                token_type: TokenType::LeftParen,
                value: "".to_string(),
                line: 0,
            },
            value: "".to_string(),
        }))
    }

    fn factor(&mut self) {
        // self.comparison()
    }

    fn unary(&mut self) {
        // self.comparison()
    }

    fn primary(&mut self) -> LiteralExpr {
        LiteralExpr {
            token: Token {
                token_type: TokenType::LeftParen,
                value: "".to_string(),
                line: 0,
            },
            value: "".to_string(),
        }
    }


    fn advance(&mut self) {
        self.current += 1;
    }
}

#[test]
fn equality_test() {
    let vec = get_bang_equal_tokens();
    let size = vec.len();
    let mut parser = Parser { tokens: vec, current: 0, size: size };
    let option = parser.equality();
    println!("{:?}", option.unwrap())
}

fn get_bang_equal_tokens() -> Vec<Token> {
    let token = Token {
        token_type: TokenType::LeftParen,
        value: "".to_string(),
        line: 0,
    };

    let bang_equal = Token {
        token_type: TokenType::BangEqual,
        value: "!=".to_string(),
        line: 0,
    };

    let second_comparison = Token {
        token_type: TokenType::LeftParen,
        value: "".to_string(),
        line: 0,
    };

    vec!(token, bang_equal, second_comparison)
}
