use crate::parser::parse_error::ParseError;

pub(crate) type ParseResult<T> = std::result::Result<T, ParseError>;
