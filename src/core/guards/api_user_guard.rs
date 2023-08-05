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

pub struct AuthenticatedApiUser {
    pub api_user: ApiUser,
}

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    #[error("X-API-KEY header not found")]
    HeaderNotFound,
    #[error("Invalid X-API-KEY header")]
    InvalidHeader,
    #[error("Token in X-API-KEY header is not found")]
    UserNotFound,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedApiUser {
    type Error = AuthenticationError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let db = req.rocket().state::<DatabaseState<Connected>>();

        if db.is_none() {
            let message = "Database connection not found".to_string();
            req.local_cache(|| ErrorMessage {
                message: message.clone(),
            });
            return Outcome::Failure((
                Status::InternalServerError,
                AuthenticationError::InternalServerError(message),
            ));
        }

        let db = db.unwrap();

        let api_user_middleware = ApiUserMiddleware::new(db.get_new_connection());

        let api_key = req.headers().get_one("X-API-KEY");

        if api_key.is_none() {
            req.local_cache(|| ErrorMessage {
                message: "X-API-KEY header not found".to_string(),
            });
            return Outcome::Failure((Status::BadRequest, AuthenticationError::HeaderNotFound));
        }

        let api_key = api_key.unwrap();

        if api_key.is_empty() {
            req.local_cache(|| ErrorMessage {
                message: "Invalid X-API-KEY header; api key is empty.".to_string(),
            });
            return Outcome::Failure((Status::BadRequest, AuthenticationError::InvalidHeader));
        }

        let ciphered = Cipher::hash(api_key);

        let api_user = api_user_middleware.get_by_api_key(&ciphered).await;

        if let Err(err) = &api_user {
            let orig_error = err.downcast_ref::<ApiUserMiddlewareError>();

            match orig_error {
                Some(ApiUserMiddlewareError::DatabaseError(error)) => {
                    req.local_cache(|| ErrorMessage {
                        message: error.clone(),
                    });
                    return Outcome::Failure((
                        Status::InternalServerError,
                        AuthenticationError::InternalServerError(error.clone()),
                    ));
                }
                Some(ApiUserMiddlewareError::DataError(error)) => {
                    req.local_cache(|| ErrorMessage {
                        message: error.clone(),
                    });
                    return Outcome::Failure((
                        Status::BadRequest,
                        AuthenticationError::UserNotFound,
                    ));
                }
                _ => {
                    let msg = format!("Unknown error: {}", err.to_string());
                    req.local_cache(|| ErrorMessage {
                        message: msg.clone(),
                    });
                    return Outcome::Failure((
                        Status::InternalServerError,
                        AuthenticationError::InternalServerError(msg),
                    ));
                }
            }
        }

        let api_user = api_user.unwrap();

        return Outcome::Success(AuthenticatedApiUser { api_user });
    }
}
