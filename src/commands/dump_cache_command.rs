use anyhow::Result;
use redis::AsyncCommands;
use rocket::{Build, Rocket};

use crate::core::{
    cache::{CacheState, Connected},
    commands::{
        command_trait::{CommandArgs, CommandTrait},
        command_utils::ConsoleIO,
    },
};

#[derive(Clone, Default)]
pub struct DumpCacheCommand;

#[async_trait::async_trait]
impl<'a> CommandTrait<'a> for DumpCacheCommand {
    fn name(&self) -> &'a str {
        "dump:cache"
    }

    fn description(&self) -> &'a str {
        "Dump the cache."
    }

    fn is_parallel(&self) -> bool {
        false
    }

    async fn do_run(
        &self,
        rocket: &Rocket<Build>,
        io: &ConsoleIO,
        _args: &CommandArgs,
    ) -> Result<()> {
        let cache = rocket.state::<CacheState<Connected>>().unwrap();
        let mut conn = cache.get_new_connection().get_async_connection().await?;

        io.info("Dumping the cache...");

        let keys: Vec<String> = conn.keys("*").await?;

        for key in keys.iter() {
            let value: String = conn.get(key).await?;

            io.writeln(&format!("{}: {}", key, value));
        }

        Ok(())
    }
}
