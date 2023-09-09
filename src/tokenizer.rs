use crate::error::Error;
use crate::token::{Kind, Token};
use core::iter::Peekable;

macro_rules! token {
    ($self:ident, $kind:expr) => {
        Some(Ok(Token::new($kind, $self.row, $self.column)))
    };
}

pub struct Tokenizer<Chars: Iterator<Item = char>> {
    chars: Chars,
    row: usize,
    column: usize,
}

impl<Chars: Iterator<Item = char>> Tokenizer<Peekable<Chars>> {
    pub fn new(chars: Chars) -> Self {
        Self {
            chars: chars.peekable(),
            row: 1,
            column: 0,
        }
    }

    fn next_identifier(&mut self, c: char) -> Option<Result<Token, Error>> {
        let mut identifier = String::from(c);
        loop {
            match self.chars.peek() {
                Some(&c) if c.is_ascii_alphabetic() || c.is_numeric() || c == '_' => {
                    self.chars.next();
                    identifier.push(c);
                }
                _ => break,
            }
        }

        if let Some(kind) = Kind::str_to_identifier(&identifier) {
            return token!(self, kind);
        }
        token!(self, Kind::Identifier(identifier))
    }
}

impl<Chars: Iterator<Item = char>> Iterator for Tokenizer<Peekable<Chars>> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Result<Token, Error>> {
        self.column += 1;
        return match self.chars.next() {
            Some(' ') => self.next(),
            Some('\n') => {
                let token = token!(self, Kind::NewLine);
                self.row += 1;
                self.column = 0;
                token
            }
            Some(c) if c.is_ascii_alphabetic() || c == '_' => self.next_identifier(c),
            Some('+') => token!(self, Kind::Addition),
            Some('-') => match self.chars.peek() {
                Some('>') => {
                    self.chars.next();
                    token!(self, Kind::Arrow)
                }
                _ => token!(self, Kind::Subtraction),
            },
            Some('*') => token!(self, Kind::Multiplication),
            Some('/') => token!(self, Kind::Division),
            Some('%') => token!(self, Kind::Modulo),
            Some('^') => token!(self, Kind::Power),
            Some('=') => match self.chars.peek() {
                Some('>') => {
                    self.chars.next();
                    token!(self, Kind::DoubleArrow)
                }
                _ => token!(self, Kind::Equals),
            },
            Some('>') => match self.chars.peek() {
                Some('=') => {
                    self.chars.next();
                    token!(self, Kind::GreaterEq)
                }
                _ => token!(self, Kind::Greater),
            },
            Some('<') => match self.chars.peek() {
                Some('=') => {
                    self.chars.next();
                    token!(self, Kind::LessThanEq)
                }
                _ => token!(self, Kind::LessThan),
            },
            Some('(') => token!(self, Kind::RightParen),
            Some(')') => token!(self, Kind::LeftParen),
            Some('{') => token!(self, Kind::RightCurly),
            Some('}') => token!(self, Kind::LeftCurly),
            Some(c) => Some(Err(Error::UnrecognizedChar(c))),
            _ => None,
        };
    }
}
