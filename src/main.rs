mod utils;
mod console;
mod commands;
mod settings;
mod token;
mod r#macro;
mod parser;
mod ast;

use clap::{Parser};
use console::cli::Cli;
use console::zeta_command::ZetaCommand;
use commands::new_command::new;
use commands::init_command::init;
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