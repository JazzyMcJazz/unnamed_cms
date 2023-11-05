use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::{prelude::*, service, utils::LoginForm};

pub async fn login_page(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let tera = data.tera();
    let (claims, context) = Extensions::unwrap_claims_and_context(&req);

    if claims.is_authenticated {
        return HttpResponse::Found()
            .append_header(("Location", data.prefix()))
            .append_header(("HX-Redirect", data.prefix()))
            .finish();
    }

    let html = tera
        .render("login.html", &context)
        .unwrap_or_else(|e| tera.template_error(e));

    HttpResponse::Ok().body(html)
}

pub async fn login(
    data: web::Data<AppState>,
    form: web::Form<LoginForm>,
    req: HttpRequest,
) -> impl Responder {
    let claims = Extensions::unwrap_claims(&req);

    if claims.is_authenticated {
        return CmsError::SeeOther(data.prefix()).build_response();
    }

    let auth_cookie = match service::login(data.database(), data.prefix(), form.into_inner()).await
    {
        Ok(cookie) => cookie,
        Err(e) => {
            return e.build_response();
        }
    };

    HttpResponse::Ok()
        .append_header(("Location", data.prefix())) // TODO: redirect to next
        .append_header(("HX-Redirect", data.prefix())) // TODO: redirect to next
        .cookie(auth_cookie)
        .finish()
}

pub async fn logout(data: web::Data<AppState>) -> impl Responder {
    let auth_cookie = service::logout(data.prefix());

    HttpResponse::Ok()
        .append_header(("Location", data.prefix()))
        .append_header(("HX-Redirect", data.prefix()))
        .cookie(auth_cookie)
        .finish()
}
