use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    row: usize,
    column: usize,
}

impl Position {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn next(&mut self) {
        self.column += 1;
    }

    pub fn newline(&mut self) {
        self.row += 1;
        self.column = 0;
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.row, self.column)
    }
}
