use crate::token::Token;

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
