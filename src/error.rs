#[derive(Debug, PartialEq)]
pub enum Error {
    UnrecognizedChar(char),
    InvalidSyntax {
        expected: Vec<char>,
        actual: Option<char>,
    },
}
