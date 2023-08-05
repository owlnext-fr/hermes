use anyhow::{bail, Result};
use surrealdb::{engine::remote::ws::Client, Error, Surreal};
use thiserror::Error;

use crate::{
    core::cipher::Cipher,
    model::{
        api_user::{ApiUser, NewApiUserDTO, API_USER_TABLE},
        user::{NewUserDTO, User, UserDetailsDTO, UserDetailsFetchedDTO, USER_TABLE},
    },
};

use super::impls::model_middleware_trait::ModelMiddlewareTrait;

#[derive(Debug, Error)]
pub enum UserMiddlewareError {
    #[error("database error: {0}")]
    DatabaseError(String),
    #[error("data error: {0}")]
    DataError(String),
}

#[derive(Clone)]
pub struct UserMiddleware {
    pub db: Surreal<Client>,
}

impl UserMiddleware {
    pub fn new(db: Surreal<Client>) -> Self {
        Self { db }
    }

    pub async fn create(&self, new_user: &NewUserDTO, creator: &ApiUser) -> Result<User> {
        let mut user = User::from(new_user);

        let ciphered_password = Cipher::cipher(&user.password);
        user.password = ciphered_password;

        let creator_id = creator.id.clone().unwrap();
        user.set_created_by(creator_id);

        self.flag_creation(&mut user)?;

        let created = self.db.create(USER_TABLE).content(user).await;

        if let Err(error) = &created {
            bail!(UserMiddlewareError::DatabaseError(error.to_string()));
        }

        Ok(created?)
    }

    pub fn to_details(&self, user: User) -> UserDetailsDTO {
        user.into()
    }

    pub async fn to_details_fetched(&self, user: &User) -> Result<UserDetailsFetchedDTO> {
        let sql = r#"
            SELECT *
            FROM type::thing("user", $user_id)
            FETCH created_by
        "#;
        let user_id = user.id.clone().unwrap().id.to_string();

        let result = self.db.query(sql).bind(("user_id", user_id)).await;

        if let Err(error) = &result {
            bail!(UserMiddlewareError::DatabaseError(error.to_string()));
        }

        let user: Option<UserDetailsFetchedDTO> = result?.take(0)?;

        if user.is_none() {
            bail!(UserMiddlewareError::DataError(
                "No user found with this id.".to_string()
            ));
        }

        Ok(user.unwrap())
    }

    pub fn find_by_login(&self, login: String) -> Result<User> {
        todo!()
    }
}

impl ModelMiddlewareTrait for UserMiddleware {}