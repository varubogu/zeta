use crate::ast::markdown_doc::MarkdownDoc;
use crate::ast::element::Element;
use crate::parser::platforms::zeta::frontmatter::ZetaFrontmatter;

pub type ParsedMarkdown = MarkdownDoc<ZetaFrontmatter, Element>;