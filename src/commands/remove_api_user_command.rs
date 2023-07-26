use anyhow::Result;
use rocket::{Build, Rocket};

use crate::{core::{commands::{command_trait::{CommandArgs, CommandTrait}, command_utils::ConsoleIO}, database::{DatabaseState, Connected}}, middlewares::api_user_middleware::ApiUserMiddleware};

#[derive(Clone, Default)]
/// A simple test command.
pub struct RemoveApiUserCommand;

#[async_trait::async_trait]
impl<'a> CommandTrait<'a> for RemoveApiUserCommand {
    fn name(&self) -> &'a str {
        "remove_api_user"
    }

    fn description(&self) -> &'a str {
        "Removes an API user to the database"
    }

    fn is_parallel(&self) -> bool {
        false
    }

    async fn do_run(&self, rocket: &Rocket<Build>, io: &ConsoleIO, _args: &CommandArgs) -> Result<()> {
        let db_conn = rocket.state::<DatabaseState<Connected>>().unwrap();
        let api_user_middleware = ApiUserMiddleware::new(db_conn.get_new_connection());
        
        let active_users = api_user_middleware.list_active_api_users().await?;

        if active_users.len() == 0 {
            io.info("No active users found");
            return Ok(());
        }

        let mut displayable = Vec::<Vec<String>>::new();

        for (index, user) in active_users.iter().enumerate() {
            displayable.push(vec![
                index.to_string(),
                user.name.clone()
            ]);
        }

        io.table(
            vec![
                "Index".to_string(),
                "Name".to_string()
            ],
            displayable.clone()
        );
        let max = displayable.len() - 1;

        let mut idx: usize = 0;
        let mut response_ok = false;

        while !response_ok {
            let index = io.ask_question(&format!("Which user do you want to remove ? (0-{})", max));
            idx = index.parse::<usize>()?;

            if idx <= max {
                response_ok = true;
            } else {
                io.error(&format!("Index must be between 0 and {}", max));
            }
        }

        let user = active_users.get(idx).unwrap().clone();

        let confirm = io.ask_confirm(&format!("Are you sure you want to remove {} ?", user.name));

        if confirm {
            api_user_middleware.delete(&mut user.clone()).await?;
            io.success(&format!("User {} removed", user.name));
        } else {
            io.info("User not removed");
        }

        Ok(())
    }
}