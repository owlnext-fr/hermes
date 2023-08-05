use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use validator::{Validate, ValidationError};

use crate::core::cipher::Cipher;

use super::{api_user::ApiUser, impls::model_trait::ModelTrait};

pub const USER_TABLE: &str = "user";

// ---------------------------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct User {
    pub id: Option<Thing>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_by: Option<Thing>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub is_deleted: bool,
}

impl User {
    pub fn get_created_by(&self) -> Option<Thing> {
        self.created_by.clone()
    }

    pub fn set_created_by(&mut self, created_by: Thing) -> &mut Self {
        self.created_by = Some(created_by);
        self
    }
}

impl ModelTrait for User {
    fn get_created_date(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn get_updated_date(&self) -> DateTime<Utc> {
        self.updated_at
    }

    fn get_deleted_date(&self) -> Option<DateTime<Utc>> {
        self.deleted_at
    }

    fn get_is_deleted(&self) -> bool {
        self.is_deleted
    }

    fn set_created_date(&mut self, created_date: DateTime<Utc>) -> &mut Self {
        self.created_at = created_date;
        self
    }

    fn set_updated_date(&mut self, updated_date: DateTime<Utc>) -> &mut Self {
        self.updated_at = updated_date;
        self
    }

    fn set_deleted_date(&mut self, deleted_date: Option<DateTime<Utc>>) -> &mut Self {
        self.deleted_at = deleted_date;
        self
    }

    fn set_is_deleted(&mut self, is_deleted: bool) -> &mut Self {
        self.is_deleted = is_deleted;
        self
    }
}

impl From<&NewUserDTO> for User {
    fn from(new_user: &NewUserDTO) -> Self {
        Self {
            id: None,
            name: new_user.name.clone(),
            email: new_user.email.clone(),
            password: new_user.password.clone(),
            created_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            is_deleted: false,
        }
    }
}

impl Into<UserDetailsDTO> for User {
    fn into(self) -> UserDetailsDTO {
        UserDetailsDTO {
            id: self.id,
            name: self.name,
            email: self.email,
            created_by: self.created_by,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted_at: self.deleted_at,
            is_deleted: self.is_deleted,
        }
    }
}

// ---------------------------------------------------------------------------------------------
// DTOs
// ---------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct NewUserDTO {
    #[validate(length(min = 3, max = 255))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3, max = 255))]
    #[validate(custom(
        function = "validate_password",
        message = "The password must be at least 10 characters long and contain at least one number, one uppercase letter, one lowercase letter and one special character"
    ))]
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserDetailsDTO {
    pub id: Option<Thing>,
    pub name: String,
    pub email: String,
    pub created_by: Option<Thing>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub is_deleted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserDetailsFetchedDTO {
    pub id: Option<Thing>,
    pub name: String,
    pub email: String,
    pub created_by: ApiUser,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub is_deleted: bool,
}

// ---------------------------------------------------------------------------------------------
// Utils function
// ---------------------------------------------------------------------------------------------

pub fn validate_password(msg: &str) -> Result<(), ValidationError> {
    let result = Cipher::validate_password_complexity(msg);

    if result {
        Ok(())
    } else {
        Err(ValidationError::new("password_complexity"))
    }
}
