use rocket::{Rocket, Build, catchers, routes};
use anyhow::Result;


use crate::{commands::{test_command::TestCommand, add_api_user_command::AddApiUserCommand, remove_api_user_command::RemoveApiUserCommand}, controllers::app};

use super::{database::DatabaseState, commands::{command_utils::ConsoleIO, command_registry::CommandRegistry}};
use crate::core::catcher;

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
    if cfg!(debug_assertions) {
        command_registry.register(Box::new(TestCommand));
    }

    command_registry.register(Box::new(AddApiUserCommand));
    command_registry.register(Box::new(RemoveApiUserCommand));

    // routes
    build = build
            .mount("/", routes![
                app::status,
                app::status_json,
            ]);
            
    // catchers
    build = build
            .register("/", catchers![catcher::default_catcher]);

    // manage states
    build = build.manage(database);
    build = build.manage(console_io);
    build = build.manage(command_registry);

    Ok(build)
}