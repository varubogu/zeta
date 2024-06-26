
use crate::ast::message_type::MessageType;
use crate::r#macro::ParsedMacro;

#[derive(Debug, Clone)]
pub enum Element {
    Text(String),
    Url(String),
    Macro(ParsedMacro),
    LinkCard {
        card_type: String,
        url: String,
    },
    Image {
        alt: String,
        url: String,
    },
    InlineFootnote(String),
    Footnote(String),
    Message {
        level: usize,
        msg_type: MessageType,
        body: Vec<Element>,
    },
    Details {
        level: usize,
        title: String,
        body: Vec<Element>,
    },
}
