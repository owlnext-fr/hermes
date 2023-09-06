use std::env;

use anyhow::{Context, Result};
use rocket::{Build, Rocket};

use crate::{
    core::{
        commands::{
            command_trait::{CommandArgs, CommandTrait},
            command_utils::ConsoleIO,
        },
        database::{Connected, DatabaseState},
    },
    middlewares::site_middleware::SiteMiddleware,
    model::site::{NewSiteDTO, Site},
};

use rocket::tokio::fs;
use slugify::slugify;

#[derive(Clone, Default)]
pub struct ScanSitesCommand;

#[async_trait::async_trait]
impl<'a> CommandTrait<'a> for ScanSitesCommand {
    fn name(&self) -> &'a str {
        "scan_sites"
    }

    fn description(&self) -> &'a str {
        "Scans all sites in the site directory and syncs them with the database"
    }

    fn is_parallel(&self) -> bool {
        false
    }

    async fn do_run(
        &self,
        rocket: &Rocket<Build>,
        io: &ConsoleIO,
        _args: &CommandArgs,
    ) -> Result<()> {
        let db_conn = rocket.state::<DatabaseState<Connected>>().unwrap();
        let site_middleware = SiteMiddleware::new(db_conn.get_new_connection());

        let nb_step = 10;

        io.step(1, nb_step, "Scanning for sites...");
        let base_path = env::var("SITES_BASE_PATH").with_context(|| {
            "Please set the SITES_BASE_PATH environment variable to the path of your sites"
        })?;

        let mut sites = Vec::<String>::new();

        let mut site_directories = fs::read_dir(&base_path)
            .await
            .with_context(|| "Unable to read the site directory")?;

        while let Some(site_directory) = site_directories.next_entry().await? {
            if site_directory.file_type().await?.is_dir() {
                let site_name = site_directory.file_name().to_string_lossy().to_string();

                io.writeln(&format!(" - Found site {}", slugify!(&site_name)));

                sites.push(site_name);
            }
        }

        io.step(
            2,
            nb_step,
            "Gathering already existing sites from database...",
        );

        let existing_sites = site_middleware.find_all().await?;

        for site in &existing_sites {
            io.writeln(&format!(" - Found site {}", &site.name));
        }

        io.step(3, nb_step, "Computing differences...");
        let mut sites_to_create = Vec::<String>::new();
        let mut sites_to_delete = Vec::<&Site>::new();

        for site in &sites {
            if !existing_sites.iter().any(|s| s.name == slugify!(site)) {
                sites_to_create.push(site.clone());
            }
        }

        for site in &existing_sites {
            let site_name = site.name.clone();

            if !sites.iter().any(|s| slugify!(s) == site_name) {
                sites_to_delete.push(site);
            }
        }

        io.step(4, nb_step, "Creating new sites...");
        for site in sites_to_create {
            io.writeln(&format!(" - Creating site {}", slugify!(&site)));

            let site_pathbuf = fs::canonicalize(format!("{}/{}", &base_path, &site))
                .await
                .with_context(|| {
                    format!(
                        "Unable to find the path of the site {}",
                        format!("{}/{}", base_path, &site)
                    )
                })?;

            let new_site_dto = NewSiteDTO { path: site_pathbuf };

            let new_site = site_middleware.create(&new_site_dto, None).await?;

            io.writeln(&format!("   - Created site {}", &new_site.name));
        }

        io.step(5, nb_step, "Deleting non-existant sites...");

        for site in sites_to_delete {
            io.writeln(&format!(" - Deleting site {}", &site.name));

            site_middleware.delete(site).await?;

            io.writeln(&format!("   - Deleted site {}", &site.name));
        }

        Ok(())
    }
}
