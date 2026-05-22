use std::collections::HashMap;

pub type TokenType = String;

pub const ILLEGAL: &str = "ILLEGAL";
pub const PLUS: &str = "+";
pub const COMMA: &str = ",";
pub const FUNCTION: &str = "FUNCTION";
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";
pub const SEMICOLON: &str = ";";
pub const LET: &str = "LET";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const ASSIGN: &str = "=";

#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(TokenType),
    Eof(TokenType),
    Int(TokenType),
    Comma(TokenType),
    Semicolon(TokenType),
    Plus(TokenType),
    Assign(TokenType),
    Function(TokenType),
    Let(TokenType),
    Rparen(TokenType),
    Lparen(TokenType),
    Rbrace(TokenType),
    Lbrace(TokenType),
    Minus(TokenType),
    Bang(TokenType),
    Asterisk(TokenType),
    Slash(TokenType),
    Gt(TokenType),
    Lt(TokenType),
    Identifier(TokenType),
    If(TokenType),
    Else(TokenType),
    Return(TokenType),
    True(TokenType),
    False(TokenType),
    Eq(TokenType),
    NotEq(TokenType),
}

pub fn get_keyword(identifier: String) -> Token {
    match identifier.as_str() {
        "let" => Token::Let(identifier),
        "fn" => Token::Function(identifier),
        "if" => Token::If(identifier),
        "else" => Token::Else(identifier),
        "true" => Token::True(identifier),
        "false" => Token::False(identifier),
        "return" => Token::Return(identifier),

        _ => Token::Identifier(identifier),
    }
}
