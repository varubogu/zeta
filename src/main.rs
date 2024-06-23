mod cli_interface;
mod commands;
mod utils;
mod settings;

use clap::{Parser};
use cli_interface::cli::Cli;
use cli_interface::zeta_command::ZetaCommand;
use commands::init_command::init;
use commands::new_command::new;
use commands::build_command::build;
use commands::remove_command::remove;
use commands::rename_command::rename;


fn main() {
    let cli = Cli::parse();
    match cli.command {
        ZetaCommand::Init => init(),
        ZetaCommand::New { target, only } => new(&target, &only),
        ZetaCommand::Build { target } => build(&target),
        ZetaCommand::Rename { target, new_name } => rename(&target, &new_name),
        ZetaCommand::Remove { target } => remove(&target),
    }
}
