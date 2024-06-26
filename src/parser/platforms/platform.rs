#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, clap::ValueEnum)]
pub enum PlatformType {
    #[serde(alias = "zenn")]
    Zenn,
    #[serde(alias = "qiita")]
    Qiita,
    #[serde(alias = "Zeta")]
    Zeta,
}