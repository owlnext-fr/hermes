use anyhow::{bail, Result};
use redis::{AsyncCommands, Client as RedisClient};
use surrealdb::{engine::remote::ws::Client, Surreal};
use thiserror::Error;

use super::impls::model_middleware_trait::ModelMiddlewareTrait;
use crate::model::{
    api_user::ApiUser,
    site::{NewSiteDTO, Site, SiteDetailsDTO, SiteDetailsFetchedDTO, SITE_TABLE},
};
use surrealdb::Error;

const SITE_CACHE_KEY: &str = "app:sites";

#[derive(Debug, Error)]
pub enum SiteMiddlewareError {
    #[error("database error: {0}")]
    DatabaseError(String),
    #[error("data error: {0}")]
    DataError(String),
}

#[derive(Clone)]
pub struct SiteMiddleware {
    pub db: Surreal<Client>,
    pub cache: RedisClient,
}

impl ModelMiddlewareTrait for SiteMiddleware {}

impl SiteMiddleware {
    pub fn new(db: Surreal<Client>, cache: RedisClient) -> Self {
        Self { db, cache }
    }

    pub async fn create(&self, new_site: &NewSiteDTO, creator: Option<&ApiUser>) -> Result<Site> {
        let mut site = Site::try_from(new_site)?;

        if let Some(user) = creator {
            site.set_created_by(user.id.clone().unwrap());
        }

        self.flag_creation(&mut site)?;

        let created = self.db.create(SITE_TABLE).content(site).await;

        if let Err(error) = &created {
            bail!(SiteMiddlewareError::DatabaseError(error.to_string()));
        }

        self.load_names_in_cache().await?;

        Ok(created?)
    }

    pub async fn delete(&self, site: &Site) -> Result<Site> {
        let mut site = site.clone();
        self.flag_deletion(&mut site)?;

        let id = site.id.clone().unwrap().id;

        let deleted: Result<Site, Error> = self.db.update((SITE_TABLE, id)).content(site).await;

        if let Err(error) = &deleted {
            bail!(SiteMiddlewareError::DatabaseError(error.to_string()));
        }

        self.load_names_in_cache().await?;

        Ok(deleted?)
    }

    pub fn to_details(&self, site: &Site) -> SiteDetailsDTO {
        let site = site.clone();
        site.into()
    }

    pub async fn to_details_fetched(&self, site: &Site) -> Result<SiteDetailsFetchedDTO> {
        let sql = r#"
            SELECT *
            FROM type::thing("site", $site_id)
            WHERE is_deleted = false
            FETCH created_by
        "#;
        let site_id = site.id.clone().unwrap().id.to_string();

        let result = self.db.query(sql).bind(("site_id", site_id)).await;

        if let Err(error) = &result {
            bail!(SiteMiddlewareError::DatabaseError(error.to_string()));
        }

        let site: Option<SiteDetailsFetchedDTO> = result?.take(0)?;

        if site.is_none() {
            bail!(SiteMiddlewareError::DataError(
                "No site found with this id.".to_string()
            ));
        }

        Ok(site.unwrap())
    }

    pub async fn find_all(&self) -> Result<Vec<Site>> {
        let sql = r#"
            SELECT *
            FROM site
            WHERE is_deleted = false
        "#;

        let result = self.db.query(sql).await;

        if let Err(error) = &result {
            bail!(SiteMiddlewareError::DatabaseError(error.to_string()));
        }

        let sites: Vec<Site> = result?.take(0)?;

        Ok(sites)
    }

    pub async fn find_one_by_id(&self, thing: String) -> Result<Option<Site>> {
        let sql = r#"
            SELECT *
            FROM type::thing("site", $site_id)
            FETCH created_by
        "#;

        let result = self.db.query(sql).bind(("site_id", thing)).await;

        if let Err(error) = &result {
            bail!(SiteMiddlewareError::DatabaseError(error.to_string()));
        }

        let user: Option<Site> = result?.take(0)?;

        Ok(user)
    }

    pub async fn load_names_in_cache(&self) -> Result<()> {
        let sql = r#"
            SELECT VALUE name
            FROM site
            WHERE is_deleted = false
        "#;

        let result = self.db.query(sql).await;

        if let Err(error) = &result {
            bail!(SiteMiddlewareError::DatabaseError(error.to_string()));
        }

        let site_names: Vec<String> = result?.take(0)?;

        let mut conn = self.cache.get_async_connection().await?;

        let _: () = conn
            .set(SITE_CACHE_KEY, serde_json::to_string(&site_names)?)
            .await?;

        Ok(())
    }

    pub async fn get_names_from_cache(&self) -> Result<Vec<String>> {
        let mut conn = self.cache.get_async_connection().await?;

        let sites: String = conn.get(SITE_CACHE_KEY).await?;

        let sites: Vec<String> = serde_json::from_str(&sites)?;

        Ok(sites)
    }
}
