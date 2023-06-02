use std::io;

mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

fn main() {
    println!("Welcome to the monkeyball programming language!");
    println!("Feel free to type in commands...");
    repl::start(&mut io::stdin(), &mut io::stdout());
}
