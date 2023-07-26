#[macro_export]
macro_rules! http_exception {
    ($status:expr) => {{
        return Err(ApiResponse::from_status($status));
    }};
    ($status:expr, $reason:expr) => {{
        return Err(ApiResponse::from_status_with_reason($status, $reason));
    }};
}

#[macro_export]
macro_rules! http_ok {
    ($output:expr) => {{
        use rocket::serde::json::Json;
        return Ok(ApiResponse::ok(Json($output)));
    }};
}

#[macro_export]
macro_rules! http_no_content {
    () => {{
        return Ok(ApiResponse::no_content());
    }};
}
