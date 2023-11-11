use std::collections::HashMap;

use crate::{
    config::database::ResourceType,
    prelude::*,
    service::{FieldType, Table, TableField},
};

pub async fn content_index(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let db = data.database();
    let mut context = Extensions::unwrap_context(&req);

    let tables = match db.find_resource_by_type(ResourceType::SystemTable).await {
        Ok(tables) => tables,
        Err(e) => return e.build_response(),
    };

    context.insert("tables", &tables);

    let tera = &data.tera();
    let html = tera
        .render("content.html", &context)
        .unwrap_or_else(|e| tera.template_error(e));

    HttpResponse::Ok().body(html)
}

pub async fn content_add(
    data: web::Data<AppState>,
    form: Option<web::Form<HashMap<String, String>>>,
    req: HttpRequest,
) -> impl Responder {
    let db = data.database();
    let mut context = Extensions::unwrap_context(&req);

    let mut table = Table::new();
    if req.method() == "POST" {
        let Some(form) = form else {
            return HttpResponse::BadRequest().body("Form data is missing");
        };

        dbg!(&form);
        table = Table::from_form(&form);
        dbg!(&table);
    }

    context.insert("next_id", &table.fields.len());
    context.insert("table", &table);
    context.insert("field_types", &FieldType::as_list());

    match db.find_resource_by_type(ResourceType::SystemTable).await {
        Ok(tables) => context.insert("tables", &tables),
        Err(e) => return e.build_response(),
    };

    let tera = &data.tera();
    let html = tera
        .render("content_add.html", &context)
        .unwrap_or_else(|e| tera.template_error(e));

    HttpResponse::Ok().body(html)
}

pub async fn get_field(
    data: web::Data<AppState>,
    id: web::Path<u32>,
    req: HttpRequest,
) -> impl Responder {
    let mut context = Extensions::unwrap_context(&req);

    let id = id.into_inner();
    context.insert("include_next", &true);
    context.insert("field_types", &FieldType::as_list());
    context.insert("next_id", &(id + 1));
    context.insert("field", &TableField::new(id));

    let tera = &data.tera();
    let html = tera
        .render("components/field.html", &context)
        .unwrap_or_else(|e| tera.template_error(e));

    HttpResponse::Ok().body(html)
}
