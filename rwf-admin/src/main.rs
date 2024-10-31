use rwf::controller::BasicAuth;
use rwf::{
    controller::{StaticFiles, TurboStream},
    http::{self, Server},
    prelude::*,
};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), http::Error> {
    Logger::init();
    Migrations::migrate().await?;

    // Enable HMR.
    rwf::hmr::hmr(PathBuf::from("templates"));

    // Basic auth is just an example, it's not secure. I would recommend using SessionAuth
    // and checking that the user is an admin using an internal check.
    let admin = rwf_admin::engine().auth(AuthHandler::new(BasicAuth {
        user: "admin".to_string(),
        password: "admin".to_string(),
    }));

    Server::new(vec![
        engine!("/admin" => admin),
        route!("/turbo-stream" => TurboStream),
        StaticFiles::serve("static")?,
    ])
    .launch("0.0.0.0:8000")
    .await
}
