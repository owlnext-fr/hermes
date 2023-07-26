use rocket::http::Status;
use serde::Serialize;

/// a struct representing an HTTP exception
#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct HttpException {
    /// code of the exception, or HTTP status.
    pub code: u16,
    /// message string for this error.
    pub message: String,
    /// errors listed for debugging purposes.
    pub errors: Option<String>,
}

impl HttpException {
    /// creates an HttpException from numeric code.
    pub fn from_code(code: u16) -> Self {
        let status: Status = Status::new(code);

        HttpException {
            code: status.code,
            message: status.reason().unwrap_or("Unknown error").to_string(),
            errors: None,
        }
    }

    /// creates an HttpException from an HTTP status.
    pub fn from_status(status: Status) -> Self {
        HttpException {
            code: status.code,
            message: status.reason().unwrap_or("Unknown error").to_string(),
            errors: None,
        }
    }

    /// creates an HttpException from numeric code with a reason.
    pub fn from_code_with_reason(code: u16, reason: Option<String>) -> Self {
        let mut ex: Self = Self::from_code(code);
        ex.errors = reason;

        ex
    }

    /// creates an HttpException from an HTTP status with a reason.
    pub fn from_status_with_reason(status: Status, reason: Option<String>) -> Self {
        let mut ex = Self::from_status(status);
        ex.errors = reason;

        ex
    }

    /// gets the status given a valid integer code.
    pub fn get_status(&self) -> Status {
        Status::new(self.code)
    }
}
