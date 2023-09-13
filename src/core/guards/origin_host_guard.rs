use crate::{
    core::{
        cipher::Cipher,
        database::{Connected, DatabaseState},
        response::ErrorMessage,
    },
    middlewares::api_user_middleware::{ApiUserMiddleware, ApiUserMiddlewareError},
    model::api_user::ApiUser,
};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use thiserror::Error;
use url::Url;

pub struct OriginHost(pub String);

#[derive(Debug, Error)]
pub enum OriginHostError {
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    #[error("Origin header not found")]
    HeaderNotFound,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for OriginHost {
    type Error = OriginHostError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let origin = req.headers().get_one("Host");

        if origin.is_none() {
            return Outcome::Failure((Status::BadRequest, OriginHostError::HeaderNotFound));
        }

        let origin = origin.unwrap();

        let uri = Url::parse(&format!("http://{}", origin)).unwrap();

        Outcome::Success(OriginHost(uri.host().unwrap().to_string()))
    }
}
