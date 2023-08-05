use rocket::get;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        guards::api_user_guard::AuthenticatedApiUser,
        response::{ApiResponse, ApiResult},
    },
    http_ok,
    model::utils::app_status::AppStatus,
};
use rocket::response::content::RawHtml;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TestAuthResponse {
    pub message: String,
}

#[get("/test/api-user-auth", format = "json")]
pub fn test_api_user_auth(auth_api_user: AuthenticatedApiUser) -> ApiResult<TestAuthResponse> {
    let _api_user = auth_api_user.api_user;

    http_ok!(TestAuthResponse {
        message: format!("Hello, {}!", _api_user.name),
    });
}

#[get("/status", format = "json", rank = 1)]
pub fn status_json() -> ApiResult<AppStatus> {
    http_ok!(AppStatus::new());
}

#[get("/status", rank = 2)]
pub fn status() -> RawHtml<String> {
    let status = AppStatus::new();

    let page = RawHtml(format!(
        r#"
    <!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Bootstrap demo</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.1/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-4bw+/aepP/YC94hEpVNVgiZdgIC5+VKNBQNGCHeKRQN+PtmoHDEXuppvnDJzQIu9" crossorigin="anonymous">
  </head>
  <body>
    <div class="container p-5">
      <h1>App Status</h1>
      <hr>
      <p>name: {}</p>
      <p>description: {}</p>
      <p>version: {}</p>
    </div>
    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.1/dist/js/bootstrap.bundle.min.js" integrity="sha384-HwwvtgBNo3bZJJLYd8oVXjrBZt8cqVSpeBNS5n7C8IVInixGAoxmnlMuBnhbgrkm" crossorigin="anonymous"></script>
  </body>
</html>
    "#,
        status.name, status.description, status.version
    ));

    page
}
