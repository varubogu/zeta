#[derive(Debug, Clone)]
pub struct MarkdownDoc<F, E> {
    pub frontmatter: F,
    pub elements: Vec<E>,
}

impl<F, E> MarkdownDoc<F, E> {
    pub fn new(frontmatter: F, elements: Vec<E>) -> Self {
        Self {
            frontmatter,
            elements,
        }
    }
}