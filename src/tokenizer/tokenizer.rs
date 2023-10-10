use crate::models::error::{Error, InvalidSyntax};
use crate::models::position::Position;
use crate::tokenizer::token::{Token, TokenKind};

use core::iter::Peekable;

/*
The single macro takes a position and token kind and creates
a token with those attributes (meaning with end position of
None) wrapped in an Ok result and Some option.
*/
#[macro_export]
macro_rules! single {
    ($Position:expr, $TokenKind:expr) => {
        Some(Ok(Token::new($Position.clone(), None, $TokenKind)))
    };
}

/*
The multi macro takes a start and end positions, and a token
kind and creates a token with those attributes wrapped in an
Ok result and Some option.
*/
#[macro_export]
macro_rules! multi {
    ($start:expr, $end:expr, $TokenKind:expr) => {
        Some(Ok(Token::new($start, Some($end.clone()), $TokenKind)))
    };
}

/*
The raw macro is a shortened way of using the Token new
function (constructor) and on the way wrapping it in Ok
result and Some option.
*/
#[macro_export]
macro_rules! raw {
    ($start:expr, $end:expr, $TokenKind:expr) => {
        Some(Ok(Token::new($start, $end, $TokenKind)))
    };
}

/*
The syntax_error macro takes an InvalidSyntax kind and
creates an Error type out of it wrapped in Err result and
Some option.
*/
#[macro_export]
macro_rules! syntax_error {
    ($ErrorKind:expr) => {
        Some(Err(Error::invalid_syntax($ErrorKind)))
    };
}

// TokenizerItem is the return type of an iteration on the tokenizer iterator
pub type TokenizerItem = Option<Result<Token, Error>>;

/*
--- Tokenizer (struct) ---

The Tokenizer struct is responsible for tokenizing the raw
input (Mono code files or REPL). In a way of implementing
an iterator whose elements are Tokens or Errors, that way no
memory is duplicated while creating this tokens.

It uses an overhead TokenizerItem to make peeking possible,
meaning the tokenizer knows every time one step ahead, and
when the next function is called the result is already
computed and the next after it is calculated.
*/
pub struct Tokenizer<Chars: Iterator<Item = char>> {
    chars: Chars,
    overhead: TokenizerItem,
    position: Position,
}

impl<Chars: Iterator<Item = char>> Tokenizer<Peekable<Chars>> {
    /*
    The new function is the constructor for a tokenizer, it
    takes an Character iterator of the raw input text and
    returns a Tokenizer struct out of it.
    */
    pub fn new(chars: Chars) -> Self {
        let mut tokenizer = Self {
            chars: chars.peekable(),
            overhead: None,
            position: Position::new(1, 0),
        };
        tokenizer.next();
        tokenizer
    }

    /*
    The peek method returns a reference for the next item of the
    tokenizer iterator (In reality this value is stored) inside
    the tokenizer and is not calculated on call.
    */
    pub fn peek(&mut self) -> &TokenizerItem {
        &self.overhead
    }

    /*
    The get_position method returns a deep copy of the current
    tokenizer position.
    */
    pub fn get_position(&self) -> Position {
        self.position.clone()
    }

    /*
    The _next method is the helper method of the next method
    that is implement for the iterator trait (interface). It
    uses a match block to check the current char and based on it
    preforms the correct task (calling to a token creating
    method or creating a token itself). Then, it returns the
    token it iterated on. But, in a case where an error occurred
    the error is returned.
    */
    fn _next(&mut self) -> TokenizerItem {
        self.position.next();

        if let Some(c) = self.chars.next() {
            match c {
                ' ' => self._next(),
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
                c => syntax_error!(InvalidSyntax::UnrecognizedChar {
                    position: self.position.clone(),
                    c
                }),
            }
        } else {
            None
        }
    }

    /*
    The next_line method is a token creating method that creates
    a NewLine TokenKind token and moves the tokenizer position
    one line down.
    */
    fn next_line(&mut self) -> TokenizerItem {
        let token = single!(self.position, TokenKind::NewLine);
        self.position.newline();
        token
    }

    /*
    The next_dash method is a token creating method that creates
    a token based on the char that comes after the dash that was
    seen. It returns the token it created.
    */
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

    /*
    The next_eqauls method is a token creating method that
    creates a token based on the char that comes after the
    equals char that was seen. It returns the token it created.
    */
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

    /*
    The next_exclemation method is a token creating method that
    creates a token based on the char that comes after the
    exclamation mark char that was seen. It returns the token
    it created.
    */
    fn next_exclemation(&mut self) -> TokenizerItem {
        match self.chars.next() {
            Some('=') => {
                let start = self.get_position();
                self.position.next();
                multi!(start, self.position, TokenKind::NotEquals)
            }
            _ => {
                self.position.next();
                syntax_error!(InvalidSyntax::UnexpectedChar {
                    position: self.get_position(),
                    c: '!'
                })
            }
        }
    }

    /*
    The next_greater method is a token creating method that
    creates a token based on the char that comes after the
    greater than char that was seen. It returns the token it
    created.
    */
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

    /*
    The less_than method is a token creating method that creates
    a token based on the char that comes after the less than
    char that was seen. It returns the token it created.
    */
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

    /*
    The next_identifier method is a token creating method that
    creates an identifier token, meaning it consumes every
    alphabetic/numeric/'_' char that comes after the first one
    it encountered and create a token out of it.
    */
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

    /*
    The next_identifier method is a token creating method that
    creates an identifier token, meaning it consumes every
    char that came after the first `"` it seen and creates a
    String TokenKind out of it and returns it. Also, in a case
    where the closing `"` was not found an Error is returned.
    */
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
                syntax_error!(InvalidSyntax::UnclosedStringDelimeter { start })
            }
        }
    }

    /*
    The next_char method is a token creating method that creates
    an identifier token, meaning it consumes a single char that
    came after the first `'` it seen and creates a Char
    TokenKind out of it and returns it. Also, in a case
    where the closing `'` was not found an Error is returned.
    */
    fn next_char(&mut self) -> TokenizerItem {
        let start = self.get_position();
        let result: char;

        match self.chars.next() {
            Some(c) => {
                result = c;
                self.position.next();
            }
            None => {
                return syntax_error!(InvalidSyntax::UnexpectedChar {
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
                syntax_error!(InvalidSyntax::UnclosedCharDelimeter {
                    start,
                    end: self.get_position(),
                    found: Some(c),
                })
            }
            None => syntax_error!(InvalidSyntax::UnclosedCharDelimeter {
                start,
                end: self.get_position(),
                found: None,
            }),
        };
    }

    /*
    The next_number method is a token creating method that
    consumes every number or `.` to create a number and returns
    the a token created using this number. In a case where
    errors occur like multiple floating points or numbers that
    are bigger or smaller than a i32 or f32 a corresponding
    Error is returned.
    */
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
                        return syntax_error!(InvalidSyntax::MultipleFloatingPoints {
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
                _ => syntax_error!(InvalidSyntax::InvalidFloatSize {
                    start: start,
                    end: end.unwrap(),
                }),
            };
        }
        match number.parse::<i32>() {
            Ok(int) => raw!(start, end, TokenKind::Integer(int)),
            _ => syntax_error!(InvalidSyntax::InvalidIntegerSize {
                start: start,
                end: end.unwrap(),
            }),
        }
    }
}

// This impl block is the implementation of an Iterator for the Tokenizer struct.
impl<Chars: Iterator<Item = char>> Iterator for Tokenizer<Peekable<Chars>> {
    // The Item the Iterator uses
    type Item = Result<Token, Error>;

    /*
    The next method is the method required by the Iterator trait
    to be created for a struct to implement the Iterator trait.

    The method returns the overhead item and tokenizes the next
    item to store it in the overhead.
    */
    fn next(&mut self) -> Option<Result<Token, Error>> {
        let current = self.overhead.take();
        self.overhead = self._next();
        current
    }
}
