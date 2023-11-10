use std::env;

use actix_web::{self, middleware::Logger, App, HttpServer};
use env_logger::Env;
use unnamed_cms::cms::UnnamedCms;
use unnamed_cms::surrealdb::{engine::any::connect, opt::auth::Root};

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    // Establish connection to the SurrealDB database
    let surreal_url = env::var("SURREAL_URL").expect("SURREAL_URL is not set in envrionment");
    let surreal = connect(surreal_url)
        .await
        .expect("Failed to connect to SurrealDB");
    surreal
        .signin(Root {
            username: &env::var("SURREAL_USER").expect("SURREAL_USER is not set in environment"),
            password: &env::var("SURREAL_PASS").expect("SURREAL_PASS is not set in environment"),
        })
        .await
        .expect("Failed to sign in to SurrealDB");
    surreal
        .use_ns("cms")
        .use_db("cms")
        .await
        .expect("Failed to use 'cms' namespace and database");

    UnnamedCms::init_db(&surreal)
        .await
        .expect("Failed to initialize CMS database");

    let cms = UnnamedCms::new(surreal);//.base_path("/cms");

    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Start the HTTP server
    let mut server = HttpServer::new(move || {
        App::new().wrap(Logger::default()).configure(|cfg| {
            cms.config(cfg);
        })
    });

    server = server.bind("0.0.0.0:3000")?;
    server.run().await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
