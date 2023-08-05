use anyhow::{bail, Result};
use surrealdb::{engine::remote::ws::Client, Error, Surreal};
use thiserror::Error;

use crate::{
    core::cipher::Cipher,
    model::api_user::{ApiUser, NewApiUserDTO, API_USER_TABLE},
};

use super::impls::model_middleware_trait::ModelMiddlewareTrait;

#[derive(Debug, Error)]
pub enum ApiUserMiddlewareError {
    #[error("database error: {0}")]
    DatabaseError(String),
    #[error("data error: {0}")]
    DataError(String),
}

#[derive(Clone)]
pub struct ApiUserMiddleware {
    pub db: Surreal<Client>,
}

impl ApiUserMiddleware {
    pub fn new(db: Surreal<Client>) -> Self {
        Self { db }
    }

    pub async fn create(&self, input: NewApiUserDTO) -> Result<ApiUser> {
        let mut user = ApiUser::from(input);

        let hashed_key = Cipher::hash(&user.api_key);
        user.api_key = hashed_key;

        self.flag_creation(&mut user)?;

        let created = self.db.create(API_USER_TABLE).content(user).await;

        if let Err(error) = &created {
            bail!(ApiUserMiddlewareError::DatabaseError(error.to_string()));
        }

        Ok(created?)
    }

    pub async fn delete(&self, user: &mut ApiUser) -> Result<ApiUser> {
        self.flag_deletion(user)?;

        let id = user.id.clone().unwrap().id;

        let deleted: Result<ApiUser, Error> =
            self.db.update((API_USER_TABLE, id)).content(user).await;

        if let Err(error) = &deleted {
            bail!(ApiUserMiddlewareError::DatabaseError(error.to_string()));
        }

        Ok(deleted?)
    }

    pub async fn list_active_api_users(&self) -> Result<Vec<ApiUser>> {
        let sql = "
            SELECT *
            FROM api_user
            where is_deleted = false;
        ";

        let result = self.db.query(sql).await;

        if let Err(error) = &result {
            bail!(ApiUserMiddlewareError::DatabaseError(error.to_string()));
        }

        Ok(result?.take(0)?)
    }

    pub async fn get_by_api_key(&self, api_key: &str) -> Result<ApiUser> {
        let sql = "
            SELECT *
            FROM api_user
            where api_key = $api_key
            and is_deleted = false;
        ";

        let result = self.db.query(sql).bind(("api_key", api_key)).await;

        if let Err(error) = &result {
            bail!(ApiUserMiddlewareError::DatabaseError(error.to_string()));
        }

        let user: Option<ApiUser> = result?.take(0)?;

        if user.is_none() {
            bail!(ApiUserMiddlewareError::DataError(
                "No user found with this token.".to_string()
            ));
        }

        Ok(user.unwrap())
    }
}

impl ModelMiddlewareTrait for ApiUserMiddleware {}
