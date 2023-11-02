use actix_web::{web, HttpResponse, Responder};
use tera::Context;

use crate::config::app_state::{AppState, TemplateError};

pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let tera = data.tera();

    let db = data.database();
    db.query("test").await.unwrap();

    let html = tera
        .render("index.html", &Context::new())
        .unwrap_or_else(|_| tera.template_error());

    HttpResponse::Ok().body(html)
}
