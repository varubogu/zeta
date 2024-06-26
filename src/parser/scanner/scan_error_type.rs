use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScanErrorType {
    Incomplete(String),
    InvalidMacro,
}

impl Display for ScanErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanErrorType::Incomplete(string) => write!(f, "Incomplete '{}'.", string),
            ScanErrorType::InvalidMacro => write!(f, "Invalid macro."),
        }
    }
}
impl Error for ScanErrorType {}
