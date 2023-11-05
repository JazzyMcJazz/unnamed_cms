use crate::config::{self, app_state::AppState, database::Repository};
use actix_web::web::ServiceConfig;
use surrealdb::{engine::any::Any, Error, Surreal};

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
///     let surreal_url = env::var("SURREAL_URL").unwrap();
///     let surreal = connect(surreal_url).await.unwrap();
///     surreal
///         .signin(Root {
///             username: &env::var("SURREAL_USER").unwrap(),
///             password: &env::var("SURREAL_PASS").unwrap(),
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
///         let cms = UnnamedCms::new(&surreal).prefix("/");
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
pub struct UnnamedCms<'a> {
    database: &'a Surreal<Any>,
    prefix: &'static str,
}

impl<'a> UnnamedCms<'a> {
    ///
    /// Create a new instance of the CMS
    ///
    /// ### Example
    ///
    /// ```
    /// let cms = UnnamedCms::new(&surreal);
    /// ```
    ///
    /// ### Arguments
    ///
    /// * `database` - The SurrealDB database
    ///
    /// ### Returns
    ///
    /// * `Self` - The CMS instance
    ///
    pub fn new(database: &'a Surreal<Any>) -> Self {
        Self {
            database,
            prefix: "/cms",
        }
    }

    ///
    /// Set the prefix for the CMS routes
    ///
    /// ### Example
    ///
    /// ```
    /// let cms = UnnamedCms::new(&surreal).prefix("/cms");
    /// ```
    ///
    /// ### Arguments
    ///
    /// * `prefix` - The prefix for the CMS routes
    ///
    /// ### Returns
    ///
    /// * `Self` - The CMS instance
    ///
    /// ### Remarks
    ///
    /// The default prefix is `/`
    ///
    pub fn prefix(mut self, prefix: &'static str) -> Self {
        self.prefix = prefix;
        self
    }

    ///
    /// Configure the CMS routes
    ///
    /// ### Example
    ///
    /// ```
    /// use actix_web::{self, App, HttpServer};
    ///
    /// let mut server = HttpServer::new(move || {
    ///    let cms = UnnamedCms::new(&surreal).prefix("/");
    ///    App::new().configure(|cfg| {
    ///        cms.config(cfg);
    ///    })
    /// });
    /// ```
    ///
    /// ### Arguments
    ///
    /// * `cfg` - The ActixWeb ServiceConfig
    ///
    /// ### Remarks
    ///
    /// This function is called by the ActixWeb framework
    ///
    pub fn config(&self, cfg: &mut ServiceConfig) {
        let app_state = AppState::new(self.database.clone(), self.prefix);
        config::app_data::configure(cfg, app_state);
        config::routes::configure(cfg, self.prefix);
    }

    ///
    /// Provision the database with system tables
    ///
    /// ### Example
    ///
    /// ```
    /// UnnamedCms::init_db(&surreal).await.unwrap();
    /// ```
    ///
    /// ### Arguments
    ///
    /// * `db` - The SurrealDB database
    ///
    /// ### Returns
    ///
    /// * `Result<(), Error>` - The result of the operation
    ///
    /// ### Remarks
    ///
    /// This function should be called before starting the HTTP server
    ///
    pub async fn init_db(db: &Surreal<Any>) -> Result<(), Error> {
        db.init().await?;
        Ok(())
    }
}
