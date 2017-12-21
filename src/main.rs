extern crate rustyline;
extern crate colored;

#[macro_use]
mod errors;

mod lexer;
mod parser;
mod interpreter;
mod repl;

use colored::*;

fn main() {
    println!("{}", "\nInteractive MinScheme (0.1.0) - press Ctrl+C to exit\n".blue());
    repl::start("> ", (|s, runtime| interpreter::run(&s, runtime)))
}
