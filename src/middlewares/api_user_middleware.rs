use anyhow::{Result, bail};
use surrealdb::{Surreal, engine::remote::ws::Client, Error};
use thiserror::Error;

use crate::model::api_user::{NewApiUserDTO, ApiUser, API_USER_TABLE};

use super::impls::model_middleware_trait::ModelMiddlewareTrait;

#[derive(Debug, Error)]
pub enum ApiUserMiddlewareError {
    #[error("database error: {0}")]
    DatabaseError(String),
}

#[derive(Clone)]
pub struct ApiUserMiddleware {
    pub db: Surreal<Client>,
}

impl ApiUserMiddleware {
    pub fn new(db: Surreal<Client>) -> Self {
        Self {
            db
        }
    }

    pub async fn create(&self, input: NewApiUserDTO) -> Result<ApiUser> {
        let mut user = ApiUser::from(input);

        let hashed_key = sha256::digest(user.api_key.clone());
        user.api_key = hashed_key;

        self.flag_creation(&mut user)?;

        let created = self.db
            .create(API_USER_TABLE)
            .content(user).await;

        if let Err(error) = &created {
            bail!(ApiUserMiddlewareError::DatabaseError(error.to_string()));
        }

        Ok(created?)
    }

    pub async fn delete(&self, user: &mut ApiUser) -> Result<ApiUser> {
        self.flag_deletion(user)?;

        let id = user.id.clone().unwrap().id;

        let deleted: Result<ApiUser, Error> = self.db
            .update((API_USER_TABLE, id))
            .content(user).await;

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

        let result = self.db
            .query(sql)
            .await;

        if let Err(error) = &result {
            bail!(ApiUserMiddlewareError::DatabaseError(error.to_string()));
        }

        Ok(result?.take(0)?)
    }
}

impl ModelMiddlewareTrait for ApiUserMiddleware {}