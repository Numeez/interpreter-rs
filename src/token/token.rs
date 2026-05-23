
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

#[derive(Debug, PartialEq,Clone)]
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

impl Token {
    pub fn get_literal(&self)->&String{
        match self {
         Token::Assign(s)=>s,
         Token::Illegal(s)=>s,
         Token::If(s)=>s,
         Token::Identifier(s)=>s,
         Token::Eof(s)=>s,
         Token::Plus(s)=>s,
         Token::Function(s)=>s,
         Token::Lbrace(s)=>s,
         Token::Rbrace(s)=>s,
         Token::Lparen(s)=>s,
         Token::Rparen(s)=>s,
         Token::Minus(s)=>s,
         Token::Slash(s)=>s,
         Token::Asterisk(s)=>s,
         Token::Gt(s)=>s,
         Token::Lt(s)=>s,
         Token::Comma(s)=>s,
         Token::Semicolon(s)=>s,
         Token::Else(s)=>s,
         Token::Return(s)=>s,
         Token::True(s)=>s,
         Token::False(s)=>s,
         Token::Int(s)=>s,
         Token::Bang(s)=>s,
         Token::Let(s)=>s,
         Token::NotEq(s)=>s,
         Token::Eq(s)=>s,


        }
    }
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
