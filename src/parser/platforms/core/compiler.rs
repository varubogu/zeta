use serde::Serialize;
use crate::ast::element::Element;
use crate::ast::markdown_doc::MarkdownDoc;

pub(crate) trait BaseCompiler<TFrontMatter>
where TFrontMatter: Clone + Serialize
{
    ///
    fn compile(mut self, file: MarkdownDoc<TFrontMatter, Element>) -> String  where Self: Sized {
        self.compile_frontmatter(file.frontmatter) + &self.compile_elements(file.elements)
    }

    fn compile_frontmatter(&mut self, frontmatter: TFrontMatter) -> String where Self: Sized;

    fn compile_elements(&mut self, elements: Vec<Element>) -> String  where Self: Sized {
        elements
            .into_iter()
            .map(|element| self.compile_element(element))
            .collect()
    }

    fn compile_element(&mut self, element: Element) -> String  where Self: Sized;
}
