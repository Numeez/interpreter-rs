#![allow(unused_imports, dead_code, private_interfaces)]
use core::panic;
use std::{collections::HashMap, io::SeekFrom, sync::OnceLock};

use crate::{
    ast::ast::{
        Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement, Node,
        PrefixExpression, Program, ReturnStatement, Statement,InfixExpression
    }, lexer::lexer::Lexer, parser::parser::Precedence::LOWEST, token::token::{Token, TokenKind}
};

type PrefixParseFn = fn(&mut Parser) -> Expression;
type InfixParseFn = fn(&mut Parser, Expression) -> Expression;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Precedence {
    LOWEST,
    EQUAL,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

#[derive(Default)]
struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<TokenKind, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenKind, InfixParseFn>,
}

impl Parser {
    fn precedence()->& 'static HashMap<TokenKind,Precedence>{
        static MAP:OnceLock<HashMap<TokenKind,Precedence>> = OnceLock::new();
        MAP.get_or_init(||{
            HashMap::from(
                [
                (TokenKind::Eq,Precedence::EQUAL),
                (TokenKind::Lt,Precedence::LESSGREATER),
                (TokenKind::NotEq,Precedence::EQUAL),
                (TokenKind::Gt,Precedence::LESSGREATER),
                (TokenKind::Plus,Precedence::SUM),
                (TokenKind::Minus,Precedence::SUM),
                (TokenKind::Slash,Precedence::PRODUCT),
                (TokenKind::Asterisk,Precedence::PRODUCT),
                ]
            )
        })

    }
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer: lexer,
            current_token: None,
            peek_token: None,
            errors: vec![],
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
        parser.register_prefix_parse_fn(TokenKind::Identifier, Parser::parse_identifier);
        parser.register_prefix_parse_fn(TokenKind::Int, Parser::parse_integer_literal);
        parser.register_prefix_parse_fn(TokenKind::Minus,Parser::parse_prefix_expression);
        parser.register_prefix_parse_fn(TokenKind::Bang,Parser::parse_prefix_expression);
        parser.register_infix_parse_fn(TokenKind::Eq, Parser::parse_infix_expression);
        parser.register_infix_parse_fn(TokenKind::NotEq, Parser::parse_infix_expression);
        parser.register_infix_parse_fn(TokenKind::Gt, Parser::parse_infix_expression);
        parser.register_infix_parse_fn(TokenKind::Lt, Parser::parse_infix_expression);
        parser.register_infix_parse_fn(TokenKind::Plus, Parser::parse_infix_expression);
        parser.register_infix_parse_fn(TokenKind::Minus, Parser::parse_infix_expression);
        parser.register_infix_parse_fn(TokenKind::Slash, Parser::parse_infix_expression);
        parser.register_infix_parse_fn(TokenKind::Asterisk, Parser::parse_infix_expression);
        parser.next_token();
        parser.next_token();
        return parser;
    }

    fn register_prefix_parse_fn(&mut self, token_kind: TokenKind, func: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_kind, func);
    }
    fn register_infix_parse_fn(&mut self, token_kind: TokenKind, func: InfixParseFn) {
        self.infix_parse_fns.insert(token_kind, func);
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.next_token());
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program::default();
        while self.current_token != Some(Token::new(TokenKind::Eof, "")) {
            let stmt = self.parse_statements();
            if stmt != Statement::Empty {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        program
    }

    fn parse_identifier(&mut self) -> Expression {
        Expression::Identifier(Identifier {
            token: self.current_token.clone().unwrap(),
        })
    }

    fn parse_integer_literal(&mut self) -> Expression {
        let integer_literal = self.get_integer_literal().unwrap();
        Expression::Integer(integer_literal)
    }
    fn parse_prefix_expression(&mut self) -> Expression {
        let curren_token = self.current_token.clone().unwrap();
        let mut prefix_expression = PrefixExpression {
            operator: curren_token.literal.clone(),
            token:curren_token,
            right: None,
        };
        self.next_token();
        prefix_expression.right = self.parse_expression(LOWEST).map(Box::new);

        Expression::PrefixStatement(prefix_expression)
    }
    fn get_integer_literal(&self) -> Option<IntegerLiteral> {
        let (token, value) = match &self.current_token {
            Some(token) => match token.literal.parse::<i64>() {
                Ok(val) => (token, val),
                Err(_) => {
                    return None;
                }
            },
            None => {
                return None;
            }
        };
        let integer = IntegerLiteral {
            token: token.clone(),
            value,
        };
        Some(integer)
    }
    fn parse_statements(&mut self) -> Statement {
        match self.current_token.as_ref().unwrap().kind {
            TokenKind::Let => {
                let let_statement = match &self.parse_let_statement() {
                    Some(val) => Statement::Let(val.clone()),
                    None => Statement::Empty,
                };
                return let_statement;
            }
            TokenKind::Return => {
                let return_statement = match &self.parse_return_statement() {
                    Some(val) => Statement::Return(val.clone()),
                    None => Statement::Empty,
                };
                return return_statement;
            }
            _ => {
                let expression_statement = match &self.parse_expression_statement() {
                    Some(val) => Statement::Expression(val.clone()),
                    None => Statement::Empty,
                };
                return expression_statement;
            }
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let mut let_stmt = LetStatement::default();
        let_stmt.token = self.current_token.clone();
        if !self.expect_peek_token(&TokenKind::Identifier) {
            return None;
        }
        let current_token = self.current_token.clone().unwrap();
        let_stmt.name = Some(Identifier::new(current_token));
        if !self.expect_peek_token(&TokenKind::Assign) {
            return None;
        }
        while !self.current_token_is(&TokenKind::Semicolon) {
            self.next_token();
        }

        Some(let_stmt)
    }
    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        let current_token = self.current_token.clone().unwrap();
        let return_stmt = ReturnStatement::new(current_token);
        self.next_token();

        while !self.current_token_is(&TokenKind::Semicolon) {
            self.next_token();
        }

        Some(return_stmt)
    }
    fn parse_expression_statement(&mut self) -> Option<ExpressionStatement> {
        let expression_statement = ExpressionStatement {
            token: self.current_token.clone().unwrap(),
            expression: self.parse_expression(Precedence::LOWEST),
        };
        if self.peek_token_is(&TokenKind::Semicolon) {
            self.next_token();
        }

        Some(expression_statement)
    }
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let parse_function = self
            .prefix_parse_fns
            .get(&self.current_token.as_ref().unwrap().kind);
        let mut left_exp = match parse_function {
            Some(function) => {
                let expression = function(self);
                 Some(expression)
            }
            None => {
                 None
            }
        };
        while !self.peek_token_is(&TokenKind::Semicolon) && precedence<self.peek_precedence(){
            let func = self.infix_parse_fns.get(&self.peek_token.as_ref().unwrap().kind).copied();
            match func{
                Some(function)=>{
                    self.next_token();
                    left_exp = Some(function(self,left_exp.unwrap()))
                },
                None=>{
                    return left_exp;
                }
            }
            }

        left_exp

    }
    fn expect_peek_token(&mut self, token: &TokenKind) -> bool {
        if self.peek_token_is(token) {
            self.next_token();
            return true;
        } else {
            self.peek_error(token);
            return false;
        }
    }
    fn peek_token_is(&self, token: &TokenKind) -> bool {
        return &self.peek_token.as_ref().unwrap().kind == token;
    }
    fn current_token_is(&self, token: &TokenKind) -> bool {
        return &self.current_token.as_ref().unwrap().kind == token;
    }

    fn errors(&self) -> &Vec<String> {
        return &self.errors;
    }
    fn peek_error(&mut self, token: &TokenKind) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            token,
            self.peek_token.clone().unwrap()
        );
        self.errors.push(msg);
    }

    fn parse_infix_expression(&mut self,left: Expression)->Expression{
        let current_token = self.current_token.clone().unwrap();
        let mut infix_expression = InfixExpression{
            operator:current_token.literal.to_string(),
            token:current_token,
            left:Some(Box::from(left)),
            right:None

        } ;  
        let precedence = self.current_precedence();
        self.next_token();
        infix_expression.right = self.parse_expression(precedence).map(Box::new);
        Expression::InfixExpression(infix_expression)
    }

    fn peek_precedence(&self)->Precedence{
        let precdence_map = Parser::precedence();
        match precdence_map.get(&self.peek_token.as_ref().unwrap().kind){
        Some(val)=>{
                val.clone()
            }
            ,
            None=>{
                Precedence::LOWEST
            }
        }  

    }
    fn current_precedence(&self)-> Precedence{
        let precdence_map = Parser::precedence();
        match precdence_map.get(&self.current_token.as_ref().unwrap().kind){
        Some(val)=>{
                val.clone()
            }
            ,
            None=>{
                Precedence::LOWEST
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::arch::aarch64::int64x1_t;

    use crate::lexer::lexer;

    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r#"
        let x  =  5;
        let y = 10;
        let foobar = 17171;


        "#;

        let l = Lexer::new(input.to_string());
        let mut parser = Parser::new(l);
        let program = parser.parse_program();

        check_parse_errors(&parser);

        assert_eq!(program.statements.len(), 3);

        struct TestCases {
            expected_identifier: String,
        }

        let tests = vec![
            TestCases {
                expected_identifier: String::from("x"),
            },
            TestCases {
                expected_identifier: String::from("y"),
            },
            TestCases {
                expected_identifier: String::from("foobar"),
            },
        ];

        for (i, tt) in tests.iter().enumerate() {
            assert!(!program.statements.get(i).is_none());

            let stmt = program.statements.get(i).unwrap();
            if std::mem::discriminant(stmt)
                != std::mem::discriminant(&Statement::Let(LetStatement::default()))
            {
                panic!("Expected a let statement")
            }
            match &stmt {
                &Statement::Let(val) => {
                    assert_eq!(val.token_literal().as_str(), "let");
                    let name = val.name.as_ref().unwrap().token_literal();
                    assert_eq!(name.as_str(), tt.expected_identifier.as_str());
                    let value = val.name.as_ref().unwrap().token_literal();
                    assert_eq!(value.as_str(), tt.expected_identifier.as_str());
                }
                &Statement::Empty => {
                    panic!("Expected a let statement")
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_return_statements() {
        let input = r#"
        return 5;
        return 10;
        return 6969;


        "#;

        let l = Lexer::new(input.to_string());
        let mut parser = Parser::new(l);
        let program = parser.parse_program();

        check_parse_errors(&parser);

        assert_eq!(program.statements.len(), 3);

        for statement in program.statements {
            if statement == Statement::Empty {
                continue;
            }
            assert!(matches!(statement, Statement::Return(_)));
            match &statement {
                Statement::Return(return_statement) => {
                    assert_eq!(return_statement.token_literal(), String::from("return"));
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parse_errors(&p);
        assert_eq!(program.statements.len(), 1);
        let stmt = program.statements.get(0).unwrap();
        assert!(matches!(stmt, Statement::Expression(_)));
        match stmt {
            Statement::Expression(val) => {
                assert!(matches!(
                    val.expression.as_ref().unwrap(),
                    &Expression::Identifier(_)
                ));
                match &val.expression.as_ref().unwrap() {
                    Expression::Identifier(val) => {
                        assert_eq!(&val.token_literal(), "foobar")
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    #[test]
    fn test_integer_literal() {
        let input = "5;";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parse_errors(&p);
        assert_eq!(program.statements.len(), 1);
        let stmt = program.statements.get(0).unwrap();
        assert!(matches!(stmt, Statement::Expression(_)));
        match stmt {
            Statement::Expression(val) => {
                assert!(matches!(
                    val.expression.as_ref().unwrap(),
                    &Expression::Integer(_)
                ));
                match val.expression.as_ref().unwrap() {
                    Expression::Integer(val) => {
                        assert_eq!(&val.token_literal(), "5");
                        assert_eq!(val.value, 5i64);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    #[test]
    fn test_prefix_expression() {
        struct test_cases {
            input: String,
            output: i64,
            operator: String,
        }
        impl test_cases {
            fn new(input: &str, operator: &str, output: i64) -> Self {
                Self {
                    input: input.to_string(),
                    operator: operator.to_string(),
                    output: output,
                }
            }
        }
        let test_cases = vec![
            test_cases::new("!5;", "!", 5),
            test_cases::new("-15;", "-", 15),
        ];
        for tt in test_cases {
            let l = Lexer::new(tt.input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parse_errors(&p);
            assert_eq!(program.statements.len(), 1);
            let stmt = program.statements.get(0).unwrap();
            match stmt {
                Statement::Expression(val) => match &val.expression.as_ref().unwrap() {
                    Expression::PrefixStatement(prefix_expression) => {
                        assert_eq!(prefix_expression.operator, tt.operator);
                        let right_expression = *prefix_expression.right.clone().unwrap();
                        match right_expression {
                            Expression::Integer(val) => {
                                assert_eq!(val.value, tt.output);
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        panic!("expression should have been an prefix expression")
                    }
                },
                _ => {
                    panic!("statement should have been an expression")
                }
            }
        }
    }

    #[test]
    fn test_infix_expression(){
        #[derive(Default,Eq,PartialEq, PartialOrd, Ord,Hash)]
        struct infix_tests{
            input:String,
            left: i64,
            operator:String,
            right:i64
        }
        impl infix_tests{
            fn new(input:&str,left:i64,operator:&str,right:i64)->Self{
                Self { input: input.to_string(), left: left, operator: operator.to_string(), right: right }
            }
        }
        let tests:Vec<infix_tests> = vec![
        infix_tests::new("5-5;", 5, "-", 5),
        infix_tests::new("5+5;", 5, "+", 5),
        infix_tests::new("5/5;", 5, "/", 5),
        infix_tests::new("5*5;", 5, "*", 5),
        infix_tests::new("5>5;", 5, ">", 5),
        infix_tests::new("5<5;", 5, "<", 5),
        infix_tests::new("5==5;", 5, "==", 5),
        infix_tests::new("5!=5;", 5, "!=", 5)
        ];

        for test in tests{
            let l = Lexer::new(test.input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parse_errors(&p);
            assert_eq!(program.statements.len(),1);
            let stmt = program.statements.get(0).unwrap();
            match stmt {
                Statement::Expression(val) => match &val.expression.as_ref().unwrap() {
                    Expression::InfixExpression(prefix_expression) => {
                        assert_eq!(prefix_expression.operator,test.operator);
                        assert!(assert_integer_literal(prefix_expression.left.as_ref().unwrap(),test.left));
                        assert!(assert_integer_literal(prefix_expression.right.as_ref().unwrap(),test.right));
                    },
                    _ =>{}

                }
                _ =>{

                }
            }

        }

    }

    fn assert_integer_literal(expression:&Expression,expected_val:i64)->bool{
        match expression {
            Expression::Integer(val)=>{
                if val.value!=expected_val{
                    return false;
                }
                if val.token_literal()!=format!("{}",val.value){
                    return false;
                }
                return true;
            },
            _ =>{
                return false;
            }
        }
       
         
        }
         fn check_parse_errors(parser: &Parser) {
        let errors = parser.errors();
        if errors.len() == 0 {
            return;
        }
        for error in errors {
            eprintln!("{}", error)
        }
        panic!("Parsing error detected");
    }

}

   

