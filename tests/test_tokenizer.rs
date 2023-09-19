use mono::models::error::{Error, InvalidSyntax};
use mono::models::position::Position;
use mono::tokenizer::token::{Token, TokenKind};
use mono::tokenizer::tokenizer::Tokenizer;
use mono::{multi, single, syntax_error};

macro_rules! tokenizer_test {
    ($test_name:ident, $input:expr, $tokens:expr) => {
        #[test]
        fn $test_name() {
            let tokenizer = Tokenizer::new($input.chars());
            let actual: Vec<_> = tokenizer.collect();

            for (actual_token, expected_token) in actual.iter().zip($tokens.iter()) {
                assert_eq!(Some(actual_token), expected_token.as_ref());
            }
        }
    };
}

tokenizer_test! {
    identifiers,
    "x y z\ntrue false none\nnot and or",
    vec![
        single!(Position::new(1, 1), TokenKind::Identifier("x".to_string())),
        single!(Position::new(1, 3), TokenKind::Identifier("y".to_string())),
        single!(Position::new(1, 5), TokenKind::Identifier("z".to_string())),
        single!(Position::new(1, 6), TokenKind::NewLine),
        multi!(Position::new(2, 1), Position::new(2, 4), TokenKind::Boolean(true)),
        multi!(Position::new(2, 6), Position::new(2, 10), TokenKind::Boolean(false)),
        multi!(Position::new(2, 12), Position::new(2, 15), TokenKind::None),
        single!(Position::new(2, 16), TokenKind::NewLine),
        multi!(Position::new(3, 1), Position::new(3, 3), TokenKind::Not),
        multi!(Position::new(3, 5), Position::new(3, 7), TokenKind::And),
        multi!(Position::new(3, 9), Position::new(3, 10), TokenKind::Or),
    ]
}

tokenizer_test! {
    unrecognized_char,
    "x y z@",
    vec![
        single!(Position::new(1, 1), TokenKind::Identifier("x".to_string())),
        single!(Position::new(1, 3), TokenKind::Identifier("y".to_string())),
        single!(Position::new(1, 5), TokenKind::Identifier("z".to_string())),
        syntax_error!(InvalidSyntax::UnrecognizedChar(Position::new(1, 6), '@')),
    ]
}

tokenizer_test! {
    brackets_and_parentheses,
    "({[]})",
    vec![
        single!(Position::new(1, 1), TokenKind::LeftParen),
        single!(Position::new(1, 2), TokenKind::LeftCurly),
        single!(Position::new(1, 3), TokenKind::LeftBracket),
        single!(Position::new(1, 4), TokenKind::RightBracket),
        single!(Position::new(1, 5), TokenKind::RightCurly),
        single!(Position::new(1, 6), TokenKind::RightParen),
    ]
}

tokenizer_test! {
    comparison_operators,
    "== != > >= < <=",
    vec![
        multi!(Position::new(1, 1), Position::new(1, 2), TokenKind::Equals),
        multi!(Position::new(1, 4), Position::new(1, 5), TokenKind::NotEquals),
        single!(Position::new(1, 7), TokenKind::Greater),
        multi!(Position::new(1, 9), Position::new(1, 10), TokenKind::GreaterEq),
        single!(Position::new(1, 12), TokenKind::LessThan),
        multi!(Position::new(1, 14), Position::new(1, 15), TokenKind::LessThanEq),
    ]
}

tokenizer_test! {
    arithmetic_operators,
    "+ - * / % ^",
    vec![
        single!(Position::new(1, 1), TokenKind::Add),
        single!(Position::new(1, 3), TokenKind::Sub),
        single!(Position::new(1, 5), TokenKind::Mul),
        single!(Position::new(1, 7), TokenKind::Div),
        single!(Position::new(1, 9), TokenKind::Mod),
        single!(Position::new(1, 11), TokenKind::Pow),
    ]
}

tokenizer_test! {
    numeric_literals_test,
    "123 12.3 1.23 0.01 1000",
    vec![
        multi!(Position::new(1, 1), Position::new(1, 3), TokenKind::Integer(123)),
        multi!(Position::new(1, 5), Position::new(1, 8), TokenKind::Float(12.3)),
        multi!(Position::new(1, 10), Position::new(1, 13), TokenKind::Float(1.23)),
        multi!(Position::new(1, 15), Position::new(1, 18), TokenKind::Float(0.01)),
        multi!(Position::new(1, 20), Position::new(1, 23), TokenKind::Integer(1000)),
    ]
}

tokenizer_test! {
    string_and_char_literals_test,
    "\"Hello, World!\" 'A'",
    vec![
        multi!(Position::new(1, 1), Position::new(1, 14), TokenKind::String("Hello, World!".to_string())),
        multi!(Position::new(1, 16), Position::new(1, 18), TokenKind::Character('A')),
    ]
}
