use clap::Subcommand;
use crate::utils::r#macro::Platform;

#[derive(Debug, Clone, Subcommand)]
pub enum ZetaCommand {
    /// Initialize Zeta
    Init,
    /// Create new article
    New {
        target: String,
        #[arg(long)]
        only: Option<Platform>,
    },
    /// Build article
    Build { target: String },
    /// Rename article
    Rename { target: String, new_name: String },
    /// Remove article
    Remove { target: String },
}
