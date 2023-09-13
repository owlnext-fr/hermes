use rocket::{fs::NamedFile, get};
use std::path::{Path, PathBuf};

#[get("/<file..>")]
pub async fn serve(file: PathBuf) -> Option<NamedFile> {
    // site guessing by url

    let mut file = file;
    let path_as_str = file.to_str().unwrap();

    if path_as_str.ends_with("/") {
        file = file.join("index.html");
    }

    NamedFile::open(Path::new("/srv/owlnext/hermes/data/sites/site_1").join(file))
        .await
        .ok()
}
