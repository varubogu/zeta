use serde::Serialize;
use crate::parser::platforms::zeta::frontmatter::ZetaFrontmatter;
use crate::ast::element::Element;
use crate::ast::message_type::MessageType;
use crate::ast::parsed_markdown::ParsedMarkdown;

pub struct ZetaCompiler;

impl ZetaCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(mut self, file: ParsedMarkdown) -> String {
        self.compile_frontmatter(file.frontmatter) + &self.compile_elements(file.elements)
    }

    fn compile_frontmatter(&mut self, frontmatter: ZetaFrontmatter) -> String {
        let mut result = b"---\n".to_vec();
        let frontmatter = ZetaFrontmatter {
            title: frontmatter.title,
            emoji: frontmatter.emoji,
            r#type: frontmatter.r#type,
            topics: frontmatter.topics,
            published: frontmatter.published,
            only: frontmatter.only,
            qiita_id: frontmatter.qiita_id,
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

                let mut compiler = ZetaCompiler {};
                let body = compiler.compile_elements(body);

                format!(
                    ":::{0}message {1}{2}:::{0}",
                    ":".repeat(level),
                    msg_type,
                    body
                )
            }
            Element::Details { level, title, body } => {
                let mut compiler = ZetaCompiler {};
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
