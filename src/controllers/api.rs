use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, State};

use crate::http_exception;
use crate::model::user::UserDetailsFetchedDTO;
use crate::{
    core::{
        database::{Connected, DatabaseState},
        guards::api_user_guard::AuthenticatedApiUser,
        response::ApiResult,
        validation::Validated,
    },
    http_ok,
    middlewares::user_middleware::UserMiddleware,
    model::user::{NewUserDTO, UserDetailsDTO},
};

#[post("/api/users", format = "json", data = "<new_user>")]
pub async fn create_user(
    auth_api_user: AuthenticatedApiUser,
    new_user: Validated<Json<NewUserDTO>>,
    db_state: &State<DatabaseState<Connected>>,
) -> ApiResult<UserDetailsFetchedDTO> {
    let middleware = UserMiddleware::new(db_state.get_new_connection());
    let current_api_user = auth_api_user.api_user;
    let new_user = new_user.into_inner().into_inner();

    let user = middleware.create(&new_user, &current_api_user).await;

    if let Err(error) = &user {
        http_exception!(Status::InternalServerError, &error.to_string());
    }

    let user = user.unwrap();

    let output = middleware.to_details_fetched(&user).await;

    if let Err(error) = &output {
        http_exception!(Status::InternalServerError, &error.to_string());
    }

    let output = output.unwrap();

    http_ok!(output)
}
