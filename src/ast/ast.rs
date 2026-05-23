use crate::token::token::Token;
pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Empty,
}

#[derive(Clone, PartialEq)]
enum Expression {
    Identifier(Identifier),
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Default for Program {
    fn default() -> Self {
        Self { statements: vec![] }
    }
}
impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            match &self.statements.get(0) {
                Some(val) => match val {
                    Statement::Let(s) => s.token_literal(),
                    _ => String::from(""),
                },
                None => {
                    return String::from("");
                }
            }
        } else {
            return String::from("");
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct LetStatement {
    pub token: Option<Token>,
    pub name: Option<Identifier>,
    pub value: Option<Expression>,
}

impl Default for LetStatement {
    fn default() -> Self {
        Self {
            token: None,
            name: None,
            value: None,
        }
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        match &self.token {
            Some(val) => val.get_literal().to_string(),
            None => String::from(""),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}
impl Identifier {
    pub fn new(token: Token, value: String) -> Self {
        return Self { token, value };
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.get_literal().to_string()
    }
}
