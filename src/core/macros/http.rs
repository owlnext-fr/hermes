#[macro_export]
macro_rules! http_exception {
    ($status:expr) => {{
        use crate::core::response::ApiResponse;
        return Err(ApiResponse::from_status($status));
    }};
    ($status:expr, $reason:expr) => {{
        use crate::core::response::ApiResponse;
        return Err(ApiResponse::from_status_with_reason($status, $reason));
    }};
}

#[macro_export]
macro_rules! http_ok {
    ($output:expr) => {{
        use crate::core::response::ApiResponse;
        use rocket::serde::json::Json;
        return Ok(ApiResponse::ok(Json($output)));
    }};
}

#[macro_export]
macro_rules! http_no_content {
    () => {{
        use crate::core::response::ApiResponse;
        return Ok(ApiResponse::no_content());
    }};
}
