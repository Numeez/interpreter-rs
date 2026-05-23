use whoami::fallible;

mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

use repl::repl::start;

fn main() {
    let username = whoami::username();
    let hostname = fallible::hostname().unwrap();
    let os = whoami::devicename_os().into_string().unwrap();
    println!(
        "Hello {} with hostname: {} using {}",
        username, hostname, os
    );
    println!("Feel free to type in commands");
    start(std::io::stdin(), std::io::stdout());
}
