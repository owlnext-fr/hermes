use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use thiserror::Error;
use validator::Validate;

use super::{
    api_user::ApiUser,
    impls::model_trait::ModelTrait,
    site::SITE_TABLE,
    user::{UserDetailsDTO, USER_TABLE},
};

pub const SITE_ACCESS_TABLE: &str = "site_access";

// ---------------------------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SiteAccess {
    pub id: Option<Thing>,
    pub user: Thing,
    pub site: Thing,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub created_by: Option<Thing>,
    pub created_at: DateTime<Utc>,
    pub updated_by: Option<Thing>,
    pub updated_at: DateTime<Utc>,
    pub deleted_by: Option<Thing>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub is_deleted: bool,
}

impl SiteAccess {
    pub fn get_created_by(&self) -> Option<Thing> {
        self.created_by.clone()
    }

    pub fn set_created_by(&mut self, created_by: Thing) -> &mut Self {
        self.created_by = Some(created_by);
        self
    }

    pub fn get_updated_by(&self) -> Option<Thing> {
        self.updated_by.clone()
    }

    pub fn set_updated_by(&mut self, updated_by: Thing) -> &mut Self {
        self.updated_by = Some(updated_by);
        self
    }

    pub fn get_deleted_by(&self) -> Option<Thing> {
        self.deleted_by.clone()
    }

    pub fn set_deleted_by(&mut self, deleted_by: Thing) -> &mut Self {
        self.deleted_by = Some(deleted_by);
        self
    }
}

impl ModelTrait for SiteAccess {
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
pub enum SiteAccessCreationError {}

impl TryFrom<&NewSiteAccessDTO> for SiteAccess {
    type Error = SiteAccessCreationError;

    fn try_from(dto: &NewSiteAccessDTO) -> Result<Self, Self::Error> {
        let user_id = Thing::from((USER_TABLE.into(), dto.user_id.clone()));
        let site_id = Thing::from((SITE_TABLE.into(), dto.site_id.clone()));

        let site_access = SiteAccess {
            id: None,
            user: user_id,
            site: site_id,
            start_date: dto.start_date,
            end_date: dto.end_date,
            created_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            is_deleted: false,
            updated_by: None,
            deleted_by: None,
        };

        Ok(site_access)
    }
}

impl Into<SiteAccessDetailsDTO> for SiteAccess {
    fn into(self) -> SiteAccessDetailsDTO {
        SiteAccessDetailsDTO {
            id: self.id,
            user: self.user,
            site: self.site,
            start_date: self.start_date,
            end_date: self.end_date,
            created_by: self.created_by,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted_at: self.deleted_at,
            is_deleted: self.is_deleted,
            updated_by: self.updated_by,
            deleted_by: self.deleted_by,
        }
    }
}

// ---------------------------------------------------------------------------------------------
// DTOs
// ---------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct NewSiteAccessDTO {
    pub user_id: String,
    pub site_id: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct UpdateSiteAccessDTO {
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SiteAccessDetailsDTO {
    pub id: Option<Thing>,
    pub user: Thing,
    pub site: Thing,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub created_by: Option<Thing>,
    pub created_at: DateTime<Utc>,
    pub updated_by: Option<Thing>,
    pub updated_at: DateTime<Utc>,
    pub deleted_by: Option<Thing>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub is_deleted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SiteAccessDetailsFetchedDTO {
    pub id: Option<Thing>,
    pub user: UserDetailsDTO,
    pub site: SiteAccessDetailsDTO,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub created_by: Option<ApiUser>,
    pub created_at: DateTime<Utc>,
    pub updated_by: Option<ApiUser>,
    pub updated_at: DateTime<Utc>,
    pub deleted_by: Option<ApiUser>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub is_deleted: bool,
}
