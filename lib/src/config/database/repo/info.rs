use std::collections::HashMap;

use surrealdb::{Surreal, engine::any::Any};

pub enum TableFilter {
    #[allow(dead_code)]
    System,
    #[allow(dead_code)]
    Custom,
    All,
}

#[allow(dead_code)]
async fn info(db: &Surreal<Any>) -> HashMap<String, HashMap<String, String>> {
    let mut result = db.query("INFO for DB;").await.unwrap();
    let info: Option<HashMap<String, HashMap<String, String>>> = result.take(0).unwrap();
    info.unwrap()
}

#[allow(dead_code)]
pub async fn table_names(db: &Surreal<Any>, filter: TableFilter) -> Vec<String> {
    let info = info(db).await;
    
    if let Some(tables) = info.get("tables") {
        dbg!(&tables);
        match filter {
            TableFilter::System => tables
                .iter()
                .filter(|(k, _)| k.starts_with("system_"))
                .map(|(k, _)| k.clone())
                .collect(),
            TableFilter::Custom => tables
                .iter()
                .filter(|(k, _)| !k.starts_with("system_"))
                .map(|(k, _)| k.clone())
                .collect(),
            TableFilter::All => tables
                .iter()
                .map(|(k, _)| k.clone())
                .collect(),
        }
    } else {
        Vec::new()
    }
}