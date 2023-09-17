pub mod models;
pub mod parser;
pub mod tokenizer;

use crate::parser::Parser;
use crate::tokenizer::Tokenizer;

pub fn tokenizer(code: &str) {
    let tok = Tokenizer::new(code.chars());
    for token in tok {
        match token {
            Ok(token) => println!("Token:\t{:?}", token),
            Err(error) => eprintln!("Error:\t{:?}", error),
        }
    }
}

pub fn run(code: &str) {
    let tokenizer = Tokenizer::new(code.chars());
    let mut parser = Parser::new(tokenizer);
    match parser.parse() {
        Ok(expression) => println!("Ok:\t{:?}", expression),
        Err(error) => eprintln!("Error:\t{:?}", error),
    }
}
