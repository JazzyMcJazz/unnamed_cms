///
/// # Example Usage:
///
///```
/// use std::env;
///
/// use actix_web::{self, App, HttpServer};
/// use unnamed_cms_lib::cms::UnnamedCms;
/// use unnamed_cms_lib::surrealdb::{engine::any::connect, opt::auth::Root};
///
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///    
///     // Establish connection to the SurrealDB database
///     let surreal_url = env::var("SURREAL_URL").expect("SURREAL_URL is not set in envrionment");
///     let surreal = connect(surreal_url).await.unwrap();
///     surreal
///         .signin(Root {
///             username: &env::var("SURREAL_USER").expect("SURREAL_USER is not set in environment"),
///             password: &env::var("SURREAL_PASS").expect("SURREAL_PASS is not set in environment"),
///         })
///         .await
///         .unwrap();
///     surreal.use_ns("cms").use_db("cms").await.unwrap();
///
///    // Provision the database with system tables
///    UnnamedCms::init_db(&surreal).await.unwrap();
///
///
///     // Start the HTTP server
///     let mut server = HttpServer::new(move || {
///         let cms = UnnamedCms::new(&surreal).base_path("/");
///
///         App::new()
///             .configure(|cfg| {
///                 cms.config(cfg);
///             })
///             // your own configuration
///     });
///
///     server = server.bind("127.0.0.1:3000")?;
///     server.run().await?;
///
///     Ok(())
/// }
/// ```
pub use actix_web;
pub use surrealdb;

pub mod cms;

mod api;
mod config;
mod middleware;
mod prelude;
mod service;
mod utils;
