use whoami::fallible;

mod lexer;
mod token;
mod repl;
mod ast;
mod parser;

use repl::repl::{start};



fn main() {
    let username = whoami::username();
    let hostname = fallible::hostname().unwrap();
    let os = whoami::devicename_os().into_string().unwrap();
    println!("Hello {} with hostname: {} using {}",username,hostname,os);
    println!("Feel free to type in commands");
    start(std::io::stdin(), std::io::stdout());
}
