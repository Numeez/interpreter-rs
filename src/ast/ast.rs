#![allow(unused_imports,dead_code,private_interfaces)]
use crate::token::token::{Token, TokenKind};



pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

#[derive(PartialEq, Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Empty,
}

impl Statement {
    fn string(&self) -> String {
        let mut out = String::new();
        match self {
            Statement::Let(val) => {
                out.push_str(val.string().as_str());
            }
            Statement::Return(val) => {
                out.push_str(val.string().as_str());
            }
            Statement::Empty => {}
        }
        out
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Expression {
    Identifier(Identifier),
}

impl Expression {
    fn string(&self) -> String {
        match self {
            Expression::Identifier(val) => val.string(),
        }
    }
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

    fn string(&self) -> String {
        let mut out = String::new();
        for statement in &self.statements {
            out.push_str(&statement.string());
        }
        return out;
    }
}

#[derive(Clone, PartialEq, Debug)]
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
        self.token.as_ref().unwrap().literal.to_string()
    }

    fn string(&self) -> String {
        format!(
            "{} {}={};",
            self.token.as_ref().unwrap().literal.to_string(),
            self.name.as_ref().unwrap().string(),
            self.value.as_ref().unwrap().string()
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Identifier {
    pub token: Token,
}
impl Identifier {
    pub fn new(token: Token) -> Self {
        return Self { token };
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn string(&self) -> String {
        self.token_literal()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ReturnStatement {
    pub token: Token,
    return_value: Option<Expression>,
}

impl ReturnStatement {
    pub fn new(token: Token) -> Self {
        Self {
            token: token,
            return_value: None,
        }
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn string(&self) -> String {
        todo!()
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_stringify_of_program() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                token: Some(Token::new(TokenKind::Let, "let")),
                name: Some(Identifier {
                    token: Token::new(TokenKind::Identifier, "myVar"),
                }),
                value: Some(Expression::Identifier(Identifier {
                    token: Token::new(TokenKind::Identifier, "anotherVar"),
                })),
            })],
        };

        assert_eq!(program.string(), "let myVar=anotherVar;")
    }
}
