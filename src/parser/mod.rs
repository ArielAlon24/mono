pub mod node;

use crate::models::error::{MonoError, Syntax};
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
        Err(Box::new(Syntax::UnexpectedToken {
            token: $token,
            expected: $expected,
        }))
    };
}

macro_rules! unclosed_token {
    ($start:expr, $end:expr, $delimeter:expr) => {
        Err(Box::new(Syntax::UnclosedTokenDelimeter {
            start: $start,
            found: $end,
            delimiter: $delimeter,
        }))
    };
}

type ParserItem = Result<Box<Node>, Box<dyn MonoError>>;

pub struct Parser<'a> {
    tokenizer: Tokenizer<Peekable<Chars<'a>>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: Tokenizer<Peekable<Chars<'a>>>) -> Self {
        Self { tokenizer }
    }

    pub fn parse(&mut self) -> ParserItem {
        self.parse_program()
    }

    fn expect_token(&mut self, expected: TokenKind) -> Result<Token, Box<dyn MonoError>> {
        match self.tokenizer.next() {
            Some(Ok(token)) if token.kind == expected => Ok(token),
            Some(Ok(token)) => unexpected_token!(token, vec![expected]),
            Some(Err(error)) => Err(error.into()),
            None => Err(Box::new(Syntax::UnexpectedEOF)),
        }
    }

    fn consume(&mut self, kind: TokenKind) {
        while let Some(Ok(token)) = self.tokenizer.peek() {
            if token.kind == kind {
                self.tokenizer.next();
            } else {
                break;
            }
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

    fn parse_parameters(
        &mut self,
        delimiter: TokenKind,
    ) -> Result<Vec<Box<Node>>, Box<dyn MonoError>> {
        let mut parameters = Vec::new();
        if matches!(self.tokenizer.peek(), Some(Ok(token)) if token.kind == delimiter) {
            return Ok(parameters);
        }

        loop {
            parameters.push(self.parse_bool_expr()?);
            match self.tokenizer.peek() {
                Some(Ok(token)) => match &token.kind {
                    k if k == &delimiter => break,
                    TokenKind::Comma => {
                        self.tokenizer.next();
                    }
                    _ => {
                        return unexpected_token!(
                            self.tokenizer.next().unwrap()?,
                            vec![delimiter, TokenKind::Comma]
                        )
                    }
                },
                Some(Err(_)) => {
                    return Err(self.tokenizer.next().expect("unreachable").unwrap_err())
                }
                None => break,
            }
        }

        Ok(parameters)
    }

    fn parse_arguments(&mut self) -> Result<Vec<Token>, Box<dyn MonoError>> {
        let mut arguments = Vec::new();
        let mut expect_argument = true;

        while let Some(Ok(token)) = self.tokenizer.peek() {
            match token.kind {
                TokenKind::Identifier(_) if expect_argument => {
                    arguments.push(self.tokenizer.next().unwrap()?);
                    expect_argument = false;
                }
                TokenKind::Identifier(_) if !expect_argument => {
                    return unexpected_token!(
                        self.tokenizer.next().unwrap()?,
                        vec![TokenKind::RightParen, TokenKind::Comma]
                    );
                }
                TokenKind::Comma if !expect_argument => {
                    self.tokenizer.next();
                    expect_argument = true;
                }
                TokenKind::RightParen if !expect_argument || arguments.is_empty() => {
                    break;
                }
                _ if expect_argument => {
                    return unexpected_token!(
                        self.tokenizer.next().unwrap()?,
                        vec![TokenKind::Identifier(String::new())]
                    )
                }
                _ => {
                    return unexpected_token!(
                        self.tokenizer.next().unwrap()?,
                        vec![TokenKind::Comma, TokenKind::RightParen]
                    )
                }
            }
        }

        Ok(arguments)
    }

    fn close_delimiter(
        &mut self,
        start: Token,
        delimiter: TokenKind,
    ) -> Result<(), Box<dyn MonoError>> {
        match self.tokenizer.next() {
            None => unclosed_token!(start, None, delimiter),
            Some(Err(e)) => Err(e),
            Some(Ok(t)) if t.kind == delimiter => Ok(()),
            Some(Ok(end)) => unclosed_token!(start, Some(end), delimiter),
        }
    }

    fn parse_atom(&mut self) -> ParserItem {
        if let None = self.tokenizer.peek() {
            return Syntax::UnexpectedEOF.into();
        }

        let token = self.tokenizer.next().unwrap()?;
        match token.kind {
            TokenKind::LeftParen => {
                let bool_expr = self.parse_bool_expr()?;
                self.close_delimiter(token, TokenKind::RightParen)?;
                Ok(bool_expr)
            }
            TokenKind::LeftBracket => {
                let values = self.parse_parameters(TokenKind::RightBracket)?;
                self.close_delimiter(token, TokenKind::RightBracket)?;
                Node::List { values }.into()
            }
            TokenKind::Integer(_)
            | TokenKind::Float(_)
            | TokenKind::Boolean(_)
            | TokenKind::Character(_)
            | TokenKind::String(_)
            | TokenKind::None => atom!(token),
            TokenKind::Identifier(_) => match self.tokenizer.peek() {
                Some(Ok(paren)) if paren.kind == TokenKind::LeftParen => {
                    self.parse_func_call(token)
                }
                Some(Ok(bracket)) if bracket.kind == TokenKind::LeftBracket => {
                    self.parse_index(token)
                }
                _ => Node::Access { identifier: token }.into(),
            },
            _ => {
                unexpected_token!(
                    token,
                    vec![
                        TokenKind::LeftParen,
                        TokenKind::Integer(0),
                        TokenKind::Float(0.0),
                        TokenKind::Identifier("".to_string()),
                        TokenKind::String("".to_string()),
                        TokenKind::Character(' '),
                        TokenKind::Add,
                        TokenKind::Sub,
                    ]
                )
            }
        }
    }

    fn parse_index(&mut self, identifier: Token) -> ParserItem {
        let start = self.expect_token(TokenKind::LeftBracket)?;
        let index = self.parse_expr()?;
        self.close_delimiter(start, TokenKind::RightBracket)?;
        Node::Index { identifier, index }.into()
    }

    fn parse_func_call(&mut self, identifier: Token) -> ParserItem {
        let start = self.expect_token(TokenKind::LeftParen)?;
        let parameters = self.parse_parameters(TokenKind::RightParen)?;
        self.close_delimiter(start, TokenKind::RightParen)?;
        Node::FuncCall {
            identifier,
            parameters,
        }
        .into()
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
        let start = self.expect_token(TokenKind::LeftCurly)?;
        let program = self.parse_program()?;
        self.close_delimiter(start, TokenKind::RightCurly)?;
        Ok(program)
    }

    fn parse_assignment(&mut self, identifier: Token, is_declaration: bool) -> ParserItem {
        match self.tokenizer.next() {
            None => Err(Box::new(Syntax::UnexpectedEOF)),
            Some(Err(error)) => Err(error.into()),
            Some(Ok(token)) if token.kind == TokenKind::LeftParen => {
                let arguments = self.parse_arguments()?;
                self.close_delimiter(token, TokenKind::RightParen)?;
                self.expect_token(TokenKind::DoubleArrow)?;
                let body = self.parse_block()?;
                Node::FuncDeclearion {
                    identifier,
                    arguments,
                    body,
                }
                .into()
            }
            Some(Ok(token)) if token.kind == TokenKind::Assignment => Node::Assignment {
                identifier,
                value: self.parse_bool_expr()?,
                is_declaration,
            }
            .into(),
            Some(Ok(token)) => {
                unexpected_token!(token, vec![TokenKind::LeftParen, TokenKind::Assignment])
            }
        }
    }

    fn parse_if(&mut self) -> ParserItem {
        self.tokenizer.next(); // Going over the 'If' token

        let condition = self.parse_bool_expr()?;
        let block = self.parse_block()?;

        self.consume(TokenKind::NewLine);

        match self.tokenizer.peek() {
            Some(Ok(token)) if token.kind == TokenKind::Else => (),
            _ => {
                return Node::If {
                    condition,
                    block,
                    else_block: None,
                }
                .into();
            }
        }

        self.tokenizer.next(); // Going over the 'Else' token

        self.consume(TokenKind::NewLine);

        let else_block = if matches!(self.tokenizer.peek(), Some(Ok(token)) if token.kind == TokenKind::If)
        {
            Some(self.parse_if()?)
        } else {
            Some(self.parse_block()?)
        };

        Node::If {
            condition,
            block,
            else_block,
        }
        .into()
    }

    fn parse_while(&mut self) -> ParserItem {
        self.tokenizer.next(); // Going over the 'While' token.
        Node::While {
            condition: self.parse_bool_expr()?,
            block: self.parse_block()?,
        }
        .into()
    }

    fn parse_return(&mut self) -> ParserItem {
        self.tokenizer.next(); // Going over the 'Return' token.
        let value = self.parse_bool_expr()?;
        Node::Return { value }.into()
    }

    fn parse_statement(&mut self) -> ParserItem {
        match self.tokenizer.peek() {
            None => Syntax::UnexpectedEOF.into(),
            Some(Err(_)) => Err(self.tokenizer.next().expect("unreachable").unwrap_err()),
            Some(Ok(token)) => match token.kind {
                TokenKind::Let => {
                    self.tokenizer.next();
                    let identifier = self.expect_token(TokenKind::Identifier(String::new()))?;
                    self.parse_assignment(identifier, true)
                }
                TokenKind::If => self.parse_if(),
                TokenKind::While => self.parse_while(),
                TokenKind::Identifier(_) => self.parse_identifier_statement(),
                TokenKind::Return => self.parse_return(),
                _ => unexpected_token!(
                    self.tokenizer.next().unwrap()?,
                    vec![
                        TokenKind::Let,
                        TokenKind::If,
                        TokenKind::While,
                        TokenKind::Identifier(String::new()),
                    ]
                ),
            },
        }
    }

    fn parse_identifier_statement(&mut self) -> ParserItem {
        let identifier = self.expect_token(TokenKind::Identifier(String::new()))?;
        match self.tokenizer.peek() {
            Some(Ok(token)) if token.kind == TokenKind::Assignment => {
                self.parse_assignment(identifier, false)
            }
            Some(Ok(token)) if token.kind == TokenKind::LeftParen => {
                self.parse_func_call(identifier)
            }
            Some(Ok(token)) if token.kind == TokenKind::LeftBracket => {
                let start = self.expect_token(TokenKind::LeftBracket)?;
                let index = self.parse_expr()?;
                self.close_delimiter(start, TokenKind::RightBracket)?;
                self.expect_token(TokenKind::Assignment)?;
                let value = self.parse_bool_expr()?;
                Node::ListAssignment {
                    identifier,
                    index,
                    value,
                }
                .into()
            }
            Some(Ok(_)) => unexpected_token!(
                self.tokenizer.next().unwrap()?,
                vec![TokenKind::Assignment, TokenKind::LeftParen]
            ),
            Some(Err(_)) => Err(self.tokenizer.next().expect("unreachable").unwrap_err()),
            None => Syntax::UnexpectedEOF.into(),
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

        Node::Program { statements }.into()
    }
}
