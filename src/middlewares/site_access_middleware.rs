use anyhow::{bail, Result};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};
use thiserror::Error;

use super::{
    impls::model_middleware_trait::ModelMiddlewareTrait, site_middleware::SiteMiddleware,
    user_middleware::UserMiddleware,
};
use crate::model::{
    api_user::ApiUser,
    site::{Site, SiteDetailsDTO, SiteDetailsFetchedDTO, SITE_TABLE},
    site_access::{
        NewSiteAccessDTO, SiteAccess, SiteAccessDetailsDTO, SiteAccessDetailsFetchedDTO,
        UpdateSiteAccessDTO, SITE_ACCESS_TABLE,
    },
    user::USER_TABLE,
};

use surrealdb::Error;

#[derive(Debug, Error)]
pub enum SiteAccessMiddlewareError {
    #[error("database error: {0}")]
    DatabaseError(String),
    #[error("data error: {0}")]
    DataError(String),
    #[error("validation error: {0}")]
    ValidationError(String),
}

#[derive(Clone)]
pub struct SiteAccessMiddleware {
    pub db: Surreal<Client>,
}

impl ModelMiddlewareTrait for SiteAccessMiddleware {}

impl SiteAccessMiddleware {
    pub fn new(db: Surreal<Client>) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        new_site_access: &NewSiteAccessDTO,
        creator: Option<&ApiUser>,
        user_middleware: &UserMiddleware,
        site_middleware: &SiteMiddleware,
    ) -> Result<SiteAccess> {
        // user validation
        let maybe_user = user_middleware
            .find_one_by_id(new_site_access.user_id.clone())
            .await?;

        if maybe_user.is_none() {
            bail!(SiteAccessMiddlewareError::ValidationError(
                "user not found".to_string()
            ));
        }

        // site validation
        let maybe_site = site_middleware
            .find_one_by_id(new_site_access.site_id.clone())
            .await?;

        if maybe_site.is_none() {
            bail!(SiteAccessMiddlewareError::ValidationError(
                "site not found".to_string()
            ));
        }

        // dates validation
        if new_site_access.end_date.is_some() {
            let tmp_end_date = new_site_access.end_date.clone().unwrap();

            if new_site_access.start_date > tmp_end_date {
                bail!(SiteAccessMiddlewareError::ValidationError(
                    "start date must be before end date".to_string()
                ));
            }
        }

        // proceed to create
        let mut site_access = SiteAccess::try_from(new_site_access)?;

        site_access.user = Thing::from((USER_TABLE.into(), new_site_access.user_id.clone()));
        site_access.site = Thing::from((SITE_TABLE.into(), new_site_access.site_id.clone()));

        let creator_id = creator.unwrap().id.clone().unwrap();
        site_access.set_created_by(creator_id);

        self.flag_creation(&mut site_access)?;

        let created: SiteAccess = self
            .db
            .create(SITE_ACCESS_TABLE)
            .content(site_access)
            .await
            .map_err(|error| SiteAccessMiddlewareError::DatabaseError(error.to_string()))?;

        Ok(created)
    }

    pub async fn update(
        &self,
        site_access: &SiteAccess,
        update_data: &UpdateSiteAccessDTO,
        updater: Option<ApiUser>,
    ) -> Result<SiteAccess> {
        let mut site_access = site_access.clone();
        self.flag_update(&mut site_access)?;

        site_access.start_date = update_data.start_date.clone();
        site_access.end_date = update_data.end_date.clone();

        let updater_id = updater.unwrap().id.clone().unwrap();
        site_access.set_updated_by(updater_id);

        let id = site_access.id.clone().unwrap().id;

        let deleted: SiteAccess = self
            .db
            .update((SITE_ACCESS_TABLE, id))
            .content(site_access)
            .await
            .map_err(|error| SiteAccessMiddlewareError::DatabaseError(error.to_string()))?;

        Ok(deleted)
    }

    pub async fn delete(
        &self,
        site_access: &SiteAccess,
        deletor: Option<ApiUser>,
    ) -> Result<SiteAccess> {
        let mut site_access = site_access.clone();
        self.flag_deletion(&mut site_access)?;

        let deletor_id = deletor.unwrap().id.clone().unwrap();
        site_access.set_deleted_by(deletor_id);

        let id = site_access.id.clone().unwrap().id;

        let deleted: SiteAccess = self
            .db
            .update((SITE_ACCESS_TABLE, id))
            .content(site_access)
            .await
            .map_err(|error| SiteAccessMiddlewareError::DatabaseError(error.to_string()))?;

        Ok(deleted)
    }

    pub fn to_details(&self, site_access: &SiteAccess) -> SiteAccessDetailsDTO {
        site_access.clone().into()
    }

    pub async fn to_details_fetched(
        &self,
        site_access: &SiteAccess,
    ) -> Result<SiteAccessDetailsFetchedDTO> {
        let sql = r#"
        SELECT *
        FROM type::thing("site_access", $site_access_id)
        WHERE is_deleted = false
        FETCH created_by, updated_by, deleted_by
    "#;
        let site_access_id = site_access.id.clone().unwrap().id.to_string();

        let mut result = self
            .db
            .query(sql)
            .bind(("site_access_id", site_access_id))
            .await
            .map_err(|error| SiteAccessMiddlewareError::DatabaseError(error.to_string()))?;

        let site_access: Option<SiteAccessDetailsFetchedDTO> = result.take(0)?;

        if site_access.is_none() {
            bail!(SiteAccessMiddlewareError::DataError(
                "No site found with this id.".to_string()
            ));
        }

        Ok(site_access.unwrap())
    }
}
