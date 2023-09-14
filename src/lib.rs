pub mod models;
pub mod parser;
pub mod tokenizer;

use crate::parser::Parser;
use crate::tokenizer::Tokenizer;

pub fn run(code: &str) {
    let tokenizer = Tokenizer::new(code.chars());
    let mut parser = Parser::new(tokenizer);
    parser.parse();
}
