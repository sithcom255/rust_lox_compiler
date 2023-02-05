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
    Greate,
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
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    value: String,
    line: u32,
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
        self.chars = content.chars().map(|ch| ch as char ).collect::<Vec<_>>();
        self.size = self.chars.len();
        self.current = 0;
        self.line = 0;
        let mut vec = Vec::new();
        vec.push(self.tokenize(content));
        vec
    }

    fn advance(&mut self) {

    }

    fn peek_advance(&mut self, peeked: usize, expected: &char) -> bool {
        if peeked >= self.size || !self.chars[peeked].eq(expected) {
            return false;
        }
        self.current += 1;
        return true
    }

    pub fn tokenize(&mut self, value: String) -> Token {

        let token_type = match self.chars.iter().next().unwrap() {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            _ => self.parse_unknown(value.as_ref()),
        };
        Token { token_type, value, line: 0 }
    }

    pub fn parse_single_char(&self, value: usize) -> Option<TokenType> {
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
            _ => None
        }
    }

    // pub fn parse_two_chars(&self, value: usize) -> Option<TokenType> {
    //    match self.chars[value] {
    //        '!' =>
    //        '_' =>
    //         '<' =>
    //         '>' =>
    //    }
    // }


    fn parse_unknown(&self, value: &str) -> TokenType {
        let string_regex = Regex::new("\"\\w+\"");
        if string_regex.unwrap().is_match(value.as_ref()) {
            return TokenType::String;
        };
        let value = TokenType::Nil;
        value
    }
}


#[test]
pub fn test_parsing_one_token() {
    let mut parser = Parser::new();
    parser.parse_string("+".to_string());
    let variable = parser.parse_single_char(0);
    assert_eq!(variable, Some(TokenType::Plus));
}

#[test]
pub fn parse_two_char_token() {
    let parser = Parser::new();
    // let variable = parser.parse_two_chars("!=");
}

#[test]
pub fn peek_advance() {
    let mut parser = Parser::new();
    parser.parse_string("hello".to_string());
    let variable = parser.peek_advance(1,&'e');
}