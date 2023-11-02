use surrealdb::{engine::any::Any, Error, Surreal};

pub async fn configure(db: &Surreal<Any>) -> Result<(), Error> {
    db.query("todo").await?;
    Ok(())
}
