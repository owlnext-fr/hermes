use anyhow::{bail, Result};
use chrono::Utc;
use surrealdb::{engine::remote::ws::Client, Surreal};
use thiserror::Error;

use crate::{
    core::commands::command_trait::CommandResult,
    model::command_log::{CommandLog, CommandStatus, COMMAND_LOG_TABLE},
};

#[derive(Debug, Error)]
pub enum CommandMiddlewareError {
    #[error("command {0} ({1}) is already running.")]
    AlreadyRunning(String, String),
    #[error("database error: {0}")]
    DatabaseError(String),
}

#[derive(Clone)]
/// A middleware that contains all Command related logic.
pub struct CommandMiddleware {
    /// The database client.
    pub db: Surreal<Client>,
}

impl CommandMiddleware {
    /// Create a new CommandMiddleware.
    pub fn new(db: Surreal<Client>) -> Self {
        Self { db }
    }

    /// Check if a command is already running in database.
    pub async fn try_aquire_lock(&self, command_name: &str, command_args: &str) -> Result<()> {
        let result = self.db
        .query("SELECT * FROM type::table($table_name) WHERE command_name = $command_name AND command_args = $command_args and status = 'RUNNING'")
        .bind(("table_name", COMMAND_LOG_TABLE))
        .bind(("command_name", command_name))
        .bind(("command_args", command_args))
        .await;

        if let Err(error) = &result {
            bail!(CommandMiddlewareError::DatabaseError(error.to_string()));
        }

        let exists: Vec<CommandLog> = result?.take(0)?;

        if exists.len() > 0 {
            bail!(CommandMiddlewareError::AlreadyRunning(
                command_name.to_string(),
                command_args.to_string()
            ));
        }

        Ok(())
    }

    /// Create a new command log in database.
    pub async fn create_log(&self, command_name: &str, command_args: &str) -> Result<CommandLog> {
        let created = self
            .db
            .create(COMMAND_LOG_TABLE)
            .content(CommandLog {
                id: None,
                command_name: command_name.to_string(),
                command_args: command_args.to_string(),
                status: CommandStatus::RUNNING,
                message: None,
                created_at: Utc::now(),
                closed_at: None,
                elapsed: None,
            })
            .await;

        if let Err(error) = &created {
            bail!(CommandMiddlewareError::DatabaseError(error.to_string()));
        }

        Ok(created?)
    }

    /// Update a command log in database.
    pub async fn update_log(
        &self,
        command_log: &CommandLog,
        command_result: CommandResult,
        message: Option<String>,
    ) -> Result<CommandLog> {
        let mut log = command_log.clone();

        log.status = match command_result {
            CommandResult::SUCCESS => CommandStatus::SUCCESS,
            CommandResult::ERROR => CommandStatus::ERROR,
            CommandResult::SKIPPED => CommandStatus::SKIPPED,
        };

        log.message = message;
        log.closed_at = Some(Utc::now());
        log.elapsed = Some(
            log.closed_at
                .unwrap()
                .signed_duration_since(log.created_at)
                .num_milliseconds(),
        );

        let log_id = log.id.clone().unwrap();

        let updated = self
            .db
            .update((COMMAND_LOG_TABLE, log_id.id))
            .content(log)
            .await;

        if let Err(error) = &updated {
            bail!(CommandMiddlewareError::DatabaseError(error.to_string()));
        }

        Ok(updated?)
    }
}
