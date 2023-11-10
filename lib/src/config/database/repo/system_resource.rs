use crate::prelude::*;

use super::super::models::{ResourceType, SystemResource};

pub async fn find_resource_by_type(
    db: &Surreal<Any>,
    resource_type: ResourceType,
) -> Result<Vec<SystemResource>, CmsError> {
    let mut result = match db
        .query("SELECT * FROM system_resource WHERE resource_type IS $t;")
        .bind(("t", resource_type))
        .await
    {
        Ok(result) => result,
        Err(e) => return Err(CmsError::from(e)),
    };

    let resources: Vec<SystemResource> = match result.take(0) {
        Ok(resources) => resources,
        Err(e) => return Err(CmsError::from(e)),
    };

    Ok(resources)
}
