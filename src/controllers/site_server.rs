use rocket::{
    fs::NamedFile,
    get,
    http::uri::{Authority, Host, Uri},
    uri, State,
};
use std::{
    env,
    path::{Path, PathBuf},
};

use crate::{
    core::{
        cache::{self, CacheState},
        database::{self, DatabaseState},
        guards::origin_host_guard::OriginHost,
    },
    middlewares::site_middleware::SiteMiddleware,
    model::site,
};

#[get("/<file..>")]
pub async fn serve(
    file: PathBuf,
    origin_host: OriginHost,
    db_state: &State<DatabaseState<database::Connected>>,
    cache_state: &State<CacheState<cache::Connected>>,
) -> Option<NamedFile> {
    let site_middleware = SiteMiddleware::new(
        db_state.get_new_connection(),
        cache_state.get_new_connection(),
    );

    let sites = site_middleware.get_names_from_cache().await.unwrap();
    let mut whitelist: Vec<Host<'_>> = Vec::new();

    let base_site_domain = env::var("SITES_BASE_DOMAIN").expect("SITES_BASE_DOMAIN not set");

    let sites = sites
        .iter()
        .map(|v| format!("{}{}", v, base_site_domain))
        .collect::<Vec<String>>();

    for site in sites.iter() {
        let host = Host::new(Authority::parse(&site).unwrap());
        whitelist.push(host);
    }

    let host_header = Authority::parse(&origin_host.0).unwrap();

    let host = Host::new(host_header);
    let prefix = host.to_absolute("http", &whitelist);

    if let None = prefix {
        return None;
    }

    let prefix = prefix.unwrap();
    let final_host = prefix.authority().unwrap().host();

    let site_dir = final_host.replace(&base_site_domain, "");

    let sites_base_path = env::var("SITES_BASE_PATH").expect("SITES_BASE_PATH not set");
    let site_chroot_dir = Path::new(&sites_base_path).join(&site_dir);
    let file_path_buff = PathBuf::from(&sites_base_path).join(&site_dir).join(file);

    let cannonicalized_file_path = file_path_buff.canonicalize();

    if let Err(_) = cannonicalized_file_path {
        return None;
    }

    let cannonicalized_file_path = cannonicalized_file_path.unwrap();

    if !cannonicalized_file_path.starts_with(&site_chroot_dir) {
        return None;
    }

    NamedFile::open(cannonicalized_file_path).await.ok()
}
