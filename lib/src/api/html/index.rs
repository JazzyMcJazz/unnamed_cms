use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::prelude::*;

pub async fn index(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let tera = data.tera();
    let context = Extensions::unwrap_context(&req);

    let db = data.database();
    db.query("test").await.unwrap();

    let html = tera
        .render("index.html", &context)
        .unwrap_or_else(|_| tera.template_error());

    HttpResponse::Ok().body(html)
}
