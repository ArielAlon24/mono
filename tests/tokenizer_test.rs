use mono::error::Error;
use mono::token::{Kind, Token};
use mono::tokenizer::Tokenizer;

#[test]
fn identifiers() {
    let tokenizer = Tokenizer::new("and or".chars());
    let actual: Vec<_> = tokenizer.collect();
    let expected: Vec<Result<Token, Error>> = vec![
        Ok(Token::new(Kind::And, 1, 1)),
        Ok(Token::new(Kind::Or, 1, 5)),
    ];

    for (actual_token, expected_token) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual_token, expected_token);
    }
}

#[test]
fn opreators() {
    //          123456789012345678901234567
    //          0        1         2
    let code = "+ - * / % ^ = == > >= < <= ";
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
