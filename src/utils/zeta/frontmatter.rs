use crate::utils::r#macro::Platform;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ZetaFrontmatter {
    pub title: String,
    pub emoji: String,
    pub r#type: String,
    pub topics: Vec<String>,
    pub qiita_id: Option<String>,
    pub published: bool,
    /// compile only specified platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only: Option<Platform>,
}