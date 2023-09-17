use crate::models::error::Error;

use crate::models::error::InvalidSyntax;
use crate::models::node::Expression;
use crate::models::token::TokenKind;
use crate::Tokenizer;
use core::str::Chars;
use std::iter::Peekable;

macro_rules! atom {
    ($token:expr) => {
        Ok(Expression::Atom($token))
    };
}

macro_rules! expected_error {
    ($self:ident, $expected:expr, $found:expr) => {
        Err(Error::invalid_syntax(InvalidSyntax::InvalidToken {
            expected: $expected,
            found: $found,
        }))
    };
}

type ParserItem = Result<Expression, Error>;

pub struct Parser<'a> {
    tokenizer: Tokenizer<Peekable<Chars<'a>>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: Tokenizer<Peekable<Chars<'a>>>) -> Self {
        return Self {
            tokenizer: tokenizer,
        };
    }

    pub fn parse(&mut self) -> ParserItem {
        self.parse_atom()
    }

    fn parse_atom(&mut self) -> ParserItem {
        if let None = self.tokenizer.peek() {
            return expected_error!(
                self,
                vec![
                    TokenKind::LeftParen,
                    TokenKind::Integer(0),
                    TokenKind::Float(0.0)
                ],
                None
            );
        }

        let token = self.tokenizer.next().unwrap()?;
        match token.kind {
            TokenKind::LeftParen => todo!(),
            TokenKind::Integer(_) | TokenKind::Float(_) => atom!(token),
            _ => expected_error!(
                self,
                vec![
                    TokenKind::LeftParen,
                    TokenKind::Integer(0),
                    TokenKind::Float(0.0)
                ],
                Some(token)
            ),
        }
    }
}
