use crate::models::error::{Error, InvalidSyntax};
use crate::models::position::Position;
use crate::models::token::{Token, TokenKind};

use core::iter::Peekable;

macro_rules! single {
    ($self:ident, $TokenKind:expr) => {
        Some(Ok(Token::single($TokenKind, $self.get_position())))
    };
}

macro_rules! multi {
    ($self:ident, $TokenKind:expr, $start:expr) => {
        Some(Ok(Token::multi($TokenKind, $start, $self.get_position())))
    };
}

macro_rules! error {
    ($ErrorKind:expr) => {
        Some(Err(Error::invalid_syntax($ErrorKind)))
    };
}

type IteratorItem = Option<Result<Token, Error>>;

pub struct Tokenizer<Chars: Iterator<Item = char>> {
    chars: Chars,
    current: IteratorItem,
    position: Position,
}

impl<Chars: Iterator<Item = char>> Tokenizer<Peekable<Chars>> {
    pub fn new(chars: Chars) -> Self {
        let mut tokenizer = Self {
            chars: chars.peekable(),
            current: None,
            position: Position::new(1, 0),
        };
        tokenizer.next();
        tokenizer
    }

    pub fn peek(&mut self) -> &IteratorItem {
        &self.current
    }

    pub fn get_position(&self) -> Position {
        self.position.clone()
    }

    fn _next(&mut self) -> IteratorItem {
        self.position.next();
        return match self.chars.next() {
            Some(' ') => self._next(),
            Some('\n') => {
                let token = single!(self, TokenKind::NewLine);
                self.position.newline();
                token
            }
            Some(c) if c.is_ascii_alphabetic() || c == '_' => self.next_identifier(c),
            Some(c) if c == '"' => self.next_string(),
            Some(c) if c == '\'' => self.next_char(),
            Some(c) if c.is_numeric() => self.next_number(c),
            Some('+') => single!(self, TokenKind::Addition),
            Some('-') => match self.chars.peek() {
                Some('>') => {
                    let start = self.get_position();
                    self.position.next();
                    self.chars.next();
                    multi!(self, TokenKind::Arrow, start)
                }
                _ => single!(self, TokenKind::Subtraction),
            },
            Some('*') => single!(self, TokenKind::Multiplication),
            Some('/') => single!(self, TokenKind::Division),
            Some('%') => single!(self, TokenKind::Modulo),
            Some('^') => single!(self, TokenKind::Power),
            Some('=') => match self.chars.peek() {
                Some('>') => {
                    let start = self.get_position();
                    self.chars.next();
                    self.position.next();
                    multi!(self, TokenKind::DoubleArrow, start)
                }
                Some('=') => {
                    let start = self.get_position();
                    self.chars.next();
                    self.position.next();
                    multi!(self, TokenKind::Equals, start)
                }
                _ => single!(self, TokenKind::Assignment),
            },
            Some('!') => match self.chars.next() {
                Some('=') => {
                    let start = self.get_position();
                    self.position.next();
                    multi!(self, TokenKind::NotEquals, start)
                }
                Some(_) => {
                    self.position.next();
                    error!(InvalidSyntax::UnexpectedChar(self.get_position(), '!'))
                }
                None => error!(InvalidSyntax::UnexpectedChar(self.get_position(), '!')),
            },
            Some('>') => match self.chars.peek() {
                Some('=') => {
                    let start = self.get_position();
                    self.chars.next();
                    self.position.next();
                    multi!(self, TokenKind::GreaterEq, start)
                }
                _ => single!(self, TokenKind::Greater),
            },
            Some('<') => match self.chars.peek() {
                Some('=') => {
                    let start = self.get_position();
                    self.chars.next();
                    self.position.next();
                    multi!(self, TokenKind::LessThanEq, start)
                }
                _ => single!(self, TokenKind::LessThan),
            },
            Some('(') => single!(self, TokenKind::LeftParen),
            Some(')') => single!(self, TokenKind::RightParen),
            Some('{') => single!(self, TokenKind::LeftCurly),
            Some('}') => single!(self, TokenKind::RightCurly),
            Some('[') => single!(self, TokenKind::LeftBracket),
            Some(']') => single!(self, TokenKind::RightBracket),
            Some(c) => error!(InvalidSyntax::UnrecognizedChar(self.get_position(), c)),
            _ => None,
        };
    }

    fn next_identifier(&mut self, c: char) -> IteratorItem {
        let start = self.get_position();
        let mut identifier = String::from(c);
        loop {
            match self.chars.peek() {
                Some(&c) if c.is_ascii_alphabetic() || c.is_numeric() || c == '_' => {
                    self.position.next();
                    self.chars.next();
                    identifier.push(c);
                }
                _ => break,
            }
        }

        match TokenKind::str_to_identifier(&identifier) {
            Some(token_kind) => multi!(self, token_kind, start),
            _ => multi!(self, TokenKind::Identifier(identifier), start),
        }
    }

    fn next_string(&mut self) -> IteratorItem {
        let start = self.get_position();
        let mut string = String::new();
        while let Some(&c) = self.chars.peek() {
            if c == '"' {
                break;
            }
            self.chars.next();
            self.position.next();
            string.push(c);
        }

        match self.chars.next() {
            Some(c) if c == '"' => multi!(self, TokenKind::String(string), start),
            Some(_) => unreachable!(),
            None => {
                self.position.next();
                error!(InvalidSyntax::UnclosedCharDelimeter(
                    start,
                    self.get_position(),
                    '"',
                    None
                ))
            }
        }
    }

    fn next_char(&mut self) -> IteratorItem {
        let start = self.get_position();
        let result: char;

        match self.chars.next() {
            Some(c) => result = c,
            None => return error!(InvalidSyntax::UnexpectedChar(self.get_position(), '\'')),
        }

        self.position.next();
        return match self.chars.next() {
            Some('\'') => {
                self.position.next();
                multi!(self, TokenKind::Character(result), start)
            }
            Some(c) => {
                self.position.next();
                error!(InvalidSyntax::UnclosedCharDelimeter(
                    start,
                    self.get_position(),
                    '"',
                    Some(c),
                ))
            }
            None => {
                error!(InvalidSyntax::UnclosedCharDelimeter(
                    start,
                    self.get_position(),
                    '\'',
                    None
                ))
            }
        };
    }

    fn next_number(&mut self, c: char) -> IteratorItem {
        let start = self.get_position();
        let mut number = String::from(c);
        let mut is_float = false;

        loop {
            self.position.next();
            match self.chars.next() {
                Some(c) if c.is_numeric() => number.push(c),
                Some('.') => {
                    if !is_float {
                        number.push('.');
                        is_float = true;
                    } else {
                        return error!(InvalidSyntax::MultipleFloatingPoints(
                            start,
                            self.get_position()
                        ));
                    }
                }
                _ => break,
            }
        }

        if is_float {
            return match number.parse::<f32>() {
                Ok(float) => multi!(self, TokenKind::Float(float), start),
                _ => panic!("Couldn't parse float: {:?}", number),
            };
        }
        match number.parse::<i32>() {
            Ok(int) => multi!(self, TokenKind::Integer(int), start),
            _ => panic!("Couldn't parse integer: {:?}", number),
        }
    }
}

impl<Chars: Iterator<Item = char>> Iterator for Tokenizer<Peekable<Chars>> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Result<Token, Error>> {
        let current = self.current.take();
        self.current = self._next();
        current
    }
}
