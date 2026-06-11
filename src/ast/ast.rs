#![allow(unused_imports, dead_code, private_interfaces)]
use std::fmt::Write;

use crate::token::token::{Token, TokenKind};

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

#[derive(PartialEq, Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
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
            Statement::Expression(val) => {
                out.push_str(val.string().as_str());
            }
            Statement::Empty => {}
        }
        out
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expression {
    Identifier(Identifier),
    Integer(IntegerLiteral),
    PrefixStatement(PrefixExpression),
    InfixExpression(InfixExpression),
    BooleanExp(BooleanExpression)
}

impl Expression {
    fn string(&self) -> String {
        match self {
            Expression::Identifier(val) => val.string(),
            Expression::Integer(val) => val.string(),
            Expression::PrefixStatement(val) => val.string(),
            Expression::InfixExpression(val) => val.string(),
            Expression::BooleanExp(val)=>val.string(),
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

#[derive(Clone, PartialEq, Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Expression>,
}
impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn string(&self) -> String {
        self.expression.as_ref().unwrap().string()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Option<Box<Expression>>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("(");
        out.push_str(&self.operator.as_str());
        out.push_str(&self.right.clone().unwrap().string());
        out.push_str(")");
        out
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct BooleanExpression{
    pub   token: Token,
    pub value:bool
}

impl Node for BooleanExpression{
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn string(&self) -> String {
        self.token.literal.to_string()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Option<Box<Expression>>,
    pub operator: String,
    pub right: Option<Box<Expression>>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("(");
        out.push_str(&self.left.clone().unwrap().string());
        out.push_str(&format!(" {} ", self.operator));
        out.push_str(&self.right.clone().unwrap().string());
        out.push_str(")");
        out
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}
impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn string(&self) -> String {
        self.token.literal.to_string()
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


