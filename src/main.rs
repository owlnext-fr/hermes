use crate::core::{rocket_factory, exit_codes, commands::command_utils::ConsoleIO, bootstrap, cli, bootstrap::PreRuntimeErrors};

use clap::Parser;
use anyhow::Result;

pub mod core;
pub mod model;
pub mod middlewares;
pub mod commands;

/// main entrypoint of the program.
#[tokio::main]
async fn main() -> Result<()> {

    // Create a new ConsoleIO instance.
    let io = ConsoleIO::new();

    // Load the .env file if any into the environment variables.
    if let Err(error) = dotenvy::dotenv() {
        io.error(&format!("Failed to load the .env file: {}", error));
        exit!(exit_codes::ERR_ENV_NOT_LOADED)
    }

    // Parse the command line arguments.
    let cli = cli::Cli::parse();

    // build the rocket instance.
    let possible_rocket = rocket_factory::build().await;

    // If the rocket instance could not be built, exit the program.
    if let Err(error) = &possible_rocket {
        io.error(&format!("Failed to build the rocket instance: {}", error));
        exit!(exit_codes::ERR_ROCKET_NOT_BUILT)
    }

    // Unwrap the rocket instance.
    let rocket = possible_rocket?;

    // Launch the server or the console depending on the command line arguments.
    let exit_status = match cli.subcommand {
        // Launch the server.
        cli::Command::Server => {
            bootstrap::launch_server(rocket).await
        },
        // Launch the console.
        cli::Command::Console { console_command, args } => {
            bootstrap::launch_console(rocket, console_command, args).await
        }
    };

    // If the exit status is an error
    if let Err(error) = &exit_status {

        // Downcast the error to a PreRuntimeErrors enum value if corresponding.
        let downcasted_error = error.root_cause().downcast_ref::<PreRuntimeErrors>();

        // If the error is a PreRuntimeErrors enum value, exit the program with the corresponding exit code.
        if let Some(downcasted_error) = downcasted_error {
            let exit_code = match downcasted_error {
                PreRuntimeErrors::FailedToIgniteRocketInstance(_)=>exit_codes::ERR_ROCKET_IGNITION_FAILED,
                PreRuntimeErrors::FailedToLaunchRocketInstance(_)=>exit_codes::ERR_ROCKET_LAUNCH_FAILED,
                PreRuntimeErrors::FailedToGetCommandRegistry=>exit_codes::ERR_COMMAND_REGISTRY_NOT_FOUND,
                PreRuntimeErrors::FailedToGetCommand(_)=>exit_codes::ERR_COMMAND_NOT_FOUND,
                PreRuntimeErrors::FailedToRunCommand(_,_)=>exit_codes::ERR_COMMAND_FAILED,
                PreRuntimeErrors::CommandSkipped(_) => exit_codes::ERR_COMMAND_SKIPPED, 
            };

            // If the error is not a PreRuntimeErrors::FailedToRunCommand enum value, print the error message.
            if !matches!(downcasted_error, PreRuntimeErrors::FailedToRunCommand(_, _)) {
                io.error(&format!("{}", error));
            }

            exit!(exit_code)
        } else {
            // If the error is not a PreRuntimeErrors enum value, exit the program with the unknown error exit code.
            io.error(&format!("Unknown error: {}", error));
            exit!(exit_codes::ERR_UNKNOWN_RUNTIME_ERROR)
        }
    }

    // Return an empty Ok() result.
    Ok(())
}
