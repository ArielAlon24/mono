use crate::models::error::Error;
use crate::models::error::Syntax;
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
        Err(Error::syntax(Syntax::UnexpectedToken { token: $token }))
    };
}

macro_rules! unclosed_error {
    ($start:expr, $end:expr, $delimeter:expr) => {
        Err(Error::syntax(Syntax::UnclosedTokenDelimeter {
            start: $start,
            found: $end,
            delimiter: $delimeter,
        }))
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
        self.parse_program()
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
            TokenKind::Identifier(_) => Ok(Box::new(Node::Access(token))),
            _ => {
                unexpected_error!(Some(token))
            }
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

    fn parse_block(&mut self) -> ParserItem {
        if let None = self.tokenizer.peek() {
            return unexpected_error!(None);
        }
        let token = self.tokenizer.next().unwrap()?;
        if token.kind != TokenKind::LeftCurly {
            return unexpected_error!(Some(token));
        }

        let program = self.parse_program()?;

        if let Some(Ok(_)) = self.tokenizer.peek() {
            let token = self.tokenizer.next().unwrap()?;
            if token.kind != TokenKind::RightCurly {
                return unexpected_error!(Some(token));
            }
        }

        Ok(program)
    }

    fn parse_assignment(&mut self) -> ParserItem {
        let token = self.tokenizer.next().unwrap()?;
        if let TokenKind::Identifier(_) = token.kind {
            if let None = self.tokenizer.peek() {
                return unexpected_error!(None);
            }
            let equals = self.tokenizer.next().unwrap()?;
            if equals.kind != TokenKind::Assignment {
                return unexpected_error!(Some(equals));
            }
            let expr = self.parse_bool_expr()?;
            return Ok(Box::new(Node::Assignment(token, expr)));
        }
        unexpected_error!(Some(token))
    }

    fn parse_if(&mut self) -> ParserItem {
        self.tokenizer.next(); // Going over the If token.
        Ok(Box::new(Node::If {
            condition: self.parse_bool_expr()?,
            block: self.parse_block()?,
        }))
    }

    fn parse_statement(&mut self) -> ParserItem {
        match self.tokenizer.peek() {
            None => unexpected_error!(None),
            Some(Ok(token)) => match token.kind {
                TokenKind::Let => {
                    self.tokenizer.next();
                    self.parse_assignment()
                }
                TokenKind::If => self.parse_if(),
                _ => self.parse_bool_expr(),
            },
            Some(Err(_)) => {
                let error = self.tokenizer.next().expect("unreachable!").unwrap_err();
                return Err(error);
            }
        }
    }

    fn parse_program(&mut self) -> ParserItem {
        let mut statements: Vec<Box<Node>> = Vec::new();

        while let Some(result) = self.tokenizer.peek() {
            if let Ok(token) = result {
                if token.kind == TokenKind::RightCurly {
                    break;
                }
                if token.kind == TokenKind::NewLine {
                    self.tokenizer.next();
                    continue;
                }
            }
            statements.push(self.parse_statement()?);
        }
        Ok(Box::new(Node::Program { statements }))
    }
}
