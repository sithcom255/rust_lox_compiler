
use crate::token::{Scanner, Token, TokenType};
use crate::token::TokenType::{Else, Equal, Identifier, If, LeftBrace, LeftParen, Number, RightBrace, RightParen, Semicolon, Var};

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

#[test]
fn tokenize_var_declaration() {
    let mut tokenizer = Scanner::new();
    let variable = tokenizer.tokenize_string("var x = 1".to_string());
    let var = Token::new(Var, String::from("var"), 0);
    let x = Token::new(Identifier, String::from("x"), 0);
    let equals = Token::new(Equal, String::from("="), 0);
    let one = Token::new(Number, String::from("1"), 0);
    assert_eq!(vec![var, x, equals, one], variable)
}

#[test]
fn tokenize_scope_declaration() {
    let mut tokenizer = Scanner::new();
    let variable = tokenizer.tokenize_string("var x;\
     { x = 1; }\
     ".to_string());
    let var = Token::new(Var, String::from("var"), 0);
    let x = Token::new(Identifier, String::from("x"), 0);
    let semi = Token::new(Semicolon, String::from(";"), 0);
    let semi2 = Token::new(Semicolon, String::from(";"), 0);
    let brace = Token::new(LeftBrace, String::from("{"), 0);
    let x_assign = Token::new(Identifier, String::from("x"), 0);
    let equals = Token::new(Equal, String::from("="), 0);
    let one = Token::new(Number, String::from("1"), 0);
    let brace2 = Token::new(RightBrace, String::from("}"), 0);
    assert_eq!(vec![var, x, semi, brace, x_assign, equals, one, semi2, brace2], variable)
}

#[test]
fn tokenize_if() {
    let x1 =
        "if (true) {
        var x;
        } else {
        var y;
        }    ";
    let variable = Scanner::new().tokenize_string(x1.to_string());


    let if_tok = Token::new(If, String::from("if"), 0);
    let l = Token::new(LeftParen, String::from("("), 0);
    let boo = Token::new(TokenType::True, String::from("true"), 0);
    let r = Token::new(RightParen, String::from(")"), 0);
    let bracel = Token::new(LeftBrace, String::from("{"), 0);
    let var = Token::new(Var, String::from("var"), 1);
    let x = Token::new(Identifier, String::from("x"), 1);
    let semi = Token::new(Semicolon, String::from(";"), 1);
    let bracer = Token::new(RightBrace, String::from("}"), 2);
    let else_tok = Token::new(Else, String::from("else"), 2);

    let bracel2 = Token::new(LeftBrace, String::from("{"), 2);
    let vary = Token::new(Var, String::from("var"), 3);
    let y = Token::new(Identifier, String::from("y"), 3);
    let sem2 = Token::new(Semicolon, String::from(";"), 3);
    let bracer2 = Token::new(RightBrace, String::from("}"), 4);

    assert_eq!(vec![if_tok, l, boo, r, bracel, var, x, semi, bracer, else_tok, bracel2, vary,
                    y, sem2, bracer2], variable)
}