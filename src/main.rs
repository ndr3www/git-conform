mod core;
mod utils;
mod cli;

use crate::core::scan;
use crate::utils::handle_error;
use crate::cli::{Cli, Commands};

use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.get_command() {
        Commands::Scan { dirs, all } => {
            if let Err(e) = scan(dirs, *all) {
                handle_error(&e, 1);
            }
        }
    };
}
