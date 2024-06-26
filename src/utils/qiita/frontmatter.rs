use serde_derive::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct QiitaFrontmatter {
    pub(crate) title: String,
    pub(crate) tags: Vec<String>,
    pub(crate) private: bool,
    pub(crate) updated_at: String,
    pub(crate) id: Option<String>,
    pub(crate) organization_url_name: Option<String>,
    pub(crate) slide: bool,
    pub(crate) ignorePublish: bool,
}
