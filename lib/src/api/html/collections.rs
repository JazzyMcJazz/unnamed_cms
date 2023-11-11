use serde::{Deserialize, Serialize};

use crate::{config::database::ResourceType, prelude::*, service::CollectionService};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollectionForm {
    pub display_name: String,
    pub description: String,
}

impl CollectionForm {
    pub fn new() -> Self {
        Self {
            display_name: String::new(),
            description: String::new(),
        }
    }

    pub fn from(form: &CollectionForm) -> Self {
        Self {
            display_name: form.display_name.clone(),
            description: form.description.clone(),
        }
    }
}

pub async fn collections_index(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let db = data.database();
    let mut context = Extensions::unwrap_context(&req);

    let tables = match db.find_resource_by_type(ResourceType::Collection).await {
        Ok(tables) => tables,
        Err(e) => return e.build_response(),
    };

    context.insert("tables", &tables);

    let tera = &data.tera();
    let html = tera
        .render("collections.html", &context)
        .unwrap_or_else(|e| tera.template_error(e));

    HttpResponse::Ok().body(html)
}

pub async fn collections_add(
    data: web::Data<AppState>,
    form: Option<web::Form<CollectionForm>>,
    req: HttpRequest,
) -> impl Responder {
    let db = data.database();
    let mut context = Extensions::unwrap_context(&req);

    let mut collection = CollectionForm::new();
    if req.method() == "POST" {
        let Some(form) = &form else {
            return HttpResponse::BadRequest().body("Form data is missing");
        };

        collection = CollectionForm::from(form);
        return match CollectionService::create(db, &collection).await {
            Ok(e) => e.build_response(),
            Err(e) => e.build_response(),
        };
    }

    context.insert("collection", &collection);

    match db.find_resource_by_type(ResourceType::Collection).await {
        Ok(tables) => context.insert("tables", &tables),
        Err(e) => return e.build_response(),
    };

    let tera = &data.tera();
    let html = tera
        .render("collections_+.html", &context)
        .unwrap_or_else(|e| tera.template_error(e));

    HttpResponse::Ok().body(html)
}

// pub async fn get_field(
//     data: web::Data<AppState>,
//     id: web::Path<u32>,
//     req: HttpRequest,
// ) -> impl Responder {
//     let mut context = Extensions::unwrap_context(&req);

//     let id = id.into_inner();
//     context.insert("include_next", &true);
//     context.insert("field_types", &FieldType::as_list());
//     context.insert("next_id", &(id + 1));
//     context.insert("field", &TableField::new(id));

//     let tera = &data.tera();
//     let html = tera
//         .render("components/field.html", &context)
//         .unwrap_or_else(|e| tera.template_error(e));

//     HttpResponse::Ok().body(html)
// }
