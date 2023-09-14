use crate::models::error::{Error, InvalidSyntax, InvalidSyntaxKind};
use crate::models::position::Position;
use crate::models::token::{Token, TokenKind};
use core::iter::Peekable;

macro_rules! token {
    ($self:ident, $TokenKind:expr) => {
        Some(Ok(Token::new($TokenKind, $self.row, $self.column)))
    };
}

macro_rules! error {
    ($self:ident, $ErrorKind:expr) => {
        Some(Err(Error::InvalidSyntaxError(InvalidSyntax::new(
            $ErrorKind,
            $self.get_position(),
        ))))
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

    fn get_position(&self) -> Position {
        Position::new(self.row, self.column)
    }

    fn next_identifier(&mut self, c: char) -> Option<Result<Token, Error>> {
        let mut length = 0;
        let mut identifier = String::from(c);
        loop {
            match self.chars.peek() {
                Some(&c) if c.is_ascii_alphabetic() || c.is_numeric() || c == '_' => {
                    length += 1;
                    self.chars.next();
                    identifier.push(c);
                }
                _ => break,
            }
        }

        let token: Option<Result<Token, Error>> = match TokenKind::str_to_identifier(&identifier) {
            Some(token_kind) => token!(self, token_kind),
            _ => token!(self, TokenKind::Identifier(identifier)),
        };

        self.column += length;

        token
    }

    fn next_string(&mut self) -> Option<Result<Token, Error>> {
        let mut length = 0;
        let mut string = String::new();
        while let Some(&c) = self.chars.peek() {
            if c == '"' {
                break;
            }
            self.chars.next();
            length += 1;
            string.push(c);
        }

        return match self.chars.next() {
            Some(c) if c == '"' => {
                let token = token!(self, TokenKind::String(string));
                self.column += length + 1; // +1 for the '"'
                token
            }
            Some(_) => unreachable!(),
            None => {
                self.column += length + 1;
                error!(self, InvalidSyntaxKind::UnclosedCharDelimeter('"', None))
            }
        };
    }

    fn next_char(&mut self) -> Option<Result<Token, Error>> {
        let result: char;

        match self.chars.next() {
            Some(c) => result = c,
            None => return error!(self, InvalidSyntaxKind::UnexpectedChar('\'')),
        }

        return match self.chars.next() {
            Some('\'') => {
                let token = token!(self, TokenKind::Character(result));
                self.column += 2;
                token
            }
            Some(c) => {
                self.column += 2;
                error!(
                    self,
                    InvalidSyntaxKind::UnclosedCharDelimeter('\'', Some(c))
                )
            }
            None => error!(self, InvalidSyntaxKind::UnclosedCharDelimeter('"', None)),
        };
    }

    fn next_number(&mut self, c: char) -> Option<Result<Token, Error>> {
        let column = self.column;
        let mut number = String::from(c);
        let mut is_float = false;

        loop {
            match self.chars.peek() {
                Some(&c) if c.is_numeric() => {
                    self.chars.next();
                    self.column += 1;
                    number.push(c);
                }
                Some('.') => {
                    self.chars.next();
                    self.column += 1;
                    if !is_float {
                        number.push('.');
                        is_float = true;
                    } else {
                        return error!(self, InvalidSyntaxKind::MultipleFloatingPoints);
                    }
                }
                _ => break,
            }
        }

        if is_float {
            return match number.parse::<f32>() {
                Ok(float) => Some(Ok(Token::new(TokenKind::Float(float), self.row, column))),
                _ => panic!("Couldn't parse float: {:?}", number),
            };
        }
        match number.parse::<i32>() {
            Ok(int) => Some(Ok(Token::new(TokenKind::Integer(int), self.row, column))),
            _ => panic!("Couldn't parse integer: {:?}", number),
        }
    }
}

impl<Chars: Iterator<Item = char>> Iterator for Tokenizer<Peekable<Chars>> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Result<Token, Error>> {
        self.column += 1;
        return match self.chars.next() {
            Some(' ') => self.next(),
            Some('\n') => {
                let token = token!(self, TokenKind::NewLine);
                self.row += 1;
                self.column = 0;
                token
            }
            Some(c) if c.is_ascii_alphabetic() || c == '_' => self.next_identifier(c),
            Some(c) if c == '"' => self.next_string(),
            Some(c) if c == '\'' => self.next_char(),
            Some(c) if c.is_numeric() => self.next_number(c),
            Some('+') => token!(self, TokenKind::Addition),
            Some('-') => match self.chars.peek() {
                Some('>') => {
                    self.chars.next();
                    let token = token!(self, TokenKind::Arrow);
                    self.column += 1;
                    token
                }
                _ => token!(self, TokenKind::Subtraction),
            },
            Some('*') => token!(self, TokenKind::Multiplication),
            Some('/') => token!(self, TokenKind::Division),
            Some('%') => token!(self, TokenKind::Modulo),
            Some('^') => token!(self, TokenKind::Power),
            Some('=') => match self.chars.peek() {
                Some('>') => {
                    self.chars.next();
                    let token = token!(self, TokenKind::DoubleArrow);
                    self.column += 1;
                    token
                }
                Some('=') => {
                    self.chars.next();
                    let token = token!(self, TokenKind::Equals);
                    self.column += 1;
                    token
                }
                _ => token!(self, TokenKind::Assignment),
            },
            Some('!') => match self.chars.next() {
                Some('=') => {
                    let token = token!(self, TokenKind::NotEquals);
                    self.column += 1;
                    token
                }
                Some(_) => {
                    self.column += 1;
                    error!(self, InvalidSyntaxKind::UnexpectedChar('!'))
                }
                None => error!(self, InvalidSyntaxKind::UnexpectedChar('!')),
            },
            Some('>') => match self.chars.peek() {
                Some('=') => {
                    self.chars.next();
                    let token = token!(self, TokenKind::GreaterEq);
                    self.column += 1;
                    token
                }
                _ => token!(self, TokenKind::Greater),
            },
            Some('<') => match self.chars.peek() {
                Some('=') => {
                    self.chars.next();
                    let token = token!(self, TokenKind::LessThanEq);
                    self.column += 1;
                    token
                }
                _ => token!(self, TokenKind::LessThan),
            },
            Some('(') => token!(self, TokenKind::LeftParen),
            Some(')') => token!(self, TokenKind::RightParen),
            Some('{') => token!(self, TokenKind::LeftCurly),
            Some('}') => token!(self, TokenKind::RightCurly),
            Some('[') => token!(self, TokenKind::LeftBracket),
            Some(']') => token!(self, TokenKind::RightBracket),
            Some(c) => error!(self, InvalidSyntaxKind::UnrecognizedChar(c)),
            _ => None,
        };
    }
}
