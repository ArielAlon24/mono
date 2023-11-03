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
        Err(Syntax::UnexpectedToken {
            token: $token,
            expected: $expected,
        }
        .into())
    };
}

macro_rules! unclosed_token {
    ($start:expr, $end:expr, $delimeter:expr) => {
        Err(Syntax::UnclosedTokenDelimeter {
            start: $start,
            found: $end,
            delimiter: $delimeter,
        }
        .into())
    };
}

type ParserItem = Result<Box<Node>, Error>;

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

    fn expect_token(&mut self, expected: TokenKind) -> Result<Token, Error> {
        match self.tokenizer.next() {
            Some(Ok(token)) if token.kind == expected => Ok(token),
            Some(Ok(token)) => unexpected_token!(token, vec![expected]),
            _ => Err(Syntax::UnexpectedEOF.into()),
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

    fn parse_parameters(&mut self) -> Result<Vec<Box<Node>>, Error> {
        if let Some(Ok(token)) = self.tokenizer.peek() {
            if token.kind == TokenKind::RightParen {
                return Ok(Vec::new());
            }
        }
        let mut parameters = vec![self.parse_bool_expr()?];

        while let Some(Ok(token)) = self.tokenizer.peek() {
            match token.kind {
                TokenKind::RightParen => break,
                TokenKind::Comma => {
                    self.tokenizer.next(); // going over Comma
                    let bool_expr = self.parse_bool_expr()?;
                    parameters.push(bool_expr);
                }
                _ => {
                    return unexpected_token!(
                        self.tokenizer.next().unwrap()?,
                        vec![TokenKind::RightParen, TokenKind::Comma]
                    )
                }
            }
        }

        Ok(parameters)
    }

    fn parse_list_until_delimiter(
        &mut self,
        item: TokenKind,
        delimiter: TokenKind,
    ) -> Result<Vec<Token>, Error> {
        let mut items = Vec::new();
        let mut expect_item = true;

        while let Some(Ok(token)) = self.tokenizer.peek() {
            match token.kind {
                ref kind if *kind == item && expect_item => {
                    items.push(self.tokenizer.next().unwrap()?);
                    expect_item = false;
                }
                ref kind if *kind == item && !expect_item => {
                    return unexpected_token!(
                        self.tokenizer.next().unwrap()?,
                        vec![delimiter, TokenKind::Comma]
                    );
                }
                TokenKind::Comma if !expect_item => {
                    self.tokenizer.next();
                    expect_item = true;
                }
                TokenKind::Comma if expect_item => {
                    return unexpected_token!(self.tokenizer.next().unwrap()?, vec![item]);
                }
                ref kind if *kind == delimiter && (!expect_item || items.len() == 0) => {
                    break;
                }
                _ if expect_item => {
                    return unexpected_token!(self.tokenizer.next().unwrap()?, vec![item])
                }
                _ => {
                    return unexpected_token!(
                        self.tokenizer.next().unwrap()?,
                        vec![TokenKind::Comma, delimiter]
                    )
                }
            }
        }

        Ok(items)
    }

    fn parse_atom(&mut self) -> ParserItem {
        if let None = self.tokenizer.peek() {
            return Err(Syntax::UnexpectedEOF.into());
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
            TokenKind::Integer(_) | TokenKind::Float(_) | TokenKind::Boolean(_) => atom!(token),
            TokenKind::Identifier(_) => match self.tokenizer.peek() {
                Some(Ok(paren)) if paren.kind == TokenKind::LeftParen => {
                    self.parse_func_call(token)
                }
                _ => Ok(Box::new(Node::Access { identifier: token })),
            },
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

    fn parse_func_call(&mut self, identifier: Token) -> ParserItem {
        let start = self.expect_token(TokenKind::LeftParen)?;
        let parameters = self.parse_parameters()?;
        if let None = self.tokenizer.peek() {
            return unclosed_token!(start, None, TokenKind::RightParen);
        }
        let end = self.tokenizer.next().unwrap()?;
        match end.kind {
            TokenKind::RightParen => Ok(Box::new(Node::FuncCall {
                identifier,
                parameters,
            })),
            _ => unclosed_token!(start, Some(end), TokenKind::RightParen),
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
        self.expect_token(TokenKind::LeftCurly)?;
        let program = self.parse_program()?;
        self.expect_token(TokenKind::RightCurly)?;
        Ok(program)
    }

    fn parse_assignment(&mut self, identifier: Token, is_declaration: bool) -> ParserItem {
        if let None = self.tokenizer.peek() {
            return Err(Syntax::UnexpectedEOF.into());
        }
        let token = self.tokenizer.next().unwrap()?;
        if token.kind == TokenKind::LeftParen {
            let arguments = self.parse_list_until_delimiter(
                TokenKind::Identifier(String::new()),
                TokenKind::RightParen,
            )?;
            self.expect_token(TokenKind::RightParen)?;
            self.expect_token(TokenKind::DoubleArrow)?;
            let body = self.parse_block()?;
            return Ok(Box::new(Node::FuncDeclearion {
                identifier,
                arguments,
                body,
            }));
        } else if token.kind == TokenKind::Assignment {
            let value = self.parse_bool_expr()?;
            return Ok(Box::new(Node::Assignment {
                identifier,
                value,
                is_declaration,
            }));
        }
        unexpected_token!(token, vec![TokenKind::LeftParen, TokenKind::Assignment])
    }

    fn parse_if(&mut self) -> ParserItem {
        self.tokenizer.next(); // Going over the 'If' token.
        let condition = self.parse_bool_expr()?;
        let block = self.parse_block()?;

        if let Some(Ok(token)) = self.tokenizer.peek() {
            if token.kind != TokenKind::Else {
                return Ok(Box::new(Node::If {
                    condition,
                    block,
                    else_block: None,
                }));
            }

            self.tokenizer.next(); // Going over the 'Else' token.
            if let Some(Ok(token)) = self.tokenizer.peek() {
                if token.kind == TokenKind::If {
                    return Ok(Box::new(Node::If {
                        condition,
                        block,
                        else_block: Some(self.parse_if()?),
                    }));
                }
            }
        }
        Ok(Box::new(Node::If {
            condition: condition,
            block: block,
            else_block: Some(self.parse_block()?),
        }))
    }

    fn parse_while(&mut self) -> ParserItem {
        self.tokenizer.next(); // Going over the 'While' token.
        Ok(Box::new(Node::While {
            condition: self.parse_bool_expr()?,
            block: self.parse_block()?,
        }))
    }

    fn parse_return(&mut self) -> ParserItem {
        self.tokenizer.next(); // Going over the 'Return' token.
        Ok(Box::new(Node::Return {
            value: self.parse_bool_expr()?,
        }))
    }

    fn parse_statement(&mut self) -> ParserItem {
        match self.tokenizer.peek() {
            None => Err(Syntax::UnexpectedEOF.into()),
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
                // _ => self.parse_bool_expr(),
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
            // Some(Ok(_)) => self.parse_bool_expr(),
            Some(Ok(_)) => unexpected_token!(
                self.tokenizer.next().unwrap()?,
                vec![TokenKind::Assignment, TokenKind::LeftParen]
            ),
            Some(Err(_)) => Err(self.tokenizer.next().expect("unreachable").unwrap_err()),
            None => Err(Syntax::UnexpectedEOF.into()),
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
