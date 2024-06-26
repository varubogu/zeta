use crate::token::token_type::TokenType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub row: usize,
    pub col: usize,
}