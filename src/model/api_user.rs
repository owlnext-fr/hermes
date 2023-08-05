use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::impls::model_trait::ModelTrait;

pub const API_USER_TABLE: &str = "api_user";

// ---------------------------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ApiUser {
    pub id: Option<Thing>,
    pub name: String,
    pub api_key: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub is_deleted: bool,
}

impl ModelTrait for ApiUser {
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

impl From<NewApiUserDTO> for ApiUser {
    fn from(value: NewApiUserDTO) -> Self {
        Self {
            id: None,
            name: value.name,
            api_key: value.api_key,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            is_deleted: false,
        }
    }
}

// ---------------------------------------------------------------------------------------------
// DTOs
// ---------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NewApiUserDTO {
    pub name: String,
    pub api_key: String,
}
