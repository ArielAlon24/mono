use mono::error::{Error, ErrorKind};
use mono::position::Position;
use mono::token::{Token, TokenKind};
use mono::tokenizer::Tokenizer;

#[test]
fn identifiers() {
    let tokenizer = Tokenizer::new("and or x\ny".chars());
    let actual: Vec<_> = tokenizer.collect();
    let expected: Vec<Result<Token, Error>> = vec![
        Ok(Token::new(TokenKind::And, 1, 1)),
        Ok(Token::new(TokenKind::Or, 1, 5)),
        Ok(Token::new(TokenKind::Identifier(String::from("x")), 1, 8)),
        Ok(Token::new(TokenKind::NewLine, 1, 9)),
        Ok(Token::new(TokenKind::Identifier(String::from("y")), 2, 1)),
    ];

    for (actual_token, expected_token) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual_token, expected_token);
    }
}

#[test]
fn booleans_and_none() {
    let tokenizer = Tokenizer::new("true\nfalse\nnone".chars());
    let actual: Vec<_> = tokenizer.collect();
    let expected: Vec<Result<Token, Error>> = vec![
        Ok(Token::new(TokenKind::Boolean(true), 1, 1)),
        Ok(Token::new(TokenKind::NewLine, 1, 5)),
        Ok(Token::new(TokenKind::Boolean(false), 2, 1)),
        Ok(Token::new(TokenKind::NewLine, 2, 6)),
        Ok(Token::new(TokenKind::None, 3, 1)),
    ];

    for (actual_token, expected_token) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual_token, expected_token);
    }
}

#[test]
fn strings_and_chars() {
    //                               1      2      3       4
    //                               12 3 4 12 3 4 12345 6 1 23456 7
    let tokenizer = Tokenizer::new("\'a\'\n\"a\"\n\"mono\"\n\'mono\'".chars());
    let actual: Vec<_> = tokenizer.collect();
    let expected: Vec<Result<Token, Error>> = vec![
        Ok(Token::new(TokenKind::Character('a'), 1, 1)),
        Ok(Token::new(TokenKind::NewLine, 1, 4)),
        Ok(Token::new(TokenKind::String(String::from("a")), 2, 1)),
        Ok(Token::new(TokenKind::NewLine, 2, 4)),
        Ok(Token::new(TokenKind::String(String::from("mono")), 3, 1)),
        Ok(Token::new(TokenKind::NewLine, 3, 7)),
        Err(Error::new_char(
            ErrorKind::InvalidSyntax(vec!['\''], Some('o')),
            Position::new(4, 3),
        )),
    ];

    for (actual_token, expected_token) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual_token, expected_token);
    }
}

#[test]
fn string_unclosed_delimeter() {
    //                               1
    //                               123456
    let tokenizer = Tokenizer::new("\"mono".chars());
    let actual: Vec<_> = tokenizer.collect();
    let expected: Vec<Result<Token, Error>> = vec![Err(Error::new_char(
        ErrorKind::UnclosedDelimeter('"'),
        Position::new(1, 6),
    ))];

    for (actual_token, expected_token) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual_token, expected_token);
    }
}

#[test]
fn char_unclosed_delimeter() {
    //                              1
    //                              123456
    let tokenizer = Tokenizer::new("'a' '".chars());
    let actual: Vec<_> = tokenizer.collect();
    let expected: Vec<Result<Token, Error>> = vec![
        Ok(Token::new(TokenKind::Character('a'), 1, 1)),
        Err(Error::new_char(
            ErrorKind::UnclosedDelimeter('\''),
            Position::new(1, 6),
        )),
    ];

    for (actual_token, expected_token) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual_token, expected_token);
    }
}

#[test]
fn numbers() {
    //                              1234567890123456789
    //                              1        2
    let tokenizer = Tokenizer::new("123 1.23 12.3 1.2.3".chars());
    let actual: Vec<_> = tokenizer.collect();
    let expected: Vec<Result<Token, Error>> = vec![
        Ok(Token::new(TokenKind::Integer(123), 1, 1)),
        Ok(Token::new(TokenKind::Float(1.23), 1, 5)),
        Ok(Token::new(TokenKind::Float(12.3), 1, 10)),
        Err(Error::new_char(
            ErrorKind::UnexpectedChar('.'),
            Position::new(1, 18),
        )),
    ];

    for (actual_token, expected_token) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual_token, expected_token);
    }
}

#[test]
fn opreators() {
    //          123456789012345678901234567
    //          0        1         2
    let code = "+ - * / % ^ = == > >= < <= !=";
    let tokenizer = Tokenizer::new(code.chars());
    let actual: Vec<_> = tokenizer.collect();
    let expected: Vec<Result<Token, Error>> = vec![
        Ok(Token::new(TokenKind::Addition, 1, 1)),
        Ok(Token::new(TokenKind::Subtraction, 1, 3)),
        Ok(Token::new(TokenKind::Multiplication, 1, 5)),
        Ok(Token::new(TokenKind::Division, 1, 7)),
        Ok(Token::new(TokenKind::Modulo, 1, 9)),
        Ok(Token::new(TokenKind::Power, 1, 11)),
        Ok(Token::new(TokenKind::Assignment, 1, 13)),
        Ok(Token::new(TokenKind::Equals, 1, 15)),
        Ok(Token::new(TokenKind::Greater, 1, 18)),
        Ok(Token::new(TokenKind::GreaterEq, 1, 20)),
        Ok(Token::new(TokenKind::LessThan, 1, 23)),
        Ok(Token::new(TokenKind::LessThanEq, 1, 25)),
        Ok(Token::new(TokenKind::NotEquals, 1, 28)),
    ];

    for (actual_token, expected_token) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual_token, expected_token);
    }
}

#[test]
fn arrows() {
    //          12345
    //          0
    let code = "-> =>";
    let tokenizer = Tokenizer::new(code.chars());
    let actual: Vec<_> = tokenizer.collect();
    let expected: Vec<Result<Token, Error>> = vec![
        Ok(Token::new(TokenKind::Arrow, 1, 1)),
        Ok(Token::new(TokenKind::DoubleArrow, 1, 4)),
    ];

    for (actual_token, expected_token) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual_token, expected_token);
    }
}

#[test]
fn arrows_and_operators() {
    //          12345
    //          0
    let code = "-><=>==>";
    let tokenizer = Tokenizer::new(code.chars());
    let actual: Vec<_> = tokenizer.collect();
    let expected: Vec<Result<Token, Error>> = vec![
        Ok(Token::new(TokenKind::Arrow, 1, 1)),
        Ok(Token::new(TokenKind::LessThanEq, 1, 3)),
        Ok(Token::new(TokenKind::GreaterEq, 1, 5)),
        Ok(Token::new(TokenKind::DoubleArrow, 1, 7)),
    ];

    for (actual_token, expected_token) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual_token, expected_token);
    }
}

#[test]
fn brackets() {
    //          01234567890
    //          0         1
    let code = "({}(){}(())";
    let tokenizer = Tokenizer::new(code.chars());
    let actual: Vec<_> = tokenizer.collect();
    let expected: Vec<Result<Token, Error>> = vec![
        Ok(Token::new(TokenKind::RightParen, 1, 1)),
        Ok(Token::new(TokenKind::RightCurly, 1, 2)),
        Ok(Token::new(TokenKind::LeftCurly, 1, 3)),
        Ok(Token::new(TokenKind::RightParen, 1, 4)),
        Ok(Token::new(TokenKind::LeftParen, 1, 5)),
        Ok(Token::new(TokenKind::RightCurly, 1, 6)),
        Ok(Token::new(TokenKind::LeftCurly, 1, 7)),
        Ok(Token::new(TokenKind::RightParen, 1, 8)),
        Ok(Token::new(TokenKind::RightParen, 1, 9)),
        Ok(Token::new(TokenKind::LeftParen, 1, 10)),
        Ok(Token::new(TokenKind::LeftParen, 1, 11)),
    ];

    for (actual_token, expected_token) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual_token, expected_token);
    }
}

#[test]
fn full() {
    let code = "123 1.23 12.3 123.
\"mono\" 
'm' 'o' 'n' 'o'
or and not none false true identifier
+ - * / % ^ = == != > >= < <=
( ) { }
-> => ";
    let tokenizer = Tokenizer::new(code.chars());
    let actual: Vec<_> = tokenizer.collect();
    let expected: Vec<Result<Token, Error>> = vec![
        Ok(Token::new(TokenKind::Integer(123), 1, 1)),
        Ok(Token::new(TokenKind::Float(1.23), 1, 5)),
        Ok(Token::new(TokenKind::Float(12.3), 1, 10)),
        Ok(Token::new(TokenKind::Float(123.0), 1, 15)),
        Ok(Token::new(TokenKind::NewLine, 1, 19)),
        Ok(Token::new(TokenKind::String("mono".to_string()), 2, 1)),
        Ok(Token::new(TokenKind::NewLine, 2, 8)),
        Ok(Token::new(TokenKind::Character('m'), 3, 1)),
        Ok(Token::new(TokenKind::Character('o'), 3, 5)),
        Ok(Token::new(TokenKind::Character('n'), 3, 9)),
        Ok(Token::new(TokenKind::Character('o'), 3, 13)),
        Ok(Token::new(TokenKind::NewLine, 3, 16)),
        Ok(Token::new(TokenKind::Or, 4, 1)),
        Ok(Token::new(TokenKind::And, 4, 4)),
        Ok(Token::new(TokenKind::Not, 4, 8)),
        Ok(Token::new(TokenKind::None, 4, 12)),
        Ok(Token::new(TokenKind::Boolean(false), 4, 17)),
        Ok(Token::new(TokenKind::Boolean(true), 4, 23)),
        Ok(Token::new(
            TokenKind::Identifier("identifier".to_string()),
            4,
            28,
        )),
        Ok(Token::new(TokenKind::NewLine, 4, 38)),
        Ok(Token::new(TokenKind::Addition, 5, 1)),
        Ok(Token::new(TokenKind::Subtraction, 5, 3)),
        Ok(Token::new(TokenKind::Multiplication, 5, 5)),
        Ok(Token::new(TokenKind::Division, 5, 7)),
        Ok(Token::new(TokenKind::Modulo, 5, 9)),
        Ok(Token::new(TokenKind::Power, 5, 11)),
        Ok(Token::new(TokenKind::Assignment, 5, 13)),
        Ok(Token::new(TokenKind::Equals, 5, 15)),
        Ok(Token::new(TokenKind::NotEquals, 5, 18)),
        Ok(Token::new(TokenKind::Greater, 5, 21)),
        Ok(Token::new(TokenKind::GreaterEq, 5, 23)),
        Ok(Token::new(TokenKind::LessThan, 5, 26)),
        Ok(Token::new(TokenKind::LessThanEq, 5, 28)),
        Ok(Token::new(TokenKind::NewLine, 5, 30)),
        Ok(Token::new(TokenKind::RightParen, 6, 1)),
        Ok(Token::new(TokenKind::LeftParen, 6, 3)),
        Ok(Token::new(TokenKind::RightCurly, 6, 5)),
        Ok(Token::new(TokenKind::LeftCurly, 6, 7)),
        Ok(Token::new(TokenKind::NewLine, 6, 8)),
        Ok(Token::new(TokenKind::Arrow, 7, 1)),
        Ok(Token::new(TokenKind::DoubleArrow, 7, 4)),
    ];

    for (actual_token, expected_token) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual_token, expected_token);
    }
}
