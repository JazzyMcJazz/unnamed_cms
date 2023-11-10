use crate::{config::database::ResourceType, prelude::*};

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
