use crate::Tokenizer;
use core::str::Chars;
use std::iter::Peekable;

use std::process::exit;

pub struct Parser<'a> {
    tokenizer: Peekable<Tokenizer<Peekable<Chars<'a>>>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: Tokenizer<Peekable<Chars<'a>>>) -> Self {
        return Self {
            tokenizer: tokenizer.peekable(),
        };
    }

    pub fn parse(&mut self) {
        while let Some(result) = self.tokenizer.next() {
            match result {
                Ok(token) => println!("{:?}", token),
                Err(error) => {
                    println!("{:?}", error);
                    exit(1);
                }
            }
        }
    }
}
