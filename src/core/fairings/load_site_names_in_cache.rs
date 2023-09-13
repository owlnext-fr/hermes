use std::io::Cursor;
use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Method, Status};
use rocket::{Data, Orbit, Request, Response, Rocket};

use crate::core::cache::{self, CacheState, Connected};
use crate::core::database::{self, DatabaseState};
use crate::middlewares::site_middleware::SiteMiddleware;

pub struct LoadSiteNamesInCacheFairing;

#[rocket::async_trait]
impl Fairing for LoadSiteNamesInCacheFairing {
    // This is a request and response fairing named "GET/POST Counter".
    fn info(&self) -> Info {
        Info {
            name: "Site name cache loader",
            kind: Kind::Liftoff,
        }
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        let db_conn = rocket
            .state::<DatabaseState<database::Connected>>()
            .unwrap();
        let cache_conn = rocket.state::<CacheState<cache::Connected>>().unwrap();
        let site_middleware = SiteMiddleware::new(
            db_conn.get_new_connection(),
            cache_conn.get_new_connection(),
        );

        site_middleware.load_names_in_cache().await.unwrap();
    }
}
