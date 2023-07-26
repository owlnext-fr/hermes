use rocket::get;

use crate::{core::response::{ApiResponse, ApiResult}, model::utils::app_status::AppStatus, http_ok};
use rocket::response::content::RawHtml;


#[get("/status", format = "json", rank = 1)]
pub fn status_json() -> ApiResult<AppStatus> {
    http_ok!(AppStatus::new());
}

#[get("/status", rank = 2)]
pub fn status() -> RawHtml<String> {
    let status = AppStatus::new();

    let page = RawHtml(format!(r#"
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
    "#, status.name, status.description, status.version));

    page
}
