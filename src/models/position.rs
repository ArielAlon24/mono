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
