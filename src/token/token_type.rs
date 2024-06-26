use crate::r#macro::TokenizedMacro;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    /// string
    Text(String),
    /// http:// or https://
    Url(String),
    ///LinkCard
    LinkCard {
        card_type: String,
        url: String,
    },
    /// image
    Image {
        alt: String,
        url: String,
    },
    /// inline footnote
    InlineFootnote(String),
    /// footnote
    Footnote(String),
    /// :::message
    MessageBegin {
        level: usize,
        r#type: String,
    },
    /// :::details
    DetailsBegin {
        level: usize,
        title: String,
    },
    /// :::
    MessageOrDetailsEnd {
        level: usize,
    },
    Macro(TokenizedMacro),
}