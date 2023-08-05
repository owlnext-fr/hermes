use anyhow::Result;
use rocket::{Build, Rocket};

use crate::core::commands::{
    command_trait::{CommandArgs, CommandTrait},
    command_utils::ConsoleIO,
};

#[derive(Clone, Default)]
/// A simple test command.
pub struct TestCommand;

#[async_trait::async_trait]
impl<'a> CommandTrait<'a> for TestCommand {
    fn name(&self) -> &'a str {
        "test"
    }

    fn description(&self) -> &'a str {
        "A simple test command."
    }

    fn is_parallel(&self) -> bool {
        false
    }

    async fn do_run(
        &self,
        _rocket: &Rocket<Build>,
        io: &ConsoleIO,
        args: &CommandArgs,
    ) -> Result<()> {
        io.info("Hello from test command!");
        io.info(&format!("Args: {:?}", args));

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        if args.get("error").is_some() {
            return Err(anyhow::anyhow!("Oops !"));
        }

        io.success("Test command successfully terminated !");

        Ok(())
    }
}
