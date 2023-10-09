use crate::models::error::Error;
use crate::models::error::InvalidSyntax;
use crate::parser::node::Node;
use crate::tokenizer::token::{Token, TokenKind};
use crate::Tokenizer;
use core::str::Chars;
use std::iter::Peekable;

macro_rules! atom {
    ($token:expr) => {
        Ok(Box::new(Node::Atom($token)))
    };
}

macro_rules! unexpected_error {
    ($token:expr) => {
        Err(Error::invalid_syntax(InvalidSyntax::UnexpectedToken {
            token: $token,
        }))
    };
}

macro_rules! unclosed_error {
    ($start:expr, $end:expr, $delimeter:expr) => {
        Err(Error::invalid_syntax(
            InvalidSyntax::UnclosedTokenDelimeter {
                start: $start,
                found: $end,
                delimiter: $delimeter,
            },
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
        let expr = self.parse_bool_expr()?;
        match self.tokenizer.peek() {
            None => Ok(expr),
            _ => Err(Error::invalid_syntax(InvalidSyntax::MultipleExpressions {
                position: self.tokenizer.get_position(),
            })),
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
            root = Box::new(Node::BinaryOp(
                root,
                self.tokenizer.next().unwrap()?,
                right(self)?,
            ));
        }
        Ok(root)
    }

    fn parse_unary_op(
        &mut self,
        operators: &[TokenKind],
        operand: fn(&mut Self) -> ParserItem,
        defualt: fn(&mut Self) -> ParserItem,
    ) -> ParserItem {
        match self.tokenizer.peek() {
            Some(Ok(token)) if operators.contains(&token.kind) => Ok(Box::new(Node::UnaryOp(
                self.tokenizer.next().unwrap()?,
                operand(self)?,
            ))),
            _ => defualt(self),
        }
    }

    fn parse_atom(&mut self) -> ParserItem {
        if let None = self.tokenizer.peek() {
            return unexpected_error!(None);
        }

        let token = self.tokenizer.next().unwrap()?;
        match token.kind {
            TokenKind::LeftParen => {
                let bool_expr = self.parse_bool_expr()?;
                match self.tokenizer.next() {
                    Some(Ok(t)) if t.kind == TokenKind::RightParen => Ok(bool_expr),
                    Some(Ok(end)) => unclosed_error!(token, Some(end), TokenKind::RightParen),
                    _ => unclosed_error!(token, None, TokenKind::RightParen),
                }
            }
            TokenKind::Integer(_) | TokenKind::Float(_) => atom!(token),
            TokenKind::Boolean(_) => atom!(token),
            _ => unexpected_error!(Some(token)),
        }
    }

    fn parse_power(&mut self) -> ParserItem {
        self.parse_binary_op(&[TokenKind::Pow], Self::parse_atom, Self::parse_factor)
    }

    fn parse_factor(&mut self) -> ParserItem {
        self.parse_unary_op(
            &[TokenKind::Sub, TokenKind::Add],
            Self::parse_factor,
            Self::parse_power,
        )
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

    fn parse_comparison(&mut self) -> ParserItem {
        self.parse_binary_op(
            &Token::COMPERATORS.to_vec(),
            Self::parse_expr,
            Self::parse_expr,
        )
    }

    fn parse_bool_factor(&mut self) -> ParserItem {
        self.parse_unary_op(
            &[TokenKind::Not],
            Self::parse_bool_factor,
            Self::parse_comparison,
        )
    }

    fn parse_bool_term(&mut self) -> ParserItem {
        self.parse_binary_op(
            &[TokenKind::And],
            Self::parse_bool_factor,
            Self::parse_bool_factor,
        )
    }

    fn parse_bool_expr(&mut self) -> ParserItem {
        self.parse_binary_op(
            &[TokenKind::Or],
            Self::parse_bool_term,
            Self::parse_bool_term,
        )
    }
}
