use serde::Serialize;
use crate::{
    ast::{element::Element, message_type::MessageType, parsed_markdown::ParsedMarkdown},
    parser::platforms::{
        zenn::frontmatter::ZennFrontmatter,
        zeta::frontmatter::ZetaFrontmatter,
    }
};

pub struct ZennCompiler;

impl ZennCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(mut self, file: ParsedMarkdown) -> String {
        self.compile_frontmatter(file.frontmatter) + &self.compile_elements(file.elements)
    }

    fn compile_frontmatter(&mut self, frontmatter: ZetaFrontmatter) -> String {
        let mut result = b"---\n".to_vec();
        let frontmatter = ZennFrontmatter {
            title: frontmatter.title,
            emoji: frontmatter.emoji,
            r#type: frontmatter.r#type,
            topics: frontmatter.topics,
            published: frontmatter.published,
        };
        let mut ser = serde_yaml::Serializer::new(&mut result);
        frontmatter.serialize(&mut ser).unwrap();
        result.extend(b"---\n");
        String::from_utf8(result).unwrap()
    }

    fn compile_elements(&mut self, elements: Vec<Element>) -> String {
        elements
            .into_iter()
            .map(|element| self.compile_element(element))
            .collect()
    }

    fn compile_element(&mut self, element: Element) -> String {
        match element {
            Element::Text(text) => text,
            Element::Url(url) => format!("{}", url),
            Element::Macro(macro_info) => self.compile_elements(macro_info.zenn),
            Element::LinkCard { card_type, url } => {
                format!("@[{}]({})", card_type, url)
            }
            Element::Image { alt, url } => {
                format!("![{}]({})", alt, url)
            }
            Element::InlineFootnote(content) => format!("^[{}]", content),
            Element::Footnote(name) => format!("[^{}]", name),
            Element::Message {
                level,
                msg_type,
                body,
            } => {
                let msg_type = match msg_type {
                    MessageType::Info => "",
                    MessageType::Warn => "",
                    MessageType::Alert => "alert",
                };

                let mut compiler = ZennCompiler {};
                let body = compiler.compile_elements(body);

                format!(
                    ":::{0}message {1}{2}:::{0}",
                    ":".repeat(level),
                    msg_type,
                    body
                )
            }
            Element::Details { level, title, body } => {
                let mut compiler = ZennCompiler {};
                let body = compiler.compile_elements(body);
                format!(
                    ":::{0}details {1}{2}:::{0}",
                    ":".repeat(level),
                    title,
                    body
                )
            }
        }
    }
}
