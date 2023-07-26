use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket::serde::json::Json;
use serde::Serialize;

use crate::exceptions::dto::http_exception::HttpException;

pub type ApiResult<T> = Result<ApiResponse<T>, ApiResponse<HttpException>>;

/// Generic struct to represent an JSON HTTP response transport (e.g. representation in rocket processes).
#[derive(Debug)]
pub struct ApiResponse<T> {
    /// JSON body of the response.
    pub json: Json<T>,
    /// HTTP status of the response
    pub status: Status,
}

impl<T> ApiResponse<T> {
    /// Shorthand method to generate an HTTP 200 - OK with a JSON body
    pub fn ok(output: Json<T>) -> Self {
        ApiResponse {
            json: output,
            status: Status::Ok,
        }
    }

    /// Shorthand method to generate an HTTP response transport representation with a body and a status.
    pub fn custom(output: Json<T>, status: Status) -> Self {
        ApiResponse {
            json: output,
            status,
        }
    }
}

/// Implementation of the ApiResponse transport for HttpException responses.
/// This is usefull in controllers to return a business-logic error.
impl ApiResponse<HttpException> {
    /// Generates an automated "exception response" for the given status.
    pub fn from_status(status: Status) -> Self {
        ApiResponse {
            json: Json(HttpException::from_status(status)),
            status,
        }
    }

    /// Generates an automated "exception response" for the given status and a reason.
    pub fn from_status_with_reason(status: Status, reason: &str) -> Self {
        ApiResponse {
            json: Json(HttpException::from_status_with_reason(
                status,
                Some(reason.into()),
            )),
            status,
        }
    }
}

/// Representation of an HTTP 204 - No Content response
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NoContentResponse;

impl ApiResponse<NoContentResponse> {
    /// Shorthand method for ApiResponse to create an HTTP 204 - No Content response.
    pub fn no_content() -> Self {
        ApiResponse {
            json: Json(NoContentResponse {}),
            status: Status::NoContent,
        }
    }
}

#[rocket::async_trait]
impl<'r, T: serde::Serialize> Responder<'r, 'r> for ApiResponse<T> {
    /// Responder to handle ApiResponse transport
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        if self.status == Status::NoContent {
            // builds a response with no content
            return Response::build()
                .status(self.status)
                .header(ContentType::JSON)
                .ok();
        } else if self.status == Status::InternalServerError && !cfg!(debug_assertions) {
            // intercepts 500 errors to avoid runtime error diffusion (e.g. database errors or potentialy secure information about the application).
            let json = Json(HttpException::from_status(self.status));

            return Response::build_from(json.respond_to(req).unwrap())
                .status(self.status)
                .header(ContentType::JSON)
                .ok();
        }

        Response::build_from(self.json.respond_to(req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

/// Generic transport for pre-controller (e.g. fairing) business logic errors to store in the request local cache.
#[derive(Default, Debug)]
pub struct ErrorMessage {
    /// The error message
    pub message: String,
}
