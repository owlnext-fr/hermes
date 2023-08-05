use anyhow::Result;
use rocket::{Build, Rocket};

use crate::{
    core::{
        cipher::Cipher,
        commands::{
            command_trait::{CommandArgs, CommandTrait},
            command_utils::ConsoleIO,
        },
        database::{Connected, DatabaseState},
    },
    middlewares::api_user_middleware::ApiUserMiddleware,
    model::api_user::NewApiUserDTO,
};

#[derive(Clone, Default)]
/// A simple test command.
pub struct AddApiUserCommand;

#[async_trait::async_trait]
impl<'a> CommandTrait<'a> for AddApiUserCommand {
    fn name(&self) -> &'a str {
        "add_api_user"
    }

    fn description(&self) -> &'a str {
        "Adds an API user to the database"
    }

    fn is_parallel(&self) -> bool {
        false
    }

    async fn do_run(
        &self,
        rocket: &Rocket<Build>,
        io: &ConsoleIO,
        args: &CommandArgs,
    ) -> Result<()> {
        let db_conn = rocket.state::<DatabaseState<Connected>>().unwrap();
        let api_user_middleware = ApiUserMiddleware::new(db_conn.get_new_connection());

        let name: String;
        let key: String;

        let name_arg = args.get("name");
        let key_arg = args.get("key");

        if name_arg.is_none() {
            name = io.ask_question("Enter a name for the user");
        } else {
            name = name_arg.unwrap().as_ref().unwrap().to_string();
        }

        if key_arg.is_none() {
            key = io.ask_question("Enter a key for the user");
        } else {
            key = key_arg.unwrap().as_ref().unwrap().to_string();
        }

        if !Cipher::validate_password_complexity(&key) {
            return Err(anyhow::anyhow!("The key must be at least 10 characters long and contain at least one number, one uppercase letter, one lowercase letter and one special character"));
        }

        io.writeln(&format!("Name: {}", &name));
        io.writeln(&format!("Key: {}", &key));

        let user = api_user_middleware
            .create(NewApiUserDTO {
                name: name.clone(),
                api_key: key.clone(),
            })
            .await?;

        io.success(&format!(
            "Created user: {} ({})",
            &user.name,
            &user.id.unwrap()
        ));

        Ok(())
    }
}
