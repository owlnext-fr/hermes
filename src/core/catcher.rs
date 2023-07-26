use rocket::serde::json::Json;

use crate::core::response::{ApiResponse, ErrorMessage};
use crate::core::validation::{CachedParseErrors, CachedValidationErrors};
use crate::exceptions::dto::http_exception::HttpException;
use rocket::http::Status;
use rocket::{Request, catch};

const DEFAULT_ERROR_MESSAGE: &str = "__DEFAULT__";

/// default catcher for HTTP errors, invoked uppon recieving early errors from
/// fairings & guards.
#[catch(default)]
pub fn default_catcher(status: Status, req: &Request) -> ApiResponse<HttpException> {
    // trying to catch standard error messages from fairings & guards
    let possible_reason = req.local_cache(|| ErrorMessage {
        message: DEFAULT_ERROR_MESSAGE.into(),
    });

    // trying to catch parsing error messages from input parsers
    // this will happen if data input is malformed, BEFORE validation steps
    let possible_parse_violation = req.local_cache(|| CachedParseErrors(None)).0.as_ref();

    // trying to catch validation error messages from input validator
    // those will be last error produced by automated fairings & guards
    let possible_validation_violation = req.local_cache(|| CachedValidationErrors(None)).0.as_ref();

    // searching reason to display
    let mut reason: Option<String> = None;

    if possible_reason.message != DEFAULT_ERROR_MESSAGE {
        reason = Some(possible_reason.message.clone());
    }

    if reason.is_none() {
        if let Some(parse_violation) = possible_parse_violation {
            reason = Some(parse_violation.clone())
        }
    }

    if reason.is_none() {
        if let Some(validation_violation) = possible_validation_violation {
            reason = Some(validation_violation.to_string());
        }
    }

    ApiResponse {
        json: Json(HttpException::from_code_with_reason(status.code, reason)),
        status,
    }
}
