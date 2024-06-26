use std::fmt::Display;
use crate::parser::parse_error_type::ParseErrorType;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub error_type: ParseErrorType,
    pub row: usize,
    pub col: usize,
}

impl ParseError {
    pub fn new(error_type: ParseErrorType, row: usize, col: usize) -> Self {
        Self {
            error_type,
            row,
            col,
        }
    }
}

impl std::error::Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at row {}, col {}",
            self.error_type, self.row, self.col
        )
    }
}