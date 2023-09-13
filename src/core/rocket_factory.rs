use anyhow::Result;
use rocket::{catchers, routes, Build, Rocket};

use crate::{
    commands::{
        add_api_user_command::AddApiUserCommand, dump_cache_command::DumpCacheCommand,
        remove_api_user_command::RemoveApiUserCommand, scan_sites_command::ScanSitesCommand,
        test_command::TestCommand,
    },
    controllers::{api, app, site_server},
};

use super::{
    cache::CacheState,
    commands::{command_registry::CommandRegistry, command_utils::ConsoleIO},
    database::DatabaseState,
    fairings::load_site_names_in_cache::LoadSiteNamesInCacheFairing,
};
use crate::core::catcher;

/// Build a rocket instance.
///
/// This function will build a rocket instance with all the required states, middlewares and routes.
pub async fn build() -> Result<Rocket<Build>> {
    // build rocket instance
    let mut build = rocket::build();

    // states
    let database = DatabaseState::connect().await?;
    let cache = CacheState::connect().await?;
    let console_io = ConsoleIO::new();
    let mut command_registry = CommandRegistry::new();

    // register commands
    if cfg!(debug_assertions) {
        command_registry.register(Box::new(TestCommand));
        command_registry.register(Box::new(DumpCacheCommand));
    }

    command_registry.register(Box::new(AddApiUserCommand));
    command_registry.register(Box::new(RemoveApiUserCommand));
    command_registry.register(Box::new(ScanSitesCommand));

    // routes
    build = build.mount(
        "/",
        routes![
            app::status,
            app::status_json,
            api::create_user,
            site_server::serve
        ],
    );

    if cfg!(debug_assertions) {
        build = build.mount("/", routes![app::test_api_user_auth,]);
    }

    // catchers
    build = build.register("/", catchers![catcher::default_catcher]);

    // manage states
    build = build.manage(database);
    build = build.manage(cache);
    build = build.manage(console_io);
    build = build.manage(command_registry);

    // fairings
    build = build.attach(LoadSiteNamesInCacheFairing);

    Ok(build)
}
