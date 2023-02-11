use std::ptr::eq;
use crate::token::{Token, TokenType};
use crate::expressions::expression;
use crate::expressions::expression::{BinaryExpr, Comparison, Equality, Expr, Expression, LiteralExpr};

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

    fn equality(&mut self) -> Option<Box<dyn Expression>> {
        let rhs = self.comparison().or_else("This is bad");

        while match self.tokens[self.current].token_type {
            TokenType::BangEqual |
            TokenType::EqualEqual => {
                let token = &self.tokens[self.current];
                self.advance();
                let rhs = self.comparison().ok_or("This should be a comparison");
                BinaryExpr { token, rhs: Box::new(rhs.unwrap()),  lhs: Box::new(rhs.unwrap()) };
                true
            },
            _ =>  false,
        } {

        }
        None
    }

    fn comparison(&mut self) -> Option<Box<dyn Expression>>{
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


    fn advance(&mut self) {
        self.current += 1;
    }
}



