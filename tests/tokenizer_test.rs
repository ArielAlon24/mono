use mono::error::Error;
use mono::token::{Kind, Token};
use mono::tokenizer::Tokenizer;

#[test]
fn identifiers() {
    let tokenizer = Tokenizer::new("and or x\ny".chars());
    let actual: Vec<_> = tokenizer.collect();
    let expected: Vec<Result<Token, Error>> = vec![
        Ok(Token::new(Kind::And, 1, 1)),
        Ok(Token::new(Kind::Or, 1, 5)),
        Ok(Token::new(Kind::Identifier(String::from("x")), 1, 8)),
        Ok(Token::new(Kind::NewLine, 1, 9)),
        Ok(Token::new(Kind::Identifier(String::from("y")), 2, 1)),
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
        Ok(Token::new(Kind::Boolean(true), 1, 1)),
        Ok(Token::new(Kind::NewLine, 1, 5)),
        Ok(Token::new(Kind::Boolean(false), 2, 1)),
        Ok(Token::new(Kind::NewLine, 2, 6)),
        Ok(Token::new(Kind::None, 3, 1)),
    ];

    for (actual_token, expected_token) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual_token, expected_token);
    }
}

#[test]
fn strings_and_chars() {
    let tokenizer = Tokenizer::new("\'a\'\n\"a\"\n\"mono\"\n\'mono\'".chars());
    let actual: Vec<_> = tokenizer.collect();
    let expected: Vec<Result<Token, Error>> = vec![
        Ok(Token::new(Kind::Character('a'), 1, 1)),
        Ok(Token::new(Kind::NewLine, 1, 4)),
        Ok(Token::new(Kind::String(String::from("a")), 2, 1)),
        Ok(Token::new(Kind::NewLine, 2, 4)),
        Ok(Token::new(Kind::String(String::from("mono")), 3, 1)),
        Ok(Token::new(Kind::NewLine, 3, 7)),
        Err(Error::InvalidSyntax {
            expected: vec!['\''],
            actual: vec!['o'],
        }),
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
        Ok(Token::new(Kind::Addition, 1, 1)),
        Ok(Token::new(Kind::Subtraction, 1, 3)),
        Ok(Token::new(Kind::Multiplication, 1, 5)),
        Ok(Token::new(Kind::Division, 1, 7)),
        Ok(Token::new(Kind::Modulo, 1, 9)),
        Ok(Token::new(Kind::Power, 1, 11)),
        Ok(Token::new(Kind::Assignment, 1, 13)),
        Ok(Token::new(Kind::Equals, 1, 15)),
        Ok(Token::new(Kind::Greater, 1, 18)),
        Ok(Token::new(Kind::GreaterEq, 1, 20)),
        Ok(Token::new(Kind::LessThan, 1, 23)),
        Ok(Token::new(Kind::LessThanEq, 1, 25)),
        Ok(Token::new(Kind::NotEquals, 1, 28)),
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
        Ok(Token::new(Kind::Arrow, 1, 1)),
        Ok(Token::new(Kind::DoubleArrow, 1, 4)),
    ];

    for (actual_token, expected_token) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual_token, expected_token);
    }
}

#[test]
fn arrows_and_operators() {
    //          12345
    //          0
    let code = "-><=>=";
    let tokenizer = Tokenizer::new(code.chars());
    let actual: Vec<_> = tokenizer.collect();
    let expected: Vec<Result<Token, Error>> = vec![
        Ok(Token::new(Kind::Arrow, 1, 1)),
        Ok(Token::new(Kind::LessThanEq, 1, 3)),
        Ok(Token::new(Kind::GreaterEq, 1, 5)),
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
        Ok(Token::new(Kind::RightParen, 1, 1)),
        Ok(Token::new(Kind::RightCurly, 1, 2)),
        Ok(Token::new(Kind::LeftCurly, 1, 3)),
        Ok(Token::new(Kind::RightParen, 1, 4)),
        Ok(Token::new(Kind::LeftParen, 1, 5)),
        Ok(Token::new(Kind::RightCurly, 1, 6)),
        Ok(Token::new(Kind::LeftCurly, 1, 7)),
        Ok(Token::new(Kind::RightParen, 1, 8)),
        Ok(Token::new(Kind::RightParen, 1, 9)),
        Ok(Token::new(Kind::LeftParen, 1, 10)),
        Ok(Token::new(Kind::LeftParen, 1, 11)),
    ];

    for (actual_token, expected_token) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual_token, expected_token);
    }
}
