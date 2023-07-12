use crate::core::{database::DatabaseState, bootstrap, cli};

use clap::Parser;
use anyhow::Result;

pub mod core;

/// main entrypoint of the program.
#[tokio::main]
async fn main() -> Result<()> {
    // Load the .env file if any into the environment variables.
    dotenvy::dotenv()?;

    // Parse the command line arguments.
    let cli = cli::Cli::parse();

    // Initialize the database connection state for future use.
    // FIXME: This variable starts with an underscore because it is not used for now.
    let _db_state = DatabaseState::connect().await?;

    // Launch the server or the console depending on the command line arguments.
    match cli.subcommand {
        // Launch the server.
        cli::Command::Server => {
            bootstrap::launch_server();
        },
        // Launch the console.
        cli::Command::Console { console_command, args } => {
            bootstrap::launch_console(console_command, args);
        }
    }
    
    // Return an empty Ok() result.
    Ok(())
}
