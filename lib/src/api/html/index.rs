use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::prelude::*;

pub async fn index(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let tera = data.tera();
    let context = Extensions::unwrap_context(&req);

    let html = tera
        .render("index.html", &context)
        .unwrap_or_else(|e| tera.template_error(e));

    HttpResponse::Ok().body(html)
}
