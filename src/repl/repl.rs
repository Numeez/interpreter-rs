use crate::{lexer::lexer::Lexer, token::token::Token};
use std::io::{BufRead, BufReader, Read, Write};

const PROMPT: &str = ">> ";

pub fn start(input: impl Read, output: impl Write) {
    let mut out = output;
    let mut reader = BufReader::new(input);
    loop {
        write!(out, "{}", PROMPT).unwrap();
        out.flush().unwrap();
        let mut line = String::new();
        if reader.read_line(&mut line).unwrap() == 0 {
            break;
        }
        let mut lexer = Lexer::new(line);
        loop {
            let token = lexer.next_token();
            if token == Token::Eof("".into()) {
                break;
            }
            writeln!(out, "{:?}", token).unwrap();
        }
    }
}
