use crate::{
    ast::ast::{Identifier, LetStatement, Node, Program, Statement,ReturnStatement},
    lexer::lexer::Lexer,
    token::token::Token,
};

#[derive(Default)]
struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer: lexer,
            current_token: None,
            peek_token: None,
            errors: vec![],
        };
        parser.next_token();
        parser.next_token();
        return parser;
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.next_token());
    }
    fn parse_program(&mut self) -> Program {
        let mut program = Program::default();
        while self.current_token != Some(Token::Eof("".to_string())) {
            let stmt = self.parse_statements();
            if stmt != Statement::Empty {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        program
    }

    fn parse_statements(&mut self) -> Statement {
        match &self.current_token {
            Some(Token::Let(_)) => {
                let let_statement = match &self.parse_let_statement() {
                    Some(val) => Statement::Let(val.clone()),
                    None => Statement::Empty,
                };
                return let_statement;
            }
            Some(Token::Return(_))=>{
                let return_statement = match &self.parse_return_statement() {
                    Some(val)=>{
                        return  Statement::Return(val.clone())
                    },
                    None=>{
                        return Statement::Empty;
                    }
                    
                };
            }
            _ => Statement::Empty,
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let mut let_stmt = LetStatement::default();
        let_stmt.token = self.current_token.clone();
        if !self.expect_peek_token(&Token::Identifier(self.peek_token.clone().unwrap().get_literal().to_string())) {
            return None;
        }
        let current_token = self.current_token.clone().unwrap();
        let literal = current_token.get_literal().to_string();
        let_stmt.name = Some(Identifier::new(current_token, literal));
        if !self.expect_peek_token(&Token::Assign(self.peek_token.clone().unwrap().get_literal().to_string())) {
            return None;
        }
        while !self.current_token_is(&Token::Semicolon(";".to_string())) {
            self.next_token();
        }

        Some(let_stmt)
    }
    fn parse_return_statement(&mut self)->Option<ReturnStatement>{
        let current_token = self.current_token.clone().unwrap();
        let return_stmt = ReturnStatement::new(current_token);
        self.next_token();

         while !self.current_token_is(&Token::Semicolon(";".to_string())) {
            self.next_token();
        }

        Some(return_stmt)
    }
    fn expect_peek_token(&mut self, token: &Token) -> bool {
        if self.peek_token_is(token) {
            self.next_token();
            return true;
        } else {
            self.peek_error(token);
            return false;
        }
    }
    fn peek_token_is(&self, token: &Token) -> bool {
        match &self.peek_token {
            Some(val) => val==token,
            None => false,
        }
    }
    fn current_token_is(&self, token: &Token) -> bool {
        match &self.current_token {
            Some(val) => val==token,
            None => false,
        }
    }
    fn Errors(&self) -> &Vec<String> {
        return &self.errors;
    }
    fn peek_error(&mut self, token: &Token) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            token,
            self.peek_token.clone().unwrap()
        );
        self.errors.push(msg);
    }
}

#[cfg(test)]
mod test {
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

            assert_eq!(program.statements.len(), 3 );
        
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
                    assert_eq!( val.token_literal().as_str(),"let");
                    let name = val.name.clone().unwrap().token_literal();
                    assert_eq!( name.as_str(), tt.expected_identifier.as_str());
                    let value = val.name.clone().unwrap().value;
                    assert_eq!( value.as_str(), tt.expected_identifier.as_str());
                }
                &Statement::Empty => {
                    panic!("Expected a let statement")
                }
                _=>{}
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

        
            assert_eq!(program.statements.len(),3);
        
        for statement in program.statements{
            if statement==Statement::Empty{
                continue;
            }   
            assert!(matches!(statement,Statement::Return(_)));
            match &statement{
                Statement::Return(return_statement)=>{
                    assert_eq!(return_statement.token_literal(),String::from("return"));
                }
                _=>{}
            }
        }
    }


     fn check_parse_errors(parser: &Parser) {
            let errors = parser.Errors();
            if errors.len() == 0 {
                return;
            }
            for error in errors {
                eprintln!("{}", error)
            }
            panic!("Parsing error detected");
        }
}
