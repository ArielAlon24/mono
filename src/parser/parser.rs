use crate::models::error::Error;
use crate::models::error::Syntax;
use crate::parser::node::Node;
use crate::tokenizer::token::{Token, TokenKind};
use crate::Tokenizer;
use core::str::Chars;
use std::iter::Peekable;

macro_rules! atom {
    ($token:expr) => {
        Ok(Box::new(Node::Atom { value: $token }))
    };
}

macro_rules! unexpected_token {
    ($token:expr, $expected:expr) => {
        Err(Error::syntax(Syntax::UnexpectedToken {
            token: $token,
            expected: $expected,
        }))
    };
}

macro_rules! unclosed_token {
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
            root = Box::new(Node::BinaryOp {
                left: root,
                operator: self.tokenizer.next().unwrap()?,
                right: right(self)?,
            });
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
            Some(Ok(token)) if operators.contains(&token.kind) => Ok(Box::new(Node::UnaryOp {
                operator: self.tokenizer.next().unwrap()?,
                value: operand(self)?,
            })),
            _ => defualt(self),
        }
    }

    fn parse_atom(&mut self) -> ParserItem {
        if let None = self.tokenizer.peek() {
            return Err(Error::syntax(Syntax::UnexpectedEOF));
        }

        let token = self.tokenizer.next().unwrap()?;
        match token.kind {
            TokenKind::LeftParen => {
                let bool_expr = self.parse_bool_expr()?;
                match self.tokenizer.next() {
                    Some(Ok(t)) if t.kind == TokenKind::RightParen => Ok(bool_expr),
                    Some(Ok(end)) => unclosed_token!(token, Some(end), TokenKind::RightParen),
                    _ => unclosed_token!(token, None, TokenKind::RightParen),
                }
            }
            TokenKind::Integer(_) | TokenKind::Float(_) => atom!(token),
            TokenKind::Boolean(_) => atom!(token),
            TokenKind::Identifier(_) => Ok(Box::new(Node::Access { identifier: token })),
            _ => {
                unexpected_token!(
                    token,
                    vec![
                        TokenKind::LeftParen,
                        TokenKind::Integer(0),
                        TokenKind::Float(0.0),
                        TokenKind::Identifier("".to_string()),
                        TokenKind::Add,
                        TokenKind::Sub,
                    ]
                )
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
            return Err(Error::syntax(Syntax::UnexpectedEOF));
        }
        let start = self.tokenizer.next().unwrap()?;
        if start.kind != TokenKind::LeftCurly {
            return unexpected_token!(start, vec![TokenKind::LeftCurly]);
        }

        let program = self.parse_program()?;

        if let None = self.tokenizer.peek() {
            return unclosed_token!(start, None, TokenKind::RightCurly);
        }
        let end = self.tokenizer.next().unwrap()?;
        match end.kind {
            TokenKind::RightCurly => Ok(program),
            _ => unclosed_token!(start, Some(end), TokenKind::RightCurly),
        }
    }

    fn parse_assignment(&mut self) -> ParserItem {
        let identifier = self.tokenizer.next().unwrap()?;
        if let TokenKind::Identifier(_) = identifier.kind {
            if let None = self.tokenizer.peek() {
                return Err(Error::syntax(Syntax::UnexpectedEOF));
            }
            let equals = self.tokenizer.next().unwrap()?;
            if equals.kind != TokenKind::Assignment {
                return unexpected_token!(equals, vec![TokenKind::Assignment]);
            }
            let value = self.parse_bool_expr()?;
            return Ok(Box::new(Node::Assignment { identifier, value }));
        }
        unexpected_token!(identifier, vec![TokenKind::Identifier("".to_string())])
    }

    fn parse_if(&mut self) -> ParserItem {
        self.tokenizer.next(); // Going over the 'If' token.
        let condition = self.parse_bool_expr()?;
        let block = self.parse_block()?;
        let mut else_block = None;
        if let Some(Ok(token)) = self.tokenizer.peek() {
            if token.kind == TokenKind::Else {
                self.tokenizer.next(); // Going over the 'Else' token.
                else_block = Some(self.parse_block()?);
            }
        }
        Ok(Box::new(Node::If {
            condition: condition,
            block: block,
            else_block: else_block,
        }))
    }

    fn parse_while(&mut self) -> ParserItem {
        self.tokenizer.next(); // Going over the 'While' token.
        Ok(Box::new(Node::While {
            condition: self.parse_bool_expr()?,
            block: self.parse_block()?,
        }))
    }

    fn parse_statement(&mut self) -> ParserItem {
        match self.tokenizer.peek() {
            None => Err(Error::syntax(Syntax::UnexpectedEOF)),
            Some(Ok(token)) => match token.kind {
                TokenKind::Let => {
                    self.tokenizer.next();
                    self.parse_assignment()
                }
                TokenKind::If => self.parse_if(),
                TokenKind::While => self.parse_while(),
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
