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
                    let value = self.get_string_from_char_range(self.current, self.current + 1, &self.chars);
                    result_tokens.push(Token { token_type, value, line: self.line });
                    self.advance();
                    continue;
                }
                None => {}
            };

            match self.parse_two_chars(self.current) {
                Some(token_type) => {
                    let value = self.get_string_from_char_range(initial, self.current + 1, &self.chars);
                    result_tokens.push(Token { token_type, value, line: self.line });
                    self.advance();
                    continue;
                }
                None => {}
            };

            self.consume_comment_or_divide();


            if self.chars[self.current].is_alphabetic() {
                while self.current < self.size && self.chars[self.current].is_alphabetic() {
                    self.advance();
                }

                let value = self.get_string_from_char_range(initial, self.current, &self.chars);
                match self.parse_other(&value) {
                    Some(token_type) => {
                        result_tokens.push(Token { token_type, value, line: self.line });
                        continue;
                    }
                    None => {}
                };
            }

            if self.chars[self.current].is_numeric() {
                while self.current < self.size && self.chars[self.current].is_numeric() {
                    self.advance();
                }
                let value = self.get_string_from_char_range(initial, self.current, &self.chars);
                match self.parse_other(&value) {
                    Some(token_type) => {
                        result_tokens.push(Token { token_type, value, line: self.line });
                        continue;
                    }
                    None => {}
                };
            }

        };

        result_tokens
    }

    fn parse_single_char(&mut self, value: usize) -> Option<TokenType> {
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
            '\t' => Some(TokenType::Space),
            '\n' => {
                self.line += 1;
                Some(TokenType::Space)
            }
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

    fn parse_other(&mut self, value: &str) -> Option<TokenType> {
        match value {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::Fun),
            "for" => Some(TokenType::For),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            "eof" => Some(TokenType::EOF),
            _ => None,
        }
    }


    fn consume_comment_or_divide(&mut self) -> Option<TokenType> {
        match self.chars[self.current] {
            '/' => {
                self.advance();
                if self.peek_advance(self.current, &'/') {
                    while self.chars[self.current - 1] == '\n' && self.current < self.size {
                        self.advance()
                    }
                    self.line += 1;
                    return None;
                }
                return Some(TokenType::Slash);
            }
            _ => None
        }
    }

    fn get_string_from_char_range(&self, start_inclusive: usize, end_exclusive: usize, chars: &Vec<char>) -> String {
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
    let token = Token { token_type: TokenType::Plus, value: "+".to_string(), line: 0 };
    assert_eq!(vec![token], variable)
}

#[test]
fn tokenize_two_chars() {
    let mut parser = Parser::new();
    let variable = parser.parse_string("!= ".to_string());
    let token = Token { token_type: TokenType::BangEqual, value: "!=".to_string(), line: 0 };
    assert_eq!(vec![token], variable)
}

#[test]
fn get_range() {
    let mut parser = Parser::new();
    let chars = vec!['a', 'n', 'd'];
    let string_from_range = parser.get_string_from_char_range(0, 3, &chars);
    assert_eq!("and", string_from_range)
}

#[test]
fn get_range_short() {
    let mut parser = Parser::new();
    let chars = vec!['a', 'n', 'd'];
    let string_from_range = parser.get_string_from_char_range(0, 1, &chars);
    assert_eq!("a", string_from_range)
}

#[test]
fn tokenize_and() {
    let mut parser = Parser::new();
    let variable = parser.parse_string(" and ".to_string());
    let token = Token { token_type: TokenType::And, value: "and".to_string(), line: 0 };
    assert_eq!(vec![token], variable)
}


#[test]
fn remove_comment() {
    let mut parser = Parser::new();
    let variable = parser.parse_string(" and // lots of text ".to_string());
    let token = Token { token_type: TokenType::And, value: "and".to_string(), line: 0 };
    assert_eq!(vec![token], variable)
}

#[test]
fn remove_comment_keep_next() {
    let mut parser = Parser::new();
    let variable = parser.parse_string(" and // lots of text \
    and".to_string());
    let token = Token { token_type: TokenType::And, value: "and".to_string(), line: 0 };
    let token2 = Token { token_type: TokenType::And, value: "and".to_string(), line: 0 };
    assert_eq!(vec![token, token2], variable)
}

#[test]
fn handle_example() {
    let mut parser = Parser::new();
    let variable = parser.parse_string("(( )){}".to_string());
    let token1 = Token { token_type: TokenType::LeftParen, value: "(".to_string(), line: 0 };
    let token2 = Token { token_type: TokenType::LeftParen, value: "(".to_string(), line: 0 };
    let token3 = Token { token_type: TokenType::RightParen, value: ")".to_string(), line: 0 };
    let token4 = Token { token_type: TokenType::RightParen, value: ")".to_string(), line: 0 };
    let token5 = Token { token_type: TokenType::LeftBrace, value: "{".to_string(), line: 0 };
    let token6 = Token { token_type: TokenType::RightBrace, value: "}".to_string(), line: 0 };
    assert_eq!(vec![token1, token2, token3, token4, token5, token6], variable)
}

