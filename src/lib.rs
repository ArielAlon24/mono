pub mod evaluator;
pub mod models;
pub mod parser;
pub mod tokenizer;

use crate::evaluator::evaluator::Evaluator;
use crate::parser::parser::Parser;
use crate::tokenizer::tokenizer::Tokenizer;
use colored::*;

macro_rules! ereport {
    ($color:ident, $header:expr, $error:expr) => {
        eprintln!(
            "{}\n{}{} {}\n",
            $header.$color().bold(),
            ($error.to_kind()).$color().underline(),
            ":".red(),
            ($error.to_message()).$color()
        )
    };
}

macro_rules! report {
    ($color:ident, $header:expr, $object:expr) => {
        println!("{}\n{}\n", $header.$color().bold(), ($object).$color())
    };
}

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
        Err(error) => ereport!(red, "Error", error),
    }
}

pub fn parser(code: &str) {
    let tokenizer = Tokenizer::new(code.chars());
    let mut parser = Parser::new(tokenizer);
    match parser.parse() {
        Ok(tree) => report!(purple, "Ok", tree.to_string()),
        Err(error) => ereport!(red, "Error", error),
    }
}

pub fn evaluator(code: &str) {
    let tokenizer = Tokenizer::new(code.chars());
    let mut parser = Parser::new(tokenizer);
    match parser.parse() {
        Err(error) => ereport!(red, "Parsing Error", error),
        Ok(ast) => match Evaluator::evaluate(ast) {
            Err(error) => ereport!(red, "Evaluator Error", error),
            Ok(value) => report!(green, "Ok", format!("{:?}", value)),
        },
    }
}
