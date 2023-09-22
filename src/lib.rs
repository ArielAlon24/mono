pub mod evaluator;
pub mod models;
pub mod parser;
pub mod tokenizer;

use crate::evaluator::Evaluator::Evaluator;
use crate::parser::parser::Parser;
use crate::tokenizer::tokenizer::Tokenizer;

pub fn tokenizer(code: &str) {
    let tok = Tokenizer::new(code.chars());
    for token in tok {
        match token {
            Ok(token) => println!("Token:\t{:?}", token),
            Err(error) => {
                eprintln!("Error:\t{:?}", error);
                return;
            }
        }
    }
}

pub fn parser(code: &str) {
    let tokenizer = Tokenizer::new(code.chars());
    let mut parser = Parser::new(tokenizer);
    match parser.parse() {
        Ok(expression) => println!("Ok:\t{:?}", expression),
        Err(error) => {
            eprintln!("Error:\t{:?}", error);
            return;
        }
    }
}

pub fn evaluator(code: &str) {
    let tokenizer = Tokenizer::new(code.chars());
    let mut parser = Parser::new(tokenizer);
    match parser.parse() {
        Err(error) => {
            eprintln!("Error:\t{:?}", error);
            return;
        }
        Ok(ast) => match Evaluator::evaluate(ast) {
            Err(error) => {
                eprintln!("Error:\t{:?}", error);
                return;
            }
            Ok(value) => println!("Ok:\t{:?}", value),
        },
    }
}
