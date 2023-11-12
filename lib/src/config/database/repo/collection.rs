use serde_json::Value;
use surrealdb::{engine::any::Any, Surreal};

use crate::{
    config::database::models::{Field, TableInfo},
    prelude::{CmsResponse, SELECT_ALL_FROM_COLLECTIION},
};

pub struct CollectionRepo<'a> {
    db: &'a Surreal<Any>,
}

impl<'a> CollectionRepo<'a> {
    pub fn new(db: &'a Surreal<Any>) -> Self {
        Self { db }
    }

    pub async fn find_all(
        &self,
        collection: String,
    ) -> Result<(Vec<Field>, Vec<Value>), CmsResponse> {
        let query = SELECT_ALL_FROM_COLLECTIION.replace("{$table}", collection.as_str());
        let mut result = match self.db.query(query).bind(("table", &collection)).await {
            Ok(result) => result,
            Err(e) => return Err(CmsResponse::from(e)),
        };

        let info: Option<TableInfo> = match result.take(0) {
            Ok(info) => info,
            Err(e) => return Err(CmsResponse::from(e)),
        };

        let fields = Field::from_info(info.as_ref().unwrap());
        dbg!(&fields);

        let records: Vec<Value> = match result.take(1) {
            Ok(records) => records,
            Err(e) => return Err(CmsResponse::from(e)),
        };

        // dbg!(&data);
        Ok((fields, records))
    }
}
