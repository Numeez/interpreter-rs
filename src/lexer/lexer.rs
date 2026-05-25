use crate::token::token::{
    ASSIGN, COMMA, LBRACE, LPAREN, PLUS, RBRACE, RPAREN, SEMICOLON, Token, TokenKind, get_keyword,
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
            b'+' => Token::new(TokenKind::Plus, PLUS),
            b'=' => {
                if self.peek_char() == b'=' {
                    let current_ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", (current_ch as char), (current_ch as char));
                    Token::new(TokenKind::Eq, literal.as_str())
                } else {
                    Token::new(TokenKind::Assign, ASSIGN)
                }
            }
            b',' => Token::new(TokenKind::Comma, COMMA),
            b';' => Token::new(TokenKind::Semicolon, SEMICOLON),
            b'{' => Token::new(TokenKind::Lbrace, LBRACE),
            b'}' => Token::new(TokenKind::Rbrace, RBRACE),
            b'(' => Token::new(TokenKind::Lparen, LPAREN),
            b')' => Token::new(TokenKind::Rparen, RPAREN),
            b'!' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", (ch as char), (self.ch as char));
                    Token::new(TokenKind::NotEq, literal.as_str())
                } else {
                    Token::new(TokenKind::Bang, "!")
                }
            }
            b'-' => Token::new(TokenKind::Minus, "-"),
            b'*' => Token::new(TokenKind::Asterisk, "*"),
            b'/' => Token::new(TokenKind::Slash, "/"),
            b'<' => Token::new(TokenKind::Lt, "<"),
            b'>' => Token::new(TokenKind::Gt, ">"),

            0 => Token::new(TokenKind::Eof, ""),

            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    return get_keyword(literal.as_str());
                } else if is_digit(self.ch) {
                    let int_literal = self.read_number();
                    return Token::new(TokenKind::Int, int_literal.as_str());
                } else {
                    Token::new(TokenKind::Illegal, &(self.ch as char).to_string())
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
            Token::new(TokenKind::Assign, ASSIGN),
            Token::new(TokenKind::Plus, PLUS),
            Token::new(TokenKind::Lparen, LPAREN),
            Token::new(TokenKind::Rparen, RPAREN),
            Token::new(TokenKind::Lbrace, LBRACE),
            Token::new(TokenKind::Rbrace, RBRACE),
            Token::new(TokenKind::Comma, COMMA),
            Token::new(TokenKind::Semicolon, SEMICOLON),
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
            Token::new(TokenKind::Let, "let"),
            Token::new(TokenKind::Identifier, "five"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Int, "5"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Let, "let"),
            Token::new(TokenKind::Identifier, "ten"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Int, "10"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Let, "let"),
            Token::new(TokenKind::Identifier, "add"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Function, "fn"),
            Token::new(TokenKind::Lparen, "("),
            Token::new(TokenKind::Identifier, "x"),
            Token::new(TokenKind::Comma, ","),
            Token::new(TokenKind::Identifier, "y"),
            Token::new(TokenKind::Rparen, ")"),
            Token::new(TokenKind::Lbrace, "{"),
            Token::new(TokenKind::Identifier, "x"),
            Token::new(TokenKind::Plus, "+"),
            Token::new(TokenKind::Identifier, "y"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Rbrace, "}"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Let, "let"),
            Token::new(TokenKind::Identifier, "result"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Identifier, "add"),
            Token::new(TokenKind::Lparen, "("),
            Token::new(TokenKind::Identifier, "five"),
            Token::new(TokenKind::Comma, ","),
            Token::new(TokenKind::Identifier, "ten"),
            Token::new(TokenKind::Rparen, ")"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Eof, ""),
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
            Token::new(TokenKind::Let, "let"),
            Token::new(TokenKind::Identifier, "five"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Int, "5"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Let, "let"),
            Token::new(TokenKind::Identifier, "ten"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Int, "10"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Let, "let"),
            Token::new(TokenKind::Identifier, "add"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Function, "fn"),
            Token::new(TokenKind::Lparen, "("),
            Token::new(TokenKind::Identifier, "x"),
            Token::new(TokenKind::Comma, ","),
            Token::new(TokenKind::Identifier, "y"),
            Token::new(TokenKind::Rparen, ")"),
            Token::new(TokenKind::Lbrace, "{"),
            Token::new(TokenKind::Identifier, "x"),
            Token::new(TokenKind::Plus, "+"),
            Token::new(TokenKind::Identifier, "y"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Rbrace, "}"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Let, "let"),
            Token::new(TokenKind::Identifier, "result"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Identifier, "add"),
            Token::new(TokenKind::Lparen, "("),
            Token::new(TokenKind::Identifier, "five"),
            Token::new(TokenKind::Comma, ","),
            Token::new(TokenKind::Identifier, "ten"),
            Token::new(TokenKind::Rparen, ")"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Bang, "!"),
            Token::new(TokenKind::Minus, "-"),
            Token::new(TokenKind::Slash, "/"),
            Token::new(TokenKind::Asterisk, "*"),
            Token::new(TokenKind::Int, "5"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Int, "5"),
            Token::new(TokenKind::Lt, "<"),
            Token::new(TokenKind::Int, "10"),
            Token::new(TokenKind::Gt, ">"),
            Token::new(TokenKind::Int, "5"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Eof, ""),
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
            Token::new(TokenKind::Let, "let"),
            Token::new(TokenKind::Identifier, "five"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Int, "5"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Let, "let"),
            Token::new(TokenKind::Identifier, "ten"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Int, "10"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Let, "let"),
            Token::new(TokenKind::Identifier, "add"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Function, "fn"),
            Token::new(TokenKind::Lparen, "("),
            Token::new(TokenKind::Identifier, "x"),
            Token::new(TokenKind::Comma, ","),
            Token::new(TokenKind::Identifier, "y"),
            Token::new(TokenKind::Rparen, ")"),
            Token::new(TokenKind::Lbrace, "{"),
            Token::new(TokenKind::Identifier, "x"),
            Token::new(TokenKind::Plus, "+"),
            Token::new(TokenKind::Identifier, "y"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Rbrace, "}"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Let, "let"),
            Token::new(TokenKind::Identifier, "result"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Identifier, "add"),
            Token::new(TokenKind::Lparen, "("),
            Token::new(TokenKind::Identifier, "five"),
            Token::new(TokenKind::Comma, ","),
            Token::new(TokenKind::Identifier, "ten"),
            Token::new(TokenKind::Rparen, ")"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Bang, "!"),
            Token::new(TokenKind::Minus, "-"),
            Token::new(TokenKind::Slash, "/"),
            Token::new(TokenKind::Asterisk, "*"),
            Token::new(TokenKind::Int, "5"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Int, "5"),
            Token::new(TokenKind::Lt, "<"),
            Token::new(TokenKind::Int, "10"),
            Token::new(TokenKind::Gt, ">"),
            Token::new(TokenKind::Int, "5"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::If, "if"),
            Token::new(TokenKind::Lparen, "("),
            Token::new(TokenKind::Int, "5"),
            Token::new(TokenKind::Lt, "<"),
            Token::new(TokenKind::Int, "10"),
            Token::new(TokenKind::Rparen, ")"),
            Token::new(TokenKind::Lbrace, "{"),
            Token::new(TokenKind::Return, "return"),
            Token::new(TokenKind::True, "true"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Rbrace, "}"),
            Token::new(TokenKind::Else, "else"),
            Token::new(TokenKind::Lbrace, "{"),
            Token::new(TokenKind::Return, "return"),
            Token::new(TokenKind::False, "false"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Rbrace, "}"),
            Token::new(TokenKind::Eof, ""),
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
            Token::new(TokenKind::Let, "let"),
            Token::new(TokenKind::Identifier, "five"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Int, "5"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Let, "let"),
            Token::new(TokenKind::Identifier, "ten"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Int, "10"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Let, "let"),
            Token::new(TokenKind::Identifier, "add"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Function, "fn"),
            Token::new(TokenKind::Lparen, "("),
            Token::new(TokenKind::Identifier, "x"),
            Token::new(TokenKind::Comma, ","),
            Token::new(TokenKind::Identifier, "y"),
            Token::new(TokenKind::Rparen, ")"),
            Token::new(TokenKind::Lbrace, "{"),
            Token::new(TokenKind::Identifier, "x"),
            Token::new(TokenKind::Plus, "+"),
            Token::new(TokenKind::Identifier, "y"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Rbrace, "}"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Let, "let"),
            Token::new(TokenKind::Identifier, "result"),
            Token::new(TokenKind::Assign, "="),
            Token::new(TokenKind::Identifier, "add"),
            Token::new(TokenKind::Lparen, "("),
            Token::new(TokenKind::Identifier, "five"),
            Token::new(TokenKind::Comma, ","),
            Token::new(TokenKind::Identifier, "ten"),
            Token::new(TokenKind::Rparen, ")"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Bang, "!"),
            Token::new(TokenKind::Minus, "-"),
            Token::new(TokenKind::Slash, "/"),
            Token::new(TokenKind::Asterisk, "*"),
            Token::new(TokenKind::Int, "5"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Int, "5"),
            Token::new(TokenKind::Lt, "<"),
            Token::new(TokenKind::Int, "10"),
            Token::new(TokenKind::Gt, ">"),
            Token::new(TokenKind::Int, "5"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::If, "if"),
            Token::new(TokenKind::Lparen, "("),
            Token::new(TokenKind::Int, "5"),
            Token::new(TokenKind::Lt, "<"),
            Token::new(TokenKind::Int, "10"),
            Token::new(TokenKind::Rparen, ")"),
            Token::new(TokenKind::Lbrace, "{"),
            Token::new(TokenKind::Return, "return"),
            Token::new(TokenKind::True, "true"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Rbrace, "}"),
            Token::new(TokenKind::Else, "else"),
            Token::new(TokenKind::Lbrace, "{"),
            Token::new(TokenKind::Return, "return"),
            Token::new(TokenKind::False, "false"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Rbrace, "}"),
            Token::new(TokenKind::Int, "10"),
            Token::new(TokenKind::Eq, "=="),
            Token::new(TokenKind::Int, "10"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Int, "9"),
            Token::new(TokenKind::NotEq, "!="),
            Token::new(TokenKind::Int, "10"),
            Token::new(TokenKind::Semicolon, ";"),
            Token::new(TokenKind::Eof, ""),
        ];
        for tt in test_cases {
            assert_eq!(lexer.next_token(), tt)
        }
    }
}
