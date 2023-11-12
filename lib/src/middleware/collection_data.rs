use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;

use crate::{config::database::ResourceType, prelude::*};

pub struct AddCollectionData;

impl<S: 'static, B> Transform<S, ServiceRequest> for AddCollectionData
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AddCollectionDataMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AddCollectionDataMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AddCollectionDataMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AddCollectionDataMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        let state = req.app_data::<web::Data<AppState>>().unwrap().clone();

        let mut context = Extensions::unwrap_context(&req);

        Box::pin(async move {
            let db = state.database();
            let tables = match db
                .find_resource_by_type(ResourceType::SystemCollection)
                .await
            {
                Ok(tables) => tables,
                Err(_) => vec![],
            };

            context.insert("collections", &tables);
            req.extensions_mut().insert(context);

            let res = svc.call(req).await?;
            Ok(res)
        })
    }
}
