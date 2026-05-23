use crate::{ast::ast::{Program,Statement,LetStatement,Identifier,Node}, lexer::lexer::Lexer, token::token::Token};

#[derive(Default)]
struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser{
    pub fn new(lexer:Lexer)->Self{
        let mut parser = Self{
            lexer:lexer,
            current_token:None,
            peek_token:None
        };
        parser.next_token();
        parser.next_token();
        return parser;
    }

    fn next_token(&mut self){
        self.current_token =  self.peek_token.take();
        self.peek_token = Some(self.lexer.next_token());
    }
    fn parse_program(&mut self)->Program{
        let mut program = Program::default();
        while self.current_token!=Some(Token::Eof("".to_string())){
            let stmt = self.parse_statements();
            if stmt!=Statement::Empty{
                program.statements.push(stmt);       
            }
            self.next_token();
        }
        program

    }

    fn parse_statements(&mut self)->Statement{
        match &self.current_token{
            Some(Token::Let(_))=>{
                  return  Statement::Let(self.parse_let_statement().clone().unwrap());
            }
            _ =>{
                Statement::Empty
            }
        }
    }

    fn parse_let_statement(&mut self)->Option<LetStatement>{
        let mut let_stmt = LetStatement::default();
        let_stmt.token  = self.current_token.clone();
        if !self.expect_peek_token(&Token::Identifier(String::new())){
           return None;
        }
        let current_token = self.current_token.clone().unwrap();
        let literal = current_token.get_literal().to_string();
        let_stmt.name = Some(Identifier::new(current_token,literal ));
        if !self.expect_peek_token(&Token::Assign(String::new())){
            return None;
        }
        while  !self.current_token_is(&Token::Semicolon(";".to_string())) {
            self.next_token();
        }

        Some(let_stmt)

    }
    fn expect_peek_token(&mut self,token:&Token)->bool{
        if self.peek_token_is(token){
            self.next_token();
            return true; 
        }else{
            return false;
        }
    }
    fn peek_token_is(&self,token:&Token)->bool{
        match  &self.peek_token {
            Some(val)=>{
                std::mem::discriminant(val)==std::mem::discriminant(token)
            },
            None=>{
                  false
            }
        }
    }
    fn current_token_is(&self,token:&Token)->bool{
        match  &self.current_token {
            Some(val)=>{
                std::mem::discriminant(val)==std::mem::discriminant(token)
            },
            None=>{
                  false
            }
        }
    }
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_let_statements(){
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 17171;


        "#;

        let l = Lexer::new(input.to_string());
        let mut parser = Parser::new(l);
        let program = parser.parse_program();
        if program.statements.len()!=3{
            panic!("program statements are not equal to 3: {}",program.statements.len())
        }
        struct TestCases{
            expected_identifier:String
        }

        let tests= vec![
           TestCases{expected_identifier:String::from("x")},
           TestCases{expected_identifier:String::from("y")},
           TestCases{expected_identifier:String::from("foobar")},
        ];

        for (i,tt) in tests.iter().enumerate(){
           if  program.statements.get(i).is_none(){
            panic!("statement should not have been none")
           }
           let stmt = program.statements.get(i).unwrap();
           if std::mem::discriminant(stmt)!=std::mem::discriminant(&Statement::Let(LetStatement::default())){
                panic!("Expected a let statement")
           }
           match &stmt{
            &Statement::Let(val)=>{
                if val.token_literal()!=String::from("let"){
                    panic!("s.TokenLiteral not 'let'. got={}", val.token_literal())
                }
                let name = val.name.clone().unwrap().token_literal();
                if name!=tt.expected_identifier{
                        panic!("letStmt.Name.Value not '{}'. got= {}", tt.expected_identifier, name)
                }
                let value = val.name.clone().unwrap().value;
                if value!=tt.expected_identifier{
                    panic!("letStmt.Name.Value not '{}'. got= {}", tt.expected_identifier, value)
                }

            }
            &Statement::Empty=>{
                panic!("Expected a let statement")
            }
           } 
           
        }



    }


}
