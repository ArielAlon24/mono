pub mod error;
pub mod token;
pub mod tokenizer;

use crate::tokenizer::Tokenizer;
use std::process::exit;

pub fn run(code: &str) {
    let tokenizer = Tokenizer::new(code.chars());

    for token in tokenizer {
        match token {
            Ok(token) => println!("Result: {:?}", token),
            Err(error) => {
                eprintln!("Error:  {:?}", error);
                exit(1);
            }
        }
    }
}
