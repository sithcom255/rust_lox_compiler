use crate::token::{Scanner, Token, TokenType};

#[test]
fn test_tokenizing_one_token() {
    let mut tokenizer = Scanner::new();
    tokenizer.tokenize_string("+".to_string());
    let variable = tokenizer.tokenize_single_char(0);
    assert_eq!(variable, Some(TokenType::Plus));
}

#[test]
fn tokenize_two_char_token() {
    let mut tokenizer = Scanner::new();
    tokenizer.tokenize_string("!=".to_string());
    let variable = tokenizer.tokenize_two_chars(0);
    println!("{:?}", variable)
}

#[test]
fn tokenize() {
    let mut tokenizer = Scanner::new();
    let variable = tokenizer.tokenize_string("+".to_string());
    let token = Token {
        token_type: TokenType::Plus,
        value: "+".to_string(),
        line: 0,
    };
    assert_eq!(vec![token], variable)
}

#[test]
fn tokenize_two_chars() {
    let mut tokenizer = Scanner::new();
    let variable = tokenizer.tokenize_string("!= ".to_string());
    let token = Token {
        token_type: TokenType::BangEqual,
        value: "!=".to_string(),
        line: 0,
    };
    assert_eq!(vec![token], variable)
}

#[test]
fn get_range() {
    let tokenizer = Scanner::new();
    let chars = vec!['a', 'n', 'd'];
    let string_from_range = tokenizer.get_string_from_char_range(0, 3, &chars);
    assert_eq!("and", string_from_range)
}

#[test]
fn get_range_short() {
    let tokenizer = Scanner::new();
    let chars = vec!['a', 'n', 'd'];
    let string_from_range = tokenizer.get_string_from_char_range(0, 1, &chars);
    assert_eq!("a", string_from_range)
}

#[test]
fn tokenize_and() {
    let mut tokenizer = Scanner::new();
    let variable = tokenizer.tokenize_string(" and ".to_string());
    let token = Token {
        token_type: TokenType::And,
        value: "and".to_string(),
        line: 0,
    };
    assert_eq!(vec![token], variable)
}

#[test]
fn remove_comment() {
    let mut tokenizer = Scanner::new();
    let variable = tokenizer.tokenize_string(" and // lots of text ".to_string());
    let token = Token {
        token_type: TokenType::And,
        value: "and".to_string(),
        line: 0,
    };
    assert_eq!(vec![token], variable)
}

#[test]
fn remove_comment_keep_next() {
    let mut tokenizer = Scanner::new();
    let variable = tokenizer.tokenize_string(
        " and // lots of text
    and"
            .to_string(),
    );
    let token = Token {
        token_type: TokenType::And,
        value: "and".to_string(),
        line: 0,
    };
    let token2 = Token {
        token_type: TokenType::And,
        value: "and".to_string(),
        line: 1,
    };
    assert_eq!(vec![token, token2], variable)
}

#[test]
fn check_correct_line_tokenizing() {
    let mut tokenizer = Scanner::new();
    let variable = tokenizer.tokenize_string(
        " and
    and"
            .to_string(),
    );
    let token = Token {
        token_type: TokenType::And,
        value: "and".to_string(),
        line: 0,
    };
    let token2 = Token {
        token_type: TokenType::And,
        value: "and".to_string(),
        line: 1,
    };
    assert_eq!(vec![token, token2], variable)
}

#[test]
fn handle_example() {
    let mut tokenizer = Scanner::new();
    let variable = tokenizer.tokenize_string("(( )){}".to_string());
    let token1 = Token {
        token_type: TokenType::LeftParen,
        value: "(".to_string(),
        line: 0,
    };
    let token2 = Token {
        token_type: TokenType::LeftParen,
        value: "(".to_string(),
        line: 0,
    };
    let token3 = Token {
        token_type: TokenType::RightParen,
        value: ")".to_string(),
        line: 0,
    };
    let token4 = Token {
        token_type: TokenType::RightParen,
        value: ")".to_string(),
        line: 0,
    };
    let token5 = Token {
        token_type: TokenType::LeftBrace,
        value: "{".to_string(),
        line: 0,
    };
    let token6 = Token {
        token_type: TokenType::RightBrace,
        value: "}".to_string(),
        line: 0,
    };
    assert_eq!(
        vec![token1, token2, token3, token4, token5, token6],
        variable
    )
}

#[test]
fn tokenize_numeric() {
    let mut tokenizer = Scanner::new();
    let variable = tokenizer.tokenize_string(" a 1".to_string());
    let token = Token {
        token_type: TokenType::Identifier,
        value: "a".to_string(),
        line: 0,
    };
    let token2 = Token {
        token_type: TokenType::Number,
        value: "1".to_string(),
        line: 0,
    };
    assert_eq!(vec![token, token2], variable)
}

#[test]
fn tokenize_string_empty() {
    let mut tokenizer = Scanner::new();
    let variable = tokenizer.tokenize_string(" \"hello\"".to_string());
    let token = Token {
        token_type: TokenType::String,
        value: "hello".to_string(),
        line: 0,
    };
    assert_eq!(vec![token], variable)
}

#[test]
fn tokenize_string_multiline() {
    let mut tokenizer = Scanner::new();
    let variable = tokenizer.tokenize_string(
        " \"
    \""
            .to_string(),
    );
    let token = Token {
        token_type: TokenType::String,
        value: "
    "
            .to_string(),
        line: 0,
    };
    assert_eq!(vec![token], variable)
}

#[test]
fn tokenize_string_throws_unterminated_string() {
    let mut tokenizer = Scanner::new();
    let variable = tokenizer.tokenize_string(" \"".to_string());
    assert_eq!(Vec::<Token>::new(), variable)
}