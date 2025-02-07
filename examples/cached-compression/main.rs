#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_async_compression::CachedCompression;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            FileServer::new(relative!("examples/cached-compression/static")),
        )
        .attach(CachedCompression::path_suffix_fairing(
            CachedCompression::static_paths(vec![".txt"]),
        ))
}
