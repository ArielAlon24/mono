use std::fmt;
/*
--- Position (struct) ---

The Value enum is an enum containing all the possible values
The Position struct in a struct for a position on a given
mono code. It has two attributes, row and column together
they can represent the a position inside a document.
*/
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    row: usize,
    column: usize,
}

impl Position {
    /*
    The new function is a constructor for creating a Position.
    it takes both row and column and returns a Position out of
    them.
    */
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    /*
    The next method advances the position one column ahead. Thus
    acts like a reading of one char.
    */
    pub fn next(&mut self) {
        self.column += 1;
    }

    /*
    The newline method advances the position one line bottom.
    */
    pub fn newline(&mut self) {
        self.row += 1;
        self.column = 0;
    }
}

impl fmt::Display for Position {
    /*
    The fmt (toString) method takes the default formatter and
    formats the self Position for a specific format later used
    in error formatting.
    */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.row, self.column)
    }
}
