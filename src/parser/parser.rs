use crate::models::error::Error;
use crate::models::error::InvalidSyntax;
use crate::parser::node::Node;
use crate::tokenizer::token::TokenKind;

use crate::Tokenizer;
use core::str::Chars;
use std::iter::Peekable;

macro_rules! atom {
    ($token:expr) => {
        Ok(Box::new(Node::Atom($token)))
    };
}

macro_rules! binary_op {
    ($left:expr, $operator:expr, $right:expr) => {
        Box::new(Node::BinaryOp($left, $operator, $right))
    };
}

macro_rules! unary_op {
    ($operator:expr, $token:expr) => {
        Box::new(Node::UnaryOp($operator, $token))
    };
}

macro_rules! expected_error {
    ($expected:expr, $found:expr) => {
        Err(Error::invalid_syntax(InvalidSyntax::InvalidToken {
            expected: $expected,
            found: $found,
        }))
    };
}

macro_rules! unclosed_error {
    ($start:expr, $end:expr, $delimeter:expr) => {
        Err(Error::invalid_syntax(
            InvalidSyntax::UnclosedTokenDelimeter($start, $end, $delimeter),
        ))
    };
}

type ParserItem = Result<Box<Node>, Error>;

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
        self.parse_expr()
    }

    fn parse_atom(&mut self) -> ParserItem {
        if let None = self.tokenizer.peek() {
            return expected_error!(
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
            TokenKind::LeftParen => {
                let expr = self.parse_expr()?;
                match self.tokenizer.next() {
                    Some(Ok(t)) if t.kind == TokenKind::RightParen => Ok(expr),
                    Some(Ok(end)) => unclosed_error!(token, Some(end), TokenKind::RightParen),
                    _ => unclosed_error!(token, None, TokenKind::RightParen),
                }
            }
            TokenKind::Integer(_) | TokenKind::Float(_) => atom!(token),
            _ => expected_error!(
                vec![
                    TokenKind::LeftParen,
                    TokenKind::Integer(0),
                    TokenKind::Float(0.0)
                ],
                Some(token)
            ),
        }
    }

    fn parse_binary_op(
        &mut self,
        operators: &[TokenKind],
        left: fn(&mut Self) -> ParserItem,
        right: fn(&mut Self) -> ParserItem,
    ) -> ParserItem {
        let mut root = left(self)?;
        while let Some(Ok(token)) = self.tokenizer.peek() {
            if !operators.contains(&token.kind) {
                break;
            }
            root = binary_op!(root, self.tokenizer.next().unwrap()?, right(self)?);
        }
        Ok(root)
    }

    fn parse_power(&mut self) -> ParserItem {
        self.parse_binary_op(&[TokenKind::Pow], Self::parse_atom, Self::parse_factor)
    }

    fn parse_factor(&mut self) -> ParserItem {
        match self.tokenizer.peek() {
            Some(Ok(token)) if token.kind == TokenKind::Add || token.kind == TokenKind::Sub => Ok(
                unary_op!(self.tokenizer.next().unwrap()?, self.parse_factor()?),
            ),
            _ => self.parse_power(),
        }
    }

    fn parse_term(&mut self) -> ParserItem {
        self.parse_binary_op(
            &[TokenKind::Mul, TokenKind::Div, TokenKind::Mod],
            Self::parse_factor,
            Self::parse_factor,
        )
    }

    fn parse_expr(&mut self) -> ParserItem {
        self.parse_binary_op(
            &[TokenKind::Add, TokenKind::Sub],
            Self::parse_term,
            Self::parse_term,
        )
    }
}
