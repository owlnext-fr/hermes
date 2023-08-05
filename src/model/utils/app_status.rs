use serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AppStatus {
    pub name: String,
    pub description: String,
    pub version: String,
}

impl AppStatus {
    pub fn new() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME").to_string(),
            description: env!("CARGO_PKG_DESCRIPTION").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}
