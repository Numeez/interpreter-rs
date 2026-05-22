use crate::token::{
    self,
    token::{ASSIGN, COMMA, LBRACE, LPAREN, PLUS, RBRACE, RPAREN, SEMICOLON, Token, get_keyword},
};

#[derive(Default)]
pub struct Lexer {
    postion: usize,
    read_position: usize,
    input: String,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer::default();
        l.input = input;
        l.read_char();
        return l;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_white_space();
        let token = match &self.ch {
            b'+' => Token::Plus(PLUS.to_string()),
            b'=' => {
                if self.peek_char() == b'=' {
                    let current_ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", (current_ch as char).to_string(), (current_ch as char).to_string());
                    Token::Eq(literal)
                } else {
                    Token::Assign(ASSIGN.to_string())
                }
            }
            b',' => Token::Comma(COMMA.to_string()),
            b';' => Token::Semicolon(SEMICOLON.to_string()),
            b'{' => Token::Lbrace(LBRACE.to_string()),
            b'}' => Token::Rbrace(RBRACE.to_string()),
            b'(' => Token::Lparen(LPAREN.to_string()),
            b')' => Token::Rparen(RPAREN.to_string()),
            b'!' => {
                if self.peek_char()==b'='{
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", (ch as char).to_string(),(self.ch as char).to_string());
                    Token::NotEq(literal)
                }else{
                Token::Bang("!".to_string())
                }
            
            }
            b'-' => Token::Minus("-".to_string()),
            b'*' => Token::Asterisk("*".to_string()),
            b'/' => Token::Slash("/".to_string()),
            b'<' => Token::Lt("<".to_string()),
            b'>' => Token::Gt(">".to_string()),

            0 => Token::Eof("".to_string()),

            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    return get_keyword(literal);
                } else if is_digit(self.ch) {
                    let int_literal = self.read_number();
                    return Token::Int(int_literal);
                } else {
                    Token::Illegal(self.ch.to_string())
                }
            }
        };

        self.read_char();
        token
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.postion = self.read_position;
        self.read_position += 1;
    }
    fn read_identifier(&mut self) -> String {
        let pos = self.postion;
        while is_letter(self.ch) {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input.as_bytes()[pos..self.postion]).to_string()
    }
    fn read_number(&mut self) -> String {
        let pos = self.postion;
        while is_digit(self.ch) {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input.as_bytes()[pos..self.postion]).to_string()
    }
    fn skip_white_space(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char()
        }
    }
    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        }
        self.input.as_bytes()[self.read_position]
    }
}

fn is_letter(ch: u8) -> bool {
    b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_'
}

fn is_digit(ch: u8) -> bool {
    b'0' <= ch && ch <= b'9'
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_lex_simple_input() {
        let input = String::from("=+(){},;");
        let test_cases = vec![
            Token::Assign(ASSIGN.to_string()),
            Token::Plus(PLUS.to_string()),
            Token::Lparen(LPAREN.to_string()),
            Token::Rparen(RPAREN.to_string()),
            Token::Lbrace(LBRACE.to_string()),
            Token::Rbrace(RBRACE.to_string()),
            Token::Comma(COMMA.to_string()),
            Token::Semicolon(SEMICOLON.to_string()),
        ];
        let mut lexer = Lexer::new(input);
        for tt in test_cases {
            assert_eq!(lexer.next_token(), tt)
        }
    }

    #[test]
    fn test_lexer_with_real_like_input() {
        let input = r#"
        let five = 5; 
		let ten = 10; 
		let add = fn(x, y) { x + y ; };


		let result = add(five, ten);	
        "#;
        let mut lexer = Lexer::new(input.to_string());
        let test_cases = vec![
            Token::Let("let".to_string()),
            Token::Identifier("five".to_string()),
            Token::Assign("=".to_string()),
            Token::Int("5".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Let("let".to_string()),
            Token::Identifier("ten".to_string()),
            Token::Assign("=".to_string()),
            Token::Int("10".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Let("let".to_string()),
            Token::Identifier("add".to_string()),
            Token::Assign("=".to_string()),
            Token::Function("fn".to_string()),
            Token::Lparen("(".to_string()),
            Token::Identifier("x".to_string()),
            Token::Comma(",".to_string()),
            Token::Identifier("y".to_string()),
            Token::Rparen(")".to_string()),
            Token::Lbrace("{".to_string()),
            Token::Identifier("x".to_string()),
            Token::Plus(PLUS.to_string()),
            Token::Identifier("y".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Rbrace("}".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Let("let".to_string()),
            Token::Identifier("result".to_string()),
            Token::Assign("=".to_string()),
            Token::Identifier("add".to_string()),
            Token::Lparen("(".to_string()),
            Token::Identifier("five".to_string()),
            Token::Comma(",".to_string()),
            Token::Identifier("ten".to_string()),
            Token::Rparen(")".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Eof("".to_string()),
        ];

        for tt in test_cases {
            assert_eq!(lexer.next_token(), tt)
        }
    }

    #[test]
    fn test_lexer_against_more_single_character() {
        let input = r#"
        let five = 5; 
		let ten = 10; 
		let add = fn(x, y) { x + y ; };


		let result = add(five, ten);
        !-/*5;
		5<10>5;
        "#;
        let mut lexer = Lexer::new(input.to_string());
        let test_cases = vec![
            Token::Let("let".to_string()),
            Token::Identifier("five".to_string()),
            Token::Assign("=".to_string()),
            Token::Int("5".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Let("let".to_string()),
            Token::Identifier("ten".to_string()),
            Token::Assign("=".to_string()),
            Token::Int("10".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Let("let".to_string()),
            Token::Identifier("add".to_string()),
            Token::Assign("=".to_string()),
            Token::Function("fn".to_string()),
            Token::Lparen("(".to_string()),
            Token::Identifier("x".to_string()),
            Token::Comma(",".to_string()),
            Token::Identifier("y".to_string()),
            Token::Rparen(")".to_string()),
            Token::Lbrace("{".to_string()),
            Token::Identifier("x".to_string()),
            Token::Plus(PLUS.to_string()),
            Token::Identifier("y".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Rbrace("}".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Let("let".to_string()),
            Token::Identifier("result".to_string()),
            Token::Assign("=".to_string()),
            Token::Identifier("add".to_string()),
            Token::Lparen("(".to_string()),
            Token::Identifier("five".to_string()),
            Token::Comma(",".to_string()),
            Token::Identifier("ten".to_string()),
            Token::Rparen(")".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Bang("!".to_string()),
            Token::Minus("-".to_string()),
            Token::Slash("/".to_string()),
            Token::Asterisk("*".to_string()),
            Token::Int("5".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Int("5".to_string()),
            Token::Lt("<".to_string()),
            Token::Int("10".to_string()),
            Token::Gt(">".to_string()),
            Token::Int("5".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Eof("".to_string()),
        ];

        for tt in test_cases {
            assert_eq!(lexer.next_token(), tt)
        }
    }
    #[test]
    fn test_lexer_against_more_keywords() {
        let input = r#"
        let five = 5; 
		let ten = 10; 
		let add = fn(x, y) { x + y ; };


		let result = add(five, ten);
        !-/*5;
		5<10>5;

        if(5<10){
		return true;
		}else{
		return false;
		}

        "#;
        let mut lexer = Lexer::new(input.to_string());
        let test_cases = vec![
            Token::Let("let".to_string()),
            Token::Identifier("five".to_string()),
            Token::Assign("=".to_string()),
            Token::Int("5".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Let("let".to_string()),
            Token::Identifier("ten".to_string()),
            Token::Assign("=".to_string()),
            Token::Int("10".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Let("let".to_string()),
            Token::Identifier("add".to_string()),
            Token::Assign("=".to_string()),
            Token::Function("fn".to_string()),
            Token::Lparen("(".to_string()),
            Token::Identifier("x".to_string()),
            Token::Comma(",".to_string()),
            Token::Identifier("y".to_string()),
            Token::Rparen(")".to_string()),
            Token::Lbrace("{".to_string()),
            Token::Identifier("x".to_string()),
            Token::Plus(PLUS.to_string()),
            Token::Identifier("y".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Rbrace("}".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Let("let".to_string()),
            Token::Identifier("result".to_string()),
            Token::Assign("=".to_string()),
            Token::Identifier("add".to_string()),
            Token::Lparen("(".to_string()),
            Token::Identifier("five".to_string()),
            Token::Comma(",".to_string()),
            Token::Identifier("ten".to_string()),
            Token::Rparen(")".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Bang("!".to_string()),
            Token::Minus("-".to_string()),
            Token::Slash("/".to_string()),
            Token::Asterisk("*".to_string()),
            Token::Int("5".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Int("5".to_string()),
            Token::Lt("<".to_string()),
            Token::Int("10".to_string()),
            Token::Gt(">".to_string()),
            Token::Int("5".to_string()),
            Token::Semicolon(";".to_string()),
            Token::If("if".to_string()),
            Token::Lparen("(".to_string()),
            Token::Int("5".to_string()),
            Token::Lt("<".to_string()),
            Token::Int("10".to_string()),
            Token::Rparen(")".to_string()),
            Token::Lbrace("{".to_string()),
            Token::Return("return".to_string()),
            Token::True("true".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Rbrace("}".to_string()),
            Token::Else("else".to_string()),
            Token::Lbrace("{".to_string()),
            Token::Return("return".to_string()),
            Token::False("false".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Rbrace("}".to_string()),
            Token::Eof("".to_string()),
        ];

        for tt in test_cases {
            assert_eq!(lexer.next_token(), tt)
        }
    }
    #[test]
    fn test_lexer_against_two_characters() {
        let input = r#"
        let five = 5; 
		let ten = 10; 
		let add = fn(x, y) { x + y ; };


		let result = add(five, ten);
        !-/*5;
		5<10>5;

        if(5<10){
		return true;
		}else{
		return false;
		}

        10==10;
        9!=10;

        "#;
        let mut lexer = Lexer::new(input.to_string());
        let test_cases = vec![
            Token::Let("let".to_string()),
            Token::Identifier("five".to_string()),
            Token::Assign("=".to_string()),
            Token::Int("5".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Let("let".to_string()),
            Token::Identifier("ten".to_string()),
            Token::Assign("=".to_string()),
            Token::Int("10".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Let("let".to_string()),
            Token::Identifier("add".to_string()),
            Token::Assign("=".to_string()),
            Token::Function("fn".to_string()),
            Token::Lparen("(".to_string()),
            Token::Identifier("x".to_string()),
            Token::Comma(",".to_string()),
            Token::Identifier("y".to_string()),
            Token::Rparen(")".to_string()),
            Token::Lbrace("{".to_string()),
            Token::Identifier("x".to_string()),
            Token::Plus(PLUS.to_string()),
            Token::Identifier("y".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Rbrace("}".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Let("let".to_string()),
            Token::Identifier("result".to_string()),
            Token::Assign("=".to_string()),
            Token::Identifier("add".to_string()),
            Token::Lparen("(".to_string()),
            Token::Identifier("five".to_string()),
            Token::Comma(",".to_string()),
            Token::Identifier("ten".to_string()),
            Token::Rparen(")".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Bang("!".to_string()),
            Token::Minus("-".to_string()),
            Token::Slash("/".to_string()),
            Token::Asterisk("*".to_string()),
            Token::Int("5".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Int("5".to_string()),
            Token::Lt("<".to_string()),
            Token::Int("10".to_string()),
            Token::Gt(">".to_string()),
            Token::Int("5".to_string()),
            Token::Semicolon(";".to_string()),
            Token::If("if".to_string()),
            Token::Lparen("(".to_string()),
            Token::Int("5".to_string()),
            Token::Lt("<".to_string()),
            Token::Int("10".to_string()),
            Token::Rparen(")".to_string()),
            Token::Lbrace("{".to_string()),
            Token::Return("return".to_string()),
            Token::True("true".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Rbrace("}".to_string()),
            Token::Else("else".to_string()),
            Token::Lbrace("{".to_string()),
            Token::Return("return".to_string()),
            Token::False("false".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Rbrace("}".to_string()),
            Token::Int("10".to_string()),
            Token::Eq("==".to_string()),
            Token::Int("10".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Int("9".to_string()),
            Token::NotEq("!=".to_string()),
            Token::Int("10".to_string()),
            Token::Semicolon(";".to_string()),
            Token::Eof("".to_string()),
        ];

        for tt in test_cases {
            assert_eq!(lexer.next_token(), tt)
        }
    }
}
