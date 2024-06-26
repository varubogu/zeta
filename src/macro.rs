use crate::{
    ast::element::Element,
    token::token::Token
};

pub type StringMacro = Macro<Option<String>>;
pub type TokenizedMacro = Macro<Vec<Token>>;
pub type ParsedMacro = Macro<Vec<Element>>;

#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq)]
pub struct Macro<T> {
    pub zenn: T,
    pub qiita: T,
}
