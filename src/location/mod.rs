
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Location {
    row: u32,
    column: u32
}

impl Location {
    pub fn new(row: u32, column: u32) -> Self {
        Self {
            row,
            column
        }
    }

    pub fn next_row(&mut self) {
        self.row += 1;
        self.column = 0;
    }

    pub fn next_col(&mut self) {
        self.column += 1;
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("lipu nanpa {}, sitelen nanpa {}", self.row, self.column))
    }
}