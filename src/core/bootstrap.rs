use std::collections::HashMap;

use anyhow::{bail, Result};
use rocket::{Build, Rocket};
use thiserror::Error;

use crate::core::commands::command_trait::CommandError;

use super::commands::command_registry::CommandRegistry;

#[derive(Debug, Error)]
/// Pre-runtime errors.
pub enum PreRuntimeErrors {
    #[error("failed to ignite rocket instance: {0}")]
    FailedToIgniteRocketInstance(String),
    #[error("failed to launch rocket instance: {0}")]
    FailedToLaunchRocketInstance(String),
    #[error("command registry not found in rocket state.")]
    FailedToGetCommandRegistry,
    #[error("command {0} not found.")]
    FailedToGetCommand(String),
    #[error("command {0} failed: {1}")]
    FailedToRunCommand(String, String),
    #[error("command skipped: {0}")]
    CommandSkipped(String),
}

/// Launches the HTTP server.
pub async fn launch_server(rocket: Rocket<Build>) -> Result<i32> {
    let possible_ignited = rocket.ignite().await;

    if let Err(error) = &possible_ignited {
        dbg!(&error);
        bail!(PreRuntimeErrors::FailedToIgniteRocketInstance(
            error.to_string()
        ));
    }

    let possible_launched = possible_ignited?.launch().await;

    if let Err(error) = &possible_launched {
        bail!(PreRuntimeErrors::FailedToLaunchRocketInstance(
            error.to_string()
        ));
    }

    Ok(0)
}

/// Launches the console interface.
pub async fn launch_console(
    rocket: Rocket<Build>,
    command: String,
    args: HashMap<String, Option<String>>,
) -> Result<i32> {
    // get command registry
    let possible_command_registry = rocket.state::<CommandRegistry>();

    // check if command registry is present
    if possible_command_registry.is_none() {
        bail!(PreRuntimeErrors::FailedToGetCommandRegistry);
    }
    let command_registry = possible_command_registry.unwrap();

    // get command
    let possible_command = command_registry.get(&command);

    // check if command is present
    if possible_command.is_none() {
        bail!(PreRuntimeErrors::FailedToGetCommand(command));
    }
    let command = possible_command.unwrap();

    // run command
    let runtime = command.run(&rocket, args).await;

    // check if command failed
    if let Err(error) = &runtime {
        let inner_error = error.root_cause().downcast_ref::<CommandError>();

        if let Some(error) = inner_error {
            if matches!(error, CommandError::AlreadyRunning(_, _)) {
                bail!(PreRuntimeErrors::CommandSkipped(error.to_string()));
            }
        }

        bail!(PreRuntimeErrors::FailedToRunCommand(
            command.name().to_string(),
            error.to_string()
        ));
    }

    Ok(0)
}
