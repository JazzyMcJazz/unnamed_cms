use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::{prelude::*, service::AuthService, utils::LoginForm};

pub async fn login_page(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let tera = data.tera();
    let (claims, context) = Extensions::unwrap_claims_and_context(&req);

    if claims.is_authenticated {
        return HttpResponse::Found()
            .append_header(("Location", data.base_path_or_root()))
            .append_header(("HX-Redirect", data.base_path_or_root()))
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
        return CmsResponse::SeeOther(data.base_path().to_owned()).build_response();
    }

    let cookies = match AuthService::login(
        data.database(),
        data.base_path_or_root(),
        form.into_inner(),
    )
    .await
    {
        Ok(cookies) => cookies,
        Err(e) => {
            return e.build_response();
        }
    };

    HttpResponse::Ok()
        .append_header(("Location", data.base_path())) // TODO: redirect to next
        .append_header(("HX-Redirect", data.base_path())) // TODO: redirect to next
        .cookie(cookies.0)
        .cookie(cookies.1)
        .finish()
}

pub async fn logout(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let claims = Extensions::unwrap_claims(&req);
    let cookies = AuthService::logout(data.database(), claims, data.base_path_or_root()).await;

    HttpResponse::Ok()
        .append_header(("Location", data.base_path_or_root()))
        .append_header(("HX-Redirect", data.base_path_or_root()))
        .cookie(cookies.0)
        .cookie(cookies.1)
        .finish()
}
