use std::ptr::eq;
use crate::token::{Token, TokenType};
use crate::expressions::expression;
use crate::expressions::expression::{Comparison, Equality, Expr, LiteralExpr};

struct Parser {
    tokens: Vec<Token>,
    current: usize,


}

impl Parser {
    fn expression(&mut self) -> Option<Expr>  {
        let equality = self.equality();

        let expr = Expr {
            value: "".to_string(),
            equality: Some(Box::new(equality.unwrap())),
        };
        Some(expr)
    }

    fn equality(&mut self) -> Option<Equality> {
        let rhs = self.comparison();

        while match self.tokens[self.current].token_type {
            TokenType::BangEqual |
            TokenType::EqualEqual => {
                let token = &self.tokens[self.current];
                true
            },
            _ =>  false,
        } {

        }
        Some(Equality { token_type: TokenType::LeftParen, value: "".to_string() })
    }

    fn comparison(&mut self) -> Option<Comparison>{
        self.comparison()
    }

    fn term(&mut self) {
        // self.comparison()
    }

    fn factor(&mut self) {
        // self.comparison()
    }

    fn unary(&mut self) {
        // self.comparison()
    }

    fn primary(&mut self) -> LiteralExpr {
        LiteralExpr { token: Token {
            token_type: TokenType::LeftParen,
            value: "".to_string(),
            line: 0,
        }, value: "".to_string() }
    }




}
