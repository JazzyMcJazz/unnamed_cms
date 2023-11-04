use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::prelude::*;

pub async fn login(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let tera = data.tera();
    let context = Extensions::unwrap_context(&req);

    let html = tera
        .render("login.html", &context)
        .unwrap_or_else(|_| tera.template_error());

    HttpResponse::Ok().body(html)
}
