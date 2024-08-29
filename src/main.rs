use clap::Parser;

use git_conform::{Cli, Commands, handle_error, scan};

fn main() {
    let cli = Cli::parse();

    match cli.get_command() {
        Commands::Scan { dirs, all } => {
            if let Err(e) = scan(dirs, all) {
                handle_error(&e, 1);
            }
        }
    };
}
