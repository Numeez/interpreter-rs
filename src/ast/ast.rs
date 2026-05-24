use std::io::Empty;

use crate::token::token::Token;
pub trait Node {
    fn token_literal(&self) -> String;
    fn String(&self)->String;
}

#[derive(PartialEq,Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Empty,
}

impl Statement{
    fn string(&self)->String{
        let mut out = String::new();
        match self{
            Statement::Let(val)=>{
                out.push_str(val.String().as_str());
            }
            Statement::Return(val)=>{
                out.push_str(val.String().as_str());
            }
            Statement::Empty=>{}
        }
        out
    }
}

#[derive(Clone, PartialEq,Debug)]
enum Expression {
    Identifier(Identifier),
}


impl Expression{
    fn string(&self)->String{
      match  self {
          Expression::Identifier(val)=>{
                    val.String()
          }
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
    
    fn String(&self)->String {
        let mut out = String::new();
        for statement in &self.statements{
            out.push_str(&statement.string());
        }
        return out;
    }
}

#[derive(Clone, PartialEq,Debug,)]
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
    
    fn String(&self)->String {
        format!("{} {}={};",self.token.clone().unwrap().get_literal().to_string(),self.name.clone().unwrap().String(),self.value.clone().unwrap().string())   
    }
}

#[derive(Clone, PartialEq,Debug)]
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
    
    fn String(&self)->String {
        self.value.clone()
    }
}



#[derive(Clone,PartialEq,Debug)]
pub struct ReturnStatement{
    pub token: Token,
    return_value: Option<Expression>
}

impl ReturnStatement{
    pub fn new(token:Token)->Self{
        Self { token:token, return_value:None}
    }
}

impl Node for ReturnStatement{
    fn token_literal(&self) -> String {
        self.token.get_literal().to_string()
    }
    
    fn String(&self)->String {
        todo!()
    }
}


#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_stringify_of_program(){
        let program = Program{
            statements:vec![
                Statement::Let(
                    LetStatement { 
                        token: Some(Token::Let("let".to_string())),
                         name: Some(Identifier { token: Token::Identifier("myVar".to_string()), value:"myVar".to_string() }), 
                         value: Some(Expression::Identifier(Identifier { token: Token::Identifier("anotherVar".to_string()), value:"anotherVar".to_string() })) }
                ),
            ]
        };

        assert_eq!(program.String(),"let myVar=anotherVar;".to_string())
    }
}