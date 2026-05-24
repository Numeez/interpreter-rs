#![allow(dead_code)]


pub type TokenType = String;
pub const ILLEGAL: &str = "ILLEGAL";
pub const PLUS: &str = "+";
pub const COMMA: &str = ",";
pub const FUNCTION: &str = "FUNCTION";
pub const IDENT: &str = "IDENT";
pub const SEMICOLON: &str = ";";
pub const LET: &str = "LET";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const ASSIGN: &str = "=";

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Illegal,
    Eof,
    Int,
    Comma,
    Semicolon,
    Plus,
    Assign,
    Function,
    Let,
    Rparen,
    Lparen,
    Rbrace,
    Lbrace,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Gt,
    Lt,
    Identifier,
    If,
    Else,
    Return,
    True,
    False,
    Eq,
    NotEq,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}
impl Token {
    pub fn new(kind: TokenKind, literal: &str) -> Self {
        Self { kind:kind, literal:literal.to_string() }
    }
}

pub fn get_keyword(identifier: &str) -> Token {
    match identifier {
        "let" => Token::new(TokenKind::Let, identifier),
        "fn" => Token::new(TokenKind::Function, identifier),
        "if" => Token::new(TokenKind::If, identifier),
        "else" => Token::new(TokenKind::Else, identifier),
        "true" => Token::new(TokenKind::True, identifier),
        "false" => Token::new(TokenKind::False, identifier),
        "return" => Token::new(TokenKind::Return, identifier),

        _ => Token::new(TokenKind::Identifier, identifier),
    }
}
