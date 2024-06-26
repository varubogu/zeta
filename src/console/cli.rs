use crate::console::zeta_command::ZetaCommand;

#[derive(Debug, Clone, clap::Parser)]
#[command(version, about)]
pub(crate) struct Cli {
    /// Subcommand
    #[command(subcommand)]
    pub(crate) command: ZetaCommand,
}
