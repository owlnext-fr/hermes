use rocket::{Rocket, Build};
use anyhow::Result;


use crate::commands::test_command::TestCommand;

use super::{database::DatabaseState, commands::{command_utils::ConsoleIO, command_registry::CommandRegistry}};

/// Build a rocket instance.
/// 
/// This function will build a rocket instance with all the required states, middlewares and routes.
pub async fn build() -> Result<Rocket<Build>> {
    // build rocket instance
    let mut build = rocket::build();

    // states
    let database = DatabaseState::connect().await?;
    let console_io = ConsoleIO::new();
    let mut command_registry = CommandRegistry::new();

    // register commands
    command_registry.register(Box::new(TestCommand));

    // manage states
    build = build.manage(database);
    build = build.manage(console_io);
    build = build.manage(command_registry);

    Ok(build)
}