use crate::parser::scanner::scan_error::ScanError;

pub(crate) type ScannerResult<T> = Result<T, ScanError>;