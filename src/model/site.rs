use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use thiserror::Error;
use validator::Validate;

use super::{api_user::ApiUser, impls::model_trait::ModelTrait};
use slugify::slugify;

pub const SITE_TABLE: &str = "site";

// ---------------------------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Site {
    pub id: Option<Thing>,
    pub name: String,
    pub path: String,
    pub created_by: Option<Thing>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub is_deleted: bool,
}

impl Site {
    pub fn get_created_by(&self) -> Option<Thing> {
        self.created_by.clone()
    }

    pub fn set_created_by(&mut self, created_by: Thing) -> &mut Self {
        self.created_by = Some(created_by);
        self
    }
}

impl ModelTrait for Site {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Error)]
pub enum SiteCreationError {
    #[error("The given path is empty")]
    EmptyPath,
    #[error("The given path is not valid")]
    InvalidPath,
    #[error("The given path's base name is not valid")]
    InvalidBaseName,
}

impl TryFrom<&NewSiteDTO> for Site {
    type Error = SiteCreationError;

    #[allow(unused_assignments)]
    fn try_from(dto: &NewSiteDTO) -> Result<Self, Self::Error> {
        let mut folder_real_name = String::new();
        let mut folder_path = String::new();

        if let Some(path) = dto.path.to_str() {
            if path.is_empty() {
                return Err(SiteCreationError::EmptyPath);
            }

            folder_path = path.to_string();

            if let Some(folder_name) = dto.path.file_name() {
                folder_real_name = folder_name.to_str().unwrap().to_string();
            } else {
                return Err(SiteCreationError::InvalidBaseName);
            }
        } else {
            return Err(SiteCreationError::InvalidPath);
        }

        let name = slugify!(&folder_real_name);

        let site = Site {
            id: None,
            name,
            path: folder_path,
            created_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            is_deleted: false,
        };

        Ok(site)
    }
}

impl Into<SiteDetailsDTO> for Site {
    fn into(self) -> SiteDetailsDTO {
        SiteDetailsDTO {
            id: self.id,
            name: self.name,
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
pub struct NewSiteDTO {
    pub path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SiteDetailsDTO {
    pub id: Option<Thing>,
    pub name: String,
    pub created_by: Option<Thing>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub is_deleted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SiteDetailsFetchedDTO {
    pub id: Option<Thing>,
    pub name: String,
    pub created_by: Option<ApiUser>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub is_deleted: bool,
}
