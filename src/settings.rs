
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct Settings {
    pub(crate) repository: String,
}
