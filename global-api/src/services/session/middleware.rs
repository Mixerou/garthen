use std::future::{Ready, ready};

use actix_web::{Error, HttpMessage};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header;
use actix_web::http::header::{HeaderName, HeaderValue};
use futures_util::future::LocalBoxFuture;

use crate::services::session::Session;

pub struct CheckSession;

impl<S, B> Transform<S, ServiceRequest> for CheckSession
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CheckSessionMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckSessionMiddleware { service }))
    }
}

pub struct CheckSessionMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckSessionMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let mut is_session_created = false;
        let session = match Session::find_by_token(
            request
                .headers()
                .get(header::AUTHORIZATION).unwrap_or(&HeaderValue::from_static(""))
                .to_str().unwrap()
                .to_string(),
        ) {
            Ok(session) => session,
            Err(error) => {
                match error.status_code {
                    404 => {
                        match Session::create() {
                            Ok(session) => {
                                is_session_created = true;
                                session
                            },
                            Err(error) => return Box::pin(async { Err(Error::from(error)) }),
                        }
                    },
                    _ => return Box::pin(async { Err(Error::from(error)) }),
                }
            },
        };

        request.extensions_mut().insert(session.clone());

        let response = self.service.call(request);

        Box::pin(async move {
            let mut response = response.await?;

            if is_session_created {
                response.headers_mut().insert(
                    HeaderName::from_static("x-set-session-token"),
                    HeaderValue::from_str(
                        session
                            .token
                            .as_str()
                    ).unwrap(),
                );
            }

            Ok(response)
        })
    }
}
