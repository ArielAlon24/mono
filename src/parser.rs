use crate::Tokenizer;
use core::str::Chars;
use std::iter::Peekable;

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
        loop {
            match self.tokenizer.next() {
                Some(Ok(token)) => println!("{:?}", token),
                Some(Err(error)) => {
                    println!("{:?}", error);
                    return;
                }
                None => return,
            }
        }
    }
}
