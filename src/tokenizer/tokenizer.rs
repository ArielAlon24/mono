use crate::models::error::{Error, Syntax};
use crate::models::position::Position;
use crate::tokenizer::token::{Token, TokenKind};

use core::iter::Peekable;

#[macro_export]
macro_rules! single {
    ($Position:expr, $TokenKind:expr) => {
        Some(Ok(Token::new($Position.clone(), None, $TokenKind)))
    };
}

#[macro_export]
macro_rules! multi {
    ($start:expr, $end:expr, $TokenKind:expr) => {
        Some(Ok(Token::new($start, Some($end.clone()), $TokenKind)))
    };
}

#[macro_export]
macro_rules! raw {
    ($start:expr, $end:expr, $TokenKind:expr) => {
        Some(Ok(Token::new($start, $end, $TokenKind)))
    };
}

#[macro_export]
macro_rules! error {
    ($ErrorKind:expr) => {
        Some(Err($ErrorKind.into()))
    };
}

pub type TokenizerItem = Option<Result<Token, Error>>;

pub struct Tokenizer<Chars: Iterator<Item = char>> {
    chars: Chars,
    overhead: TokenizerItem,
    position: Position,
}

impl<Chars: Iterator<Item = char>> Tokenizer<Peekable<Chars>> {
    pub fn new(chars: Chars) -> Self {
        let mut tokenizer = Self {
            chars: chars.peekable(),
            overhead: None,
            position: Position::new(1, 0),
        };
        tokenizer.next();
        tokenizer
    }

    pub fn peek(&mut self) -> &TokenizerItem {
        &self.overhead
    }

    pub fn get_position(&self) -> Position {
        self.position.clone()
    }

    fn _next(&mut self) -> TokenizerItem {
        self.position.next();

        if let Some(c) = self.chars.next() {
            match c {
                ' ' => self._next(),
                '#' => self.next_comment(),
                '+' => single!(self.position, TokenKind::Add),
                '*' => single!(self.position, TokenKind::Mul),
                '/' => single!(self.position, TokenKind::Div),
                '%' => single!(self.position, TokenKind::Mod),
                '^' => single!(self.position, TokenKind::Pow),
                '(' => single!(self.position, TokenKind::LeftParen),
                ')' => single!(self.position, TokenKind::RightParen),
                '{' => single!(self.position, TokenKind::LeftCurly),
                '}' => single!(self.position, TokenKind::RightCurly),
                '[' => single!(self.position, TokenKind::LeftBracket),
                ']' => single!(self.position, TokenKind::RightBracket),
                '\n' => self.next_line(),
                '-' => self.next_dash(),
                '=' => self.next_equals(),
                '!' => self.next_exclemation(),
                '>' => self.next_greater(),
                '<' => self.next_less_than(),
                '"' => self.next_string(),
                '\'' => self.next_char(),
                c if c.is_ascii_alphabetic() || c == '_' => self.next_identifier(c),
                c if c.is_numeric() => self.next_number(c),
                c => error!(Syntax::UnrecognizedChar {
                    position: self.position.clone(),
                    c,
                }),
            }
        } else {
            None
        }
    }

    fn next_comment(&mut self) -> TokenizerItem {
        while let Some(c) = self.chars.next() {
            if c == '\n' {
                return self.next_line();
            }
        }
        None
    }

    fn next_line(&mut self) -> TokenizerItem {
        let token = single!(self.position, TokenKind::NewLine);
        self.position.newline();
        token
    }

    fn next_dash(&mut self) -> TokenizerItem {
        match self.chars.peek() {
            Some('>') => {
                let start = self.get_position();
                self.position.next();
                self.chars.next();
                multi!(start, self.position, TokenKind::Arrow)
            }
            _ => single!(self.position, TokenKind::Sub),
        }
    }

    fn next_equals(&mut self) -> TokenizerItem {
        match self.chars.peek() {
            Some('>') => {
                let start = self.get_position();
                self.chars.next();
                self.position.next();
                multi!(start, self.position, TokenKind::DoubleArrow)
            }
            Some('=') => {
                let start = self.get_position();
                self.chars.next();
                self.position.next();
                multi!(start, self.position, TokenKind::Equals)
            }
            _ => single!(self.position, TokenKind::Assignment),
        }
    }

    fn next_exclemation(&mut self) -> TokenizerItem {
        match self.chars.next() {
            Some('=') => {
                let start = self.get_position();
                self.position.next();
                multi!(start, self.position, TokenKind::NotEquals)
            }
            _ => {
                self.position.next();
                error!(Syntax::UnexpectedChar {
                    position: self.get_position(),
                    c: '!'
                })
            }
        }
    }

    fn next_greater(&mut self) -> TokenizerItem {
        match self.chars.peek() {
            Some('=') => {
                let start = self.get_position();
                self.chars.next();
                self.position.next();
                multi!(start, self.position, TokenKind::GreaterEq)
            }
            _ => single!(self.position, TokenKind::Greater),
        }
    }

    fn next_less_than(&mut self) -> TokenizerItem {
        match self.chars.peek() {
            Some('=') => {
                let start = self.get_position();
                self.chars.next();
                self.position.next();
                multi!(start, self.position, TokenKind::LessThanEq)
            }
            _ => single!(self.position, TokenKind::LessThan),
        }
    }

    fn next_identifier(&mut self, c: char) -> TokenizerItem {
        let start = self.get_position();
        let mut identifier = String::from(c);

        while let Some(c) = self.chars.peek() {
            if c.is_ascii_alphabetic() || c.is_numeric() || c == &'_' {
                self.position.next();
                identifier.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }

        let end = if &self.position == &start {
            None
        } else {
            Some(self.position.clone())
        };

        match TokenKind::from_str(&identifier) {
            Some(token_kind) => raw!(start, end, token_kind),
            _ => raw!(start, end, TokenKind::Identifier(identifier)),
        }
    }

    fn next_string(&mut self) -> TokenizerItem {
        let start = self.get_position();
        let mut string = String::new();
        while let Some(&c) = self.chars.peek() {
            if c == '"' {
                break;
            }
            self.position.next();
            string.push(self.chars.next().unwrap());
        }

        match self.chars.next() {
            Some(c) if c == '"' => multi!(start, self.position, TokenKind::String(string)),
            Some(_) => unreachable!(),
            None => {
                self.position.next();
                error!(Syntax::UnclosedStringDelimeter { start })
            }
        }
    }

    fn next_char(&mut self) -> TokenizerItem {
        let start = self.get_position();
        let result: char;

        match self.chars.next() {
            Some(c) => {
                result = c;
                self.position.next();
            }
            None => {
                return error!(Syntax::UnexpectedChar {
                    position: self.get_position(),
                    c: '\''
                })
            }
        }

        return match self.chars.next() {
            Some('\'') => {
                self.position.next();
                multi!(start, self.position, TokenKind::Character(result))
            }
            Some(c) => {
                self.position.next();
                error!(Syntax::UnclosedCharDelimeter {
                    start,
                    end: self.get_position(),
                    found: Some(c),
                })
            }
            None => error!(Syntax::UnclosedCharDelimeter {
                start,
                end: self.get_position(),
                found: None,
            }),
        };
    }

    fn next_number(&mut self, c: char) -> TokenizerItem {
        let start = self.get_position();
        let mut number = String::from(c);
        let mut is_float = false;

        loop {
            match self.chars.peek() {
                Some(c) if c.is_numeric() => {
                    number.push(self.chars.next().unwrap());
                    self.position.next();
                }
                Some('.') => {
                    self.position.next();
                    if is_float {
                        return error!(Syntax::MultipleFloatingPoints {
                            start,
                            end: self.get_position(),
                        });
                    }
                    number.push(self.chars.next().unwrap());
                    is_float = true;
                }
                _ => break,
            }
        }

        let end = if &self.position == &start {
            None
        } else {
            Some(self.position.clone())
        };

        if is_float {
            return match number.parse::<f32>() {
                Ok(float) => raw!(start, end, TokenKind::Float(float)),
                _ => error!(Syntax::InvalidFloatSize {
                    start: start,
                    end: end.unwrap(),
                }),
            };
        }
        match number.parse::<i32>() {
            Ok(int) => raw!(start, end, TokenKind::Integer(int)),
            _ => error!(Syntax::InvalidIntegerSize {
                start: start,
                end: end.unwrap(),
            }),
        }
    }
}

impl<Chars: Iterator<Item = char>> Iterator for Tokenizer<Peekable<Chars>> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Result<Token, Error>> {
        let current = self.overhead.take();
        self.overhead = self._next();
        current
    }
}
