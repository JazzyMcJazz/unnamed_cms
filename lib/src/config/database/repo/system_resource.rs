use crate::prelude::*;

use super::super::models::{ResourceType, SystemResource};

pub async fn create(
    db: &Surreal<Any>,
    resource: &SystemResource,
) -> Result<SystemResource, CmsResponse> {
    // Unsafe approach, but it's currently the only way to create a table with a dynamic name
    // The risk is mitigated by the fact that the table name is validated before this function is called
    // This is further reinforced that resource.name is private and is guaranteed to be sanitized through the SystemResource::from_form function
    let query = CREATE_RESOURCE.replace("{$name}", resource.name().as_str());

    dbg!(&query);
    let mut result = match db
        .query(query)
        .bind(("name", &resource.name()))
        .bind(("display_name", &resource.display_name))
        .bind(("description", &resource.description))
        .bind(("type", &resource.resource_type))
        .await
    {
        Ok(result) => result,
        Err(e) => return Err(CmsResponse::from(e)),
    };

    let resource: Option<SystemResource> = match result.take(0) {
        Ok(resource) => resource,
        Err(e) => return Err(CmsResponse::from(e)),
    };

    let errors = result.take_errors();
    if !errors.is_empty() {
        return Err(CmsResponse::from(errors));
    }

    Ok(resource.unwrap())
}

pub async fn find_resource_by_type(
    db: &Surreal<Any>,
    resource_type: ResourceType,
) -> Result<Vec<SystemResource>, CmsResponse> {
    let mut result = match db
        .query("SELECT * FROM system_resource WHERE resource_type IS $t;")
        .bind(("t", resource_type))
        .await
    {
        Ok(result) => result,
        Err(e) => return Err(CmsResponse::from(e)),
    };

    let resources: Vec<SystemResource> = match result.take(0) {
        Ok(resources) => resources,
        Err(e) => return Err(CmsResponse::from(e)),
    };

    Ok(resources)
}
