use std::fmt::Display;
use crate::parser::constants::FRONTMATTER_TOPICS_MAX;
use crate::token::token_type::TokenType;



#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorType {
    TooManyTopics(Vec<String>),
    InvalidFrontMatter,
    InvalidMacro,
    InvalidMessageType,
    InvalidNestingLevel(usize),
    CouldNotFindEndToken(TokenType),
}

impl Display for ParseErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorType::TooManyTopics(topics) => write!(
                f,
                "Too many topics: [{}]. The maximum number of topics is {}.",
                topics.join(", "),
                FRONTMATTER_TOPICS_MAX
            ),
            ParseErrorType::InvalidFrontMatter => write!(f, "Invalid front matter"),
            ParseErrorType::InvalidMacro => write!(f, "Invalid macro"),
            ParseErrorType::InvalidMessageType => write!(f, "Invalid message type"),
            ParseErrorType::InvalidNestingLevel(level) => write!(
                f,
                "Invalid nesting level: {}. The nesting level must be smaller than the outer one.",
                level
            ),
            ParseErrorType::CouldNotFindEndToken(token_type) => write!(
                f,
                "Could not find end token: {:?}.",
                token_type
            ),
        }
    }
}