use crate::parser::scanner::{
    scan_error_type::ScanErrorType,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanError {
    pub error_type: ScanErrorType,
    pub row: usize,
    pub col: usize,
}

impl ScanError {
    pub fn new(error_type: ScanErrorType, row: usize, col: usize) -> Self {
        Self {
            error_type,
            row,
            col,
        }
    }
}