use std::{collections::HashMap, str::FromStr};

use anyhow::{Result, bail};
use rocket::{Rocket, Build};
use thiserror::Error;

use crate::{model::command_log::CommandLog, middlewares::command_middleware::{CommandMiddleware, CommandMiddlewareError}, core::database::{DatabaseState, Connected}};

use super::command_utils::ConsoleIO;

/// shorthand type for command arguments structure.
pub type CommandArgs = HashMap<String, Option<String>>;

/// Final status of a command.
pub enum CommandResult {
    /// the command successfully terminated.
    SUCCESS,
    /// the command terminated with an error.
    ERROR,
    /// the command was skipped due to external requirements, probably a lock race-condition.
    SKIPPED,
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("command {0} ({1}) is already running.")]
    AlreadyRunning(String, String),
    #[error("database error: {0}")]
    DatabaseError(String),
}

#[async_trait::async_trait]
/// A trait that represents a console command.
pub trait CommandTrait<'a>: Send + Sync {
    /// The name of the command.
    fn name(&self) -> &'a str;

    /// The description of the command.
    fn description(&self) -> &'a str;

    /// Either the command can be launched in parallel or not using same parameters.
    fn is_parallel(&self) -> bool;

    /// The command entrypoint.
    /// 
    /// This will contain all the command logic (LLOC).
    async fn do_run(&self, rocket: &Rocket<Build>, io: &ConsoleIO, args: &CommandArgs) -> Result<()>;

    /// Starting phase of the command.
    /// 
    /// This will be called before the command execution.
    /// 
    /// This will be used to aquire a lock on the command, if the command is not parallel, then declare the command log.
    async fn begin(&self, middleware: &CommandMiddleware, args: &CommandArgs) -> Result<CommandLog> {
        // get args as string
        let args_as_str = self.get_args_as_str(args);

        // if the command is not parallelizable, then try to aquire a lock.
        if !self.is_parallel() {
            let running = middleware.try_aquire_lock(&self.name(), &args_as_str).await;

            if running.is_err() {
                let inner = running.err().unwrap();
                let inner_error = inner.root_cause().downcast_ref::<CommandMiddlewareError>().unwrap();

                if matches!(inner_error, CommandMiddlewareError::AlreadyRunning(_, _)) {
                    let log = middleware.create_log(&self.name(), &args_as_str).await?;
                    self.end(middleware, log, CommandResult::SKIPPED, Some(inner_error.to_string())).await?;
                    bail!(CommandError::AlreadyRunning(self.name().into(), args_as_str));
                }

                if matches!(inner_error, CommandMiddlewareError::DatabaseError(_)) {
                    bail!(CommandError::DatabaseError(inner_error.to_string()));
                }
            }
        }

        // create the command log.
        let log = middleware.create_log(&self.name(), &args_as_str).await;

        if let Err(error) = &log {
            let inner = error.root_cause().downcast_ref::<CommandMiddlewareError>().unwrap();

            if matches!(inner, CommandMiddlewareError::DatabaseError(_)) {
                bail!(CommandError::DatabaseError(inner.to_string()));
            }
        }

        Ok(log?)
    }
    
    /// Ending phase of the command.
    /// 
    /// This will be called after the command execution.
    /// 
    /// This will be used to update the command log with the final status, error message and elapsed time.
    async fn end(&self, middleware: &CommandMiddleware, command_log: CommandLog, command_result: CommandResult, message: Option<String>) -> Result<CommandLog> {
        // update the command log.
        let log = middleware.update_log(&command_log, command_result, message).await;

        if let Err(error) = &log {
            let inner = error.root_cause().downcast_ref::<CommandMiddlewareError>().unwrap();

            if matches!(inner, CommandMiddlewareError::DatabaseError(_)) {
                bail!(CommandError::DatabaseError(inner.to_string()));
            }
        }

        Ok(log?)
    }

    async fn run(&self, rocket: &Rocket<Build>, args: CommandArgs) -> Result<()> {
        // getting requirements (console IO and database connection pool)
        let io = rocket.state::<ConsoleIO>().unwrap();
        let db_conn = rocket.state::<DatabaseState<Connected>>().unwrap();

        // creating the command middleware
        let command_log_middleware = CommandMiddleware::new(db_conn.get_new_connection());

        // display the command title
        io.title(&self.name());
        io.new_line();

        // start phase
        let mut log = self.begin(&command_log_middleware, &args).await?;

        // executes the command logic
        let exec_result = self.do_run(rocket, &io, &args).await;

        // if the command exited with an error, then update the command log with the error message.
        if let Err(error) = &exec_result {
            log = self.end(&command_log_middleware, log, CommandResult::ERROR, Some(error.to_string())).await?;
            io.error(&error.to_string());
        } else {
            // if the command exited with success, then update the command log with the success status.
            log = self.end(&command_log_middleware, log, CommandResult::SUCCESS, None).await?;
        }

        // display the command status and elapsed time.
        io.new_line();
        let elapsed = log.elapsed.unwrap() as f64 / 1000.0;
        io.writeln(&format!("-- Status: {:?}, Elapsed: {:.3} secs --", &log.status, elapsed));

        // if the command exited with an error, then return the error.
        if exec_result.is_err() {
            return Err(exec_result.err().unwrap());
        }

        // return success.
        Ok(())
    }

    /// transforms a CommandArgs payload into a string, for lock purposes.
    fn get_args_as_str(&self, args: &CommandArgs) -> String {
        serde_json::to_string(&args).unwrap_or(String::from_str("{}").unwrap())
    }
}