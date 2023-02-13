use crate::expressions::expression::{BinaryExpr, Expression, LiteralExpr, UnaryExpr};
use crate::expressions::visitor::ExpressionVisitor;
use crate::token::{Token, TokenType};

#[test]
fn unary_bang() {
    let expr = UnaryExpr {
        token: Token {
            token_type: TokenType::Bang,
            value: "".to_string(),
            line: 0,
        },
        rhs: Box::new(LiteralExpr { token_type: TokenType::False, value: "".to_string() }),
    };
    let visitor = ExpressionVisitor {};
    assert!(expr.accept(Box::new(visitor)).boolean);
}

#[test]
fn string_binary_plus() {
    let expr = BinaryExpr {
        token: Token {
            token_type: TokenType::Plus,
            value: "".to_string(),
            line: 0,
        },
        lhs: Box::new(LiteralExpr { token_type: TokenType::String, value: "hello ".to_string() }),
        rhs: Box::new(LiteralExpr { token_type: TokenType::String, value: "world".to_string() }),
    };
    let visitor = ExpressionVisitor {};
    assert_eq!(expr.accept(Box::new(visitor)).str, "hello world");
}