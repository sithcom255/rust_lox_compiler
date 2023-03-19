use std::fs;

use crate::log::Log;

#[derive(Debug, PartialEq, Copy, Clone)]
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
    Percent,
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

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value: String, line: usize) -> Token {
        Token {
            token_type,
            value,
            line,
        }
    }

    pub fn new_simple(token_type: TokenType, value: String) -> Token {
        Token {
            token_type,
            value,
            line: 0,
        }
    }
}

pub struct Scanner {
    current: usize,
    size: usize,
    line: usize,
    chars: Vec<char>,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            current: 0,
            size: 0,
            line: 0,
            chars: vec![],
        }
    }

    pub fn tokenize_file(&mut self, file_path: &str) -> Vec<Token> {
        let content = fs::read_to_string(file_path).expect("Error when reading");
        self.tokenize_string(content)
    }

    pub fn tokenize_string(&mut self, content: String) -> Vec<Token> {
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
        let mut tokens = Vec::new();
        while self.current < self.size {
            let initial = self.current;
            match self.tokenize_single_char(self.current) {
                Some(TokenType::Space) => {
                    self.advance();
                    continue;
                }
                Some(token_type) => {
                    let value = self.get_string_from_char_range(
                        self.current,
                        self.current + 1,
                        &self.chars,
                    );
                    tokens.push(Token {
                        token_type,
                        value,
                        line: self.line,
                    });
                    self.advance();
                    continue;
                }
                None => {}
            };

            match self.tokenize_two_chars(self.current) {
                Some(token_type) => {
                    let value =
                        self.get_string_from_char_range(initial, self.current + 1, &self.chars);
                    tokens.push(Token {
                        token_type,
                        value,
                        line: self.line,
                    });
                    self.advance();
                    continue;
                }
                None => {}
            };

            match self.consume_comment_or_divide() {
                Some(TokenType::Space) => {
                    continue;
                }
                Some(_value) => {
                    tokens.push(Token {
                        token_type: TokenType::Slash,
                        value: "/".to_string(),
                        line: self.line,
                    });
                    continue;
                }
                None => {}
            }

            if self.chars[self.current].is_alphabetic() || self.chars[self.current] == '_' {
                tokens.push(self.get_alphabetic_token(initial));
                continue;
            }

            if self.chars[self.current].is_numeric() {
                tokens.push(self.get_numeric_token(initial));
                continue;
            }

            if self.chars[self.current] == '"' {
                match self.get_string_token(initial) {
                    Some(token) => tokens.push(token),
                    None => break,
                };
            }
        }

        tokens
    }

    pub(crate) fn tokenize_single_char(&mut self, value: usize) -> Option<TokenType> {
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
            '%' => Some(TokenType::Percent),
            ' ' => Some(TokenType::Space),
            '\t' => Some(TokenType::Space),
            '\n' => {
                self.line += 1;
                Some(TokenType::Space)
            }
            _ => None,
        }
    }

    pub fn tokenize_two_chars(&mut self, value: usize) -> Option<TokenType> {
        match self.chars[value] {
            '!' => {
                return if self.peek_advance(value + 1, &'=') {
                    Some(TokenType::BangEqual)
                } else {
                    Some(TokenType::Bang)
                };
            }
            '=' => {
                return if self.peek_advance(value + 1, &'=') {
                    Some(TokenType::EqualEqual)
                } else {
                    Some(TokenType::Equal)
                };
            }
            '<' => {
                return if self.peek_advance(value + 1, &'=') {
                    Some(TokenType::LessEqual)
                } else {
                    Some(TokenType::Less)
                };
            }
            '>' => {
                return if self.peek_advance(value + 1, &'=') {
                    Some(TokenType::GreaterEqual)
                } else {
                    Some(TokenType::Greater)
                };
            }
            _ => None,
        }
    }

    fn consume_comment_or_divide(&mut self) -> Option<TokenType> {
        match self.chars[self.current] {
            '/' => {
                self.advance();
                if self.peek_advance(self.current, &'/') {
                    while self.chars[self.current - 1] != '\n' && self.current < self.size {
                        self.advance()
                    }
                    self.line += 1;
                    return Some(TokenType::Space);
                }
                return Some(TokenType::Slash);
            }
            _ => None,
        }
    }

    fn get_alphabetic_token(&mut self, initial: usize) -> Token {
        while self.current < self.size
            && (self.chars[self.current].is_alphabetic() || self.chars[self.current] == '_')
        {
            self.advance();
        }

        let value = self.get_string_from_char_range(initial, self.current, &self.chars);
        let token = Token {
            token_type: self.identifier_alternatives(&value),
            value,
            line: self.line,
        };
        token
    }

    fn identifier_alternatives(&mut self, value: &str) -> TokenType {
        match value {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "fun" => TokenType::Fun,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            "eof" => TokenType::EOF,
            _ => TokenType::Identifier,
        }
    }

    fn get_numeric_token(&mut self, initial: usize) -> Token {
        while self.current < self.size && self.chars[self.current].is_numeric() {
            self.advance();
        }
        let value = self.get_string_from_char_range(initial, self.current, &self.chars);
        let token = Token {
            token_type: TokenType::Number,
            value,
            line: self.line,
        };
        token
    }

    fn get_string_token(&mut self, initial: usize) -> Option<Token> {
        self.advance();
        if self.current == self.size {
            return None;
        }
        while self.chars[self.current] != '"' {
            if self.current >= (self.size - 1) {
                Log::error(self.line, "Missing closing \" in a string");
                return None;
            }
            self.advance();
        }
        self.advance();
        let value = self
            .get_string_from_char_range(initial, self.current, &self.chars)
            .strip_prefix("\"")
            .unwrap()
            .strip_suffix("\"")
            .unwrap()
            .to_string();
        Some(Token {
            token_type: TokenType::String,
            value,
            line: self.line,
        })
    }

    pub(crate) fn get_string_from_char_range(
        &self,
        start_inclusive: usize,
        end_exclusive: usize,
        chars: &Vec<char>,
    ) -> String {
        let mut char_array = vec![' '; end_exclusive - start_inclusive];
        char_array.copy_from_slice(&chars[start_inclusive..end_exclusive]);
        char_array.iter().collect()
    }
    fn peek_advance(&mut self, peeked: usize, expected: &char) -> bool {
        if peeked >= self.size || !self.chars[peeked].eq(expected) {
            return false;
        }
        self.current += 1;
        return true;
    }
}
