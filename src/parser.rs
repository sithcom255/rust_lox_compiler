use std::thread::current;
use crate::token::{Token, TokenType};
use crate::expressions::expression::{BinaryExpr, Expr, Expression, ExpressionRes, GroupingExpr, LiteralExpr, UnaryExpr, VariableExpr};
use crate::statements::statement::{PrintStatement, Statement, VarDeclaration};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    size: usize,

}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let i = tokens.len();
        Parser {
            tokens,
            current: 0,
            size: i,
        }
    }

    pub fn program(&mut self) -> Vec<Box<dyn Statement>> {
        let mut declarations = Vec::new();
        while  self.current < self.size && self.get_current().token_type != TokenType::EOF  {
            match self.declaration() {
                Some(value) => declarations.push(value),
                None => continue,
            };
        };
        declarations
    }

    pub fn declaration(&mut self) -> Option<Box<dyn Statement>> {
        if self.get_current().token_type == TokenType::Var {
            self.advance();
            let option = self.primary();

            VarDeclaration { expr: Box::new(()), identifier: "".to_string() }
        }
        self.statement_get()

    }

    pub fn statement_get(&mut self) -> Option<Box<dyn Statement>> {
        let statement;
        match self.get_current().token_type {
            TokenType::Print => statement = self.print_statement(),
            _ => statement = self.expression_statement(),
        }
        statement
    }

    pub fn print_statement(&mut self) -> Option<Box<dyn Statement>> {
        self.advance();
        let expression = self.expression();
        self.consume_until(TokenType::Semicolon);

        Some(Box::new(PrintStatement { expr: expression.unwrap() }))
    }
    pub fn expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        self.consume_until(TokenType::Semicolon);
        None
    }

    pub fn expression(&mut self) -> Option<Box<dyn Expression<ExpressionRes>>> {
        let expr = Expr {
            value: "".to_string(),
            equality: Some(self.equality().unwrap()),
        };
        Some(Box::new(expr))
    }

    pub fn equality(&mut self) -> Option<Box<dyn Expression<ExpressionRes>>> {
        let mut lhs = self.comparison().unwrap();

        while self.current < self.size && match self.get_current().token_type {
            TokenType::BangEqual |
            TokenType::EqualEqual => true,
            _ => false,
        } {
            let token = self.get_current().clone();
            self.advance();
            let rhs = self.comparison().unwrap();
            lhs = Box::new(BinaryExpr { token, rhs, lhs });
        }
        Some(lhs)
    }

    fn comparison(&mut self) -> Option<Box<dyn Expression<ExpressionRes>>> {
        let mut lhs = self.term().unwrap();

        while self.current < self.size && match self.get_current().token_type {
            TokenType::Greater |
            TokenType::GreaterEqual |
            TokenType::Less |
            TokenType::LessEqual => true,
            _ => false,
        } {
            let token = self.tokens[self.current].clone();
            self.advance();
            let rhs = self.term().unwrap();
            lhs = Box::new(BinaryExpr { token, rhs, lhs });
        };
        Some(lhs)
    }

    fn term(&mut self) -> Option<Box<dyn Expression<ExpressionRes>>> {
        let mut lhs = self.factor().unwrap();

        while self.current < self.size && match self.get_current().token_type {
            TokenType::Minus |
            TokenType::Plus => true,
            _ => false,
        } {
            let token = self.get_current().clone();
            self.advance();
            let rhs = self.factor().unwrap();
            lhs = Box::new(BinaryExpr { token, rhs, lhs });
        };
        Some(lhs)
    }

    fn factor(&mut self) -> Option<Box<dyn Expression<ExpressionRes>>> {
        let mut lhs = self.unary().unwrap();

        while self.current < self.size && match self.tokens[self.current].token_type {
            TokenType::Slash |
            TokenType::Star => true,
            _ => false,
        } {
            let token = self.tokens[self.current].clone();
            self.advance();
            let rhs = self.unary().unwrap();
            lhs = Box::new(BinaryExpr { token, rhs, lhs });
        };
        Some(lhs)
    }

    fn unary(&mut self) -> Option<Box<dyn Expression<ExpressionRes>>> {
        while self.current < self.size && match self.tokens[self.current].token_type {
            TokenType::Bang |
            TokenType::Minus => true,
            _ => false,
        } {
            let token = self.tokens[self.current].clone();
            self.advance();
            let rhs = self.unary().unwrap();
            return Some(Box::new(UnaryExpr { token, rhs }));
        };
        return self.primary();
    }

    fn primary(&mut self) -> Option<Box<dyn Expression<ExpressionRes>>> {
        let primary: Box<dyn Expression<ExpressionRes>> = match self.tokens[self.current].token_type {
            TokenType::False |
            TokenType::True |
            TokenType::Nil => {
                let token = self.tokens[self.current].clone();
                self.advance();
                Box::new(LiteralExpr { token_type: token.token_type, value: token.value })
            }
            TokenType::String |
            TokenType::Number => {
                let token = self.tokens[self.current].clone();
                self.advance();
                Box::new(LiteralExpr { token_type: token.token_type, value: token.value })
            }
            TokenType::LeftParen => {
                self.advance();
                let expression = self.expression().unwrap();
                if self.tokens[self.current].token_type != TokenType::RightParen {
                    println!("Error, missing right brace {:?}", self.tokens[self.current])
                }
                self.advance();
                Box::new(GroupingExpr { value: expression })
            }
            _ => {
                let token = self.get_current().clone();

                self.advance();
                Box::new(VariableExpr { token_type : token.token_type, value: token.value})
            }
        };
        Some(primary)
    }


    fn get_current(&mut self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn consume_until(&mut self, token: TokenType) {
        while self.current < self.size && self.tokens[self.current].token_type != token {
            self.advance();
        }
        if self.current < self.size && self.get_current().token_type == token {
            self.advance();
        }
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
        token_type: TokenType::Number,
        value: "1".to_string(),
        line: 0,
    };

    let bang_equal = Token {
        token_type: TokenType::BangEqual,
        value: "!=".to_string(),
        line: 0,
    };

    let second_comparison = Token {
        token_type: TokenType::Number,
        value: "2".to_string(),
        line: 0,
    };

    vec!(token, bang_equal, second_comparison)
}
