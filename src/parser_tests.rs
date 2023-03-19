use crate::parser::Parser;
use crate::token::{Scanner, Token, TokenType};
use crate::Statement::{BlockStatement, IfStatement, ForStatement};
use crate::statements::statement::Statement::WhileStatement;
use crate::token::TokenType::While;

#[test]
fn equality_test() {
    let vec = get_bang_equal_tokens();
    let size = vec.len();
    let mut parser = Parser::new(vec);
    let option = parser.equality();
    println!("{:?}", option.unwrap())
}

fn get_bang_equal_tokens() -> Vec<Token> {
    let token = Token::new(TokenType::Number, "1".to_string(), 0);
    let bang_equal = Token::new(TokenType::BangEqual, "!=".to_string(), 0);
    let second_comparison = Token::new(TokenType::Number, "2".to_string(), 0);
    vec!(token, bang_equal, second_comparison)
}

#[test]
fn parse_var_tokens() {
    let mut tokenizer = Scanner::new();
    let variable = tokenizer.tokenize_string("var x = 1;".to_string());
    let mut parser = Parser::new(variable);
    println!("{:?}", parser.program())
}

#[test]
fn parse_scope() {
    let program =
        "var x;\
    { x = 1; }\
    ";
    let vec = Scanner::new().tokenize_string(String::from(program));
    let mut parser = Parser::new(vec).program();
    assert!(matches!(*parser[1], BlockStatement {..}))
}

#[test]
fn parse_if_else() {
    let x1 =
        "if (true) {
        var x;
        } else {
        var y;
        }    ";
    let vec = Scanner::new().tokenize_string(x1.to_string());
    let mut parser = Parser::new(vec).program();
    assert_eq!(parser.len(), 1);
    assert!(matches!(*parser[0], IfStatement { ..}));
    println!("{:#?}", parser)
}

#[test]
fn parse_logical() {
    let x1 = "if (true or false) {
        print \"true\";
     } else {
        print \"false\";
     }";
    let vec = Scanner::new().tokenize_string(x1.to_string());
    let mut parser = Parser::new(vec).program();
    println!("{:#?}", parser);
    assert_eq!(parser.len(), 1);
    assert!(matches!(*parser[0], IfStatement { ..}));
}

#[test]
fn parse_while() {
    let x1 =
        "while(x) {
        print \"hello\"
        }";
    let vec = Scanner::new().tokenize_string(x1.to_string());
    let mut parser = Parser::new(vec).program();
    println!("{:#?}", parser);
    assert_eq!(parser.len(), 1);
    assert!(matches!(*parser[0], WhileStatement { ..}));
}

#[test]
fn parse_for() {
    let x1 =
        "for(var x = 0; x < 10; x = x + 1) {
        print \"hello\"
        }";
    let vec = Scanner::new().tokenize_string(x1.to_string());
    let mut parser = Parser::new(vec).program();
    println!("{:#?}", parser);
    assert_eq!(parser.len(), 1);
    assert!(matches!(*parser[0], ForStatement { ..}));
}

#[test]
fn parse_desug_for() {
    let x1 =
        "var x = 0;
        while(x < 100) {
        print x;
        x = x + 1;
        };".to_string();
    let vec = Scanner::new().tokenize_string(x1.to_string());
    let mut parser = Parser::new(vec).program();
    println!("{:#?}", parser);
    assert_eq!(parser.len(), 2);
    assert!(matches!(*parser[1], WhileStatement { ..}));
}

#[test]
fn parse_mod() {
    let statement =
    "var x = 3;
        if (x % 3 == 0) {
        print \"fizz\";
        }";
    let vec = Scanner::new().tokenize_string(statement.to_string());
    let mut parser = Parser::new(vec).program();
    println!("{:#?}", parser);
}



