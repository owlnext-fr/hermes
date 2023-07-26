#![deny(clippy::all, clippy::cargo)]
#![forbid(unsafe_code)]

use rocket::catch;
#[allow(unused_imports)]
use rocket::{
    data::{Data, FromData, Outcome as DataOutcome},
    form,
    form::{DataField, FromForm, ValueField},
    http::Status,
    outcome::Outcome,
    request::{FromRequest, Request},
    serde::{json::Json, Serialize},
};
use std::fmt::Debug;
pub use validator::{Validate, ValidationErrors};

// reimplementation of the rocket_validator crate to include JSON shunt errors upon serialization and validation.
// original code https://github.com/somehowchris/rocket-validation

#[derive(Clone, Debug)]
pub struct Validated<T>(pub T);

impl<T> Validated<Json<T>> {
    #[inline]
    pub fn into_deep_inner(self) -> T {
        self.0 .0
    }
}

impl<T> Validated<T> {
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Error<'a> {
    code: u128,
    message: &'a str,
    errors: Option<&'a ValidationErrors>,
}

#[catch(400)]
pub fn validation_catcher<'a>(req: &'a Request) -> Json<Error<'a>> {
    Json(Error {
        code: 400,
        message: "Bad Request. The request could not be understood by the server due to malformed \
                  syntax.",
        errors: req.local_cache(|| CachedValidationErrors(None)).0.as_ref(),
    })
}

#[derive(Clone)]
pub struct CachedValidationErrors(pub Option<ValidationErrors>);

#[derive(Clone)]
pub struct CachedParseErrors(pub Option<String>);

#[rocket::async_trait]
impl<'r, D: Validate + rocket::serde::Deserialize<'r>> FromData<'r> for Validated<Json<D>> {
    type Error = Result<ValidationErrors, rocket::serde::json::Error<'r>>;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> DataOutcome<'r, Self> {
        let data_outcome = <Json<D> as FromData<'r>>::from_data(req, data).await;

        match data_outcome {
            Outcome::Failure((status, err)) => {
                req.local_cache(|| CachedParseErrors(Some(err.to_string())));
                Outcome::Failure((status, Err(err)))
            }
            Outcome::Forward(err) => Outcome::Forward(err),
            Outcome::Success(data) => match data.validate() {
                Ok(_) => Outcome::Success(Validated(data)),
                Err(err) => {
                    req.local_cache(|| CachedValidationErrors(Some(err.to_owned())));
                    Outcome::Failure((Status::BadRequest, Ok(err)))
                }
            },
        }
    }
}

#[rocket::async_trait]
impl<'r, D: Validate + FromRequest<'r>> FromRequest<'r> for Validated<D> {
    type Error = Result<ValidationErrors, D::Error>;
    async fn from_request(req: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let data_outcome = D::from_request(req).await;

        match data_outcome {
            Outcome::Failure((status, err)) => {
                let error_message = format!("{err:?}");
                req.local_cache(|| CachedParseErrors(Some(error_message)));
                Outcome::Failure((status, Err(err)))
            }
            Outcome::Forward(err) => Outcome::Forward(err),
            Outcome::Success(data) => match data.validate() {
                Ok(_) => Outcome::Success(Validated(data)),
                Err(err) => {
                    req.local_cache(|| CachedValidationErrors(Some(err.to_owned())));
                    Outcome::Failure((Status::BadRequest, Ok(err)))
                }
            },
        }
    }
}

#[rocket::async_trait]
impl<'r, T: Validate + FromForm<'r>> FromForm<'r> for Validated<T> {
    type Context = T::Context;

    #[inline]
    fn init(opts: form::Options) -> Self::Context {
        T::init(opts)
    }

    #[inline]
    fn push_value(ctxt: &mut Self::Context, field: ValueField<'r>) {
        T::push_value(ctxt, field)
    }

    #[inline]
    async fn push_data(ctxt: &mut Self::Context, field: DataField<'r, '_>) {
        T::push_data(ctxt, field).await
    }

    fn finalize(this: Self::Context) -> form::Result<'r, Self> {
        match T::finalize(this) {
            Err(err) => Err(err),
            Ok(data) => match data.validate() {
                Ok(_) => Ok(Validated(data)),
                Err(err) => Err(err
                    .into_errors()
                    .into_iter()
                    .map(|e| form::Error {
                        name: Some(e.0.into()),
                        kind: form::error::ErrorKind::Validation(std::borrow::Cow::Borrowed(e.0)),
                        value: None,
                        entity: form::error::Entity::Value,
                    })
                    .collect::<Vec<_>>()
                    .into()),
            },
        }
    }
}
