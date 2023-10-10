pub mod evaluator;
pub mod models;
pub mod parser;
pub mod tokenizer;

use crate::evaluator::evaluator::Evaluator;
use crate::parser::parser::Parser;
use crate::tokenizer::tokenizer::Tokenizer;
use colored::*;

/*
The ereport macro get a color header and error attributes
and prints to the stderr the header and the error in the
specified color.
*/
macro_rules! ereport {
    ($color:ident, $header:expr, $error:expr) => {
        eprintln!("{}\n{}\n", $header.$color().bold(), ($error).$color())
    };
}

/*
The report macro get a color header and object attributes
and prints to the stdout the header and the object with the
specified color.
*/
macro_rules! report {
    ($color:ident, $header:expr, $object:expr) => {
        println!("{}\n{}\n", $header.$color().bold(), ($object).$color())
    };
}

/*
The tokenizer function runs the Tokenizer on the given code
string, and prints the output of it in the stdout or stderr
depending on the outcome.
*/
pub fn tokenizer(code: &str) {
    let tok = Tokenizer::new(code.chars());
    let results: Result<Vec<_>, _> = tok.collect();

    match results {
        Ok(tokens) => {
            let tokens_string = tokens
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n");
            report!(blue, "Ok", tokens_string);
        }
        Err(error) => ereport!(red, "Error", error.to_string()),
    }
}

/*
The parser function runs the Parser on the given code
string, and prints the output of it in the stdout or stderr
depending on the outcome.
*/
pub fn parser(code: &str) {
    let tokenizer = Tokenizer::new(code.chars());
    let mut parser = Parser::new(tokenizer);
    match parser.parse() {
        Ok(tree) => report!(purple, "Ok", tree.to_string()),
        Err(error) => ereport!(red, "Error", error.to_string()),
    }
}

/*
The evaluator function runs the Evaluator on the given code
string, and prints the output of it in the stdout or stderr
depending on the outcome.
*/
pub fn evaluator(code: &str) {
    let tokenizer = Tokenizer::new(code.chars());
    let mut parser = Parser::new(tokenizer);
    match parser.parse() {
        Err(error) => ereport!(red, "Parsing Error", error.to_string()),
        Ok(ast) => match Evaluator::evaluate(ast) {
            Err(error) => ereport!(red, "Evaluator Error", error.to_string()),
            Ok(value) => report!(green, "Ok", format!("{:?}", value)),
        },
    }
}
