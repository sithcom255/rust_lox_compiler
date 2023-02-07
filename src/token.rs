use std::env::var;
use std::fs;
use regex::Regex;


#[derive(Debug, PartialEq)]
pub enum TokenType {
    // single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    String,
    Number,

    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
    Space,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    value: String,
    line: usize,
}

pub struct Parser {
    current: usize,
    size: usize,
    line: usize,
    chars: Vec<char>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser { current: 0, size: 0, line: 0, chars: vec![] }
    }

    pub fn parse_file(&mut self, file_path: &str) -> Vec<Token> {
        let content = fs::read_to_string(file_path).expect("Error when reading");
        self.parse_string(content)
    }

    pub fn parse_string(&mut self, content: String) -> Vec<Token> {
        self.chars = content.chars().map(|ch| ch as char).collect::<Vec<_>>();
        self.size = self.chars.len();
        self.current = 0;
        self.line = 0;

        self.tokenize()
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut result_tokens = Vec::new();
        while self.current < self.size {
            let initial = self.current;
            match self.parse_single_char(self.current) {
                Some(TokenType::Space) => {
                    self.advance();
                    continue; 
                }
                Some(token_type) => {
                    let mut char_array = vec![' '];
                    char_array.copy_from_slice(&self.chars[self.current..self.current + 1]);
                    let value = char_array.iter().collect();
                    result_tokens.push(Token { token_type, value, line: self.line });
                    self.advance();
                    continue;
                }
                None => {}
            };

            match self.parse_two_chars(self.current) {
                
                Some(token_type) => {
                    let mut arr = vec![' '; 2];
                    self.chars[initial..self.current + 1].clone_into(&mut arr);
                    let value = arr.iter().collect();
                    result_tokens.push(Token { token_type, value, line: self.line });
                    self.advance();
                    continue;
                }
                None => {}
            };
            match self.parse_other(self.current) {
                Some(value) => {
                    // vec.push(value);
                    self.advance();
                    continue;
                }
                None => {}
            };
        };

        result_tokens
    }

    fn parse_single_char(&self, value: usize) -> Option<TokenType> {
        match self.chars[value] {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            ';' => Some(TokenType::Semicolon),
            '*' => Some(TokenType::Star),
            ' ' => Some(TokenType::Space),
             _ => None
        }
    }

    pub fn parse_two_chars(&mut self, value: usize) -> Option<TokenType> {
        match self.chars[value] {
            '!' => return if self.peek_advance(value + 1, &'=')
            {
                Some(TokenType::BangEqual)
            } else {
                Some(TokenType::Bang)
            },
            '=' => return if self.peek_advance(value + 1, &'=')
            {
                Some(TokenType::EqualEqual)
            } else {
                Some(TokenType::Equal)
            },
            '<' => return if self.peek_advance(value + 1, &'=')
            {
                Some(TokenType::LessEqual)
            } else {
                Some(TokenType::Less)
            },
            '>' => return if self.peek_advance(value + 1, &'=')
            {
                Some(TokenType::GreaterEqual)
            } else {
                Some(TokenType::Greater)
            },
            _ => None
        }
    }


    fn peek_advance(&mut self, peeked: usize, expected: &char) -> bool {
        if peeked >= self.size || !self.chars[peeked].eq(expected) {
            return false;
        }
        self.current += 1;
        return true;
    }
    fn parse_other(&self, current: usize) -> Option<TokenType> {
        // let regex = Regex::new("[a-zA-Z]").unwrap();
        if self.chars[self.current].is_alphabetic() {

        };
        return Some(TokenType::Plus)
    }
}


#[test]
fn test_parsing_one_token() {
    let mut parser = Parser::new();
    parser.parse_string("+".to_string());
    let variable = parser.parse_single_char(0);
    assert_eq!(variable, Some(TokenType::Plus));
}

#[test]
fn parse_two_char_token() {
    let mut parser = Parser::new();
    parser.parse_string("!=".to_string());
    let variable = parser.parse_two_chars(0);
    println!("{:?}", variable)
}

#[test]
fn peek_advance() {
    let mut parser = Parser::new();
    parser.parse_string("hello".to_string());
    let variable = parser.peek_advance(1, &'e');
}

#[test]
fn tokenize() {
    let mut parser = Parser::new();
    let variable = parser.parse_string("+".to_string());
    let token = Token{ token_type : TokenType::Plus, value: "+".to_string(), line: 0};
    assert_eq!(vec![token], variable)
}

#[test]
fn tokenize_two_chars() {
    let mut parser = Parser::new();
    let variable = parser.parse_string("!= ".to_string());
    let token = Token{ token_type : TokenType::BangEqual, value: "!=".to_string(), line: 0};
    assert_eq!(vec![token], variable)
}

#[test]
fn tokenize_and() {
    let mut parser = Parser::new();
    let variable = parser.parse_string(" and ".to_string());
    let token = Token{ token_type : TokenType::And, value: "and".to_string(), line: 0};
    assert_eq!(vec![token], variable)
}

