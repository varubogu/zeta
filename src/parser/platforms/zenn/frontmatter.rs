use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ZennFrontmatter {
    pub(crate) title: String,
    pub(crate) emoji: String,
    pub(crate) r#type: String,
    pub(crate) topics: Vec<String>,
    pub(crate) published: bool,
}
