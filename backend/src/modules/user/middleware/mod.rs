use actix_session::SessionExt;
use std::future::{ready, Ready};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::{future::LocalBoxFuture, FutureExt, TryFutureExt};

use crate::modules::user::LoginStatusError;

#[derive(Clone, Copy, strum_macros::Display, PartialEq)]
pub enum LoginStatus {
    LoggedIn,
    LoggedOut,
}

#[derive(Clone, Copy)]
pub struct LoginStatusChecker(LoginStatus);

impl LoginStatusChecker {
    pub fn new(status: LoginStatus) -> Self {
        Self(status)
    }
}

impl<S, B> Transform<S, ServiceRequest> for LoginStatusChecker
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = LoginStatusCheckerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoginStatusCheckerMiddleware {
            service,
            status: self.0,
        }))
    }
}

pub struct LoginStatusCheckerMiddleware<S> {
    service: S,
    status: LoginStatus,
}

impl<S, B> Service<ServiceRequest> for LoginStatusCheckerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = &req.get_session();
        let id: Option<i32> = session.get("user_id").unwrap_or(None);
        let status: LoginStatus = match id {
            Some(_) => LoginStatus::LoggedIn,
            None => LoginStatus::LoggedOut,
        };

        tracing::debug!(
            user_id = id,
            login_status = status.to_string(),
            "Checking session ID"
        );

        if self.status != status {
            return Box::pin(async move {
                Ok(req
                    .error_response(LoginStatusError::from(status))
                    .map_into_right_body())
            });
        }

        self.service
            .call(req)
            .map_ok(ServiceResponse::map_into_left_body)
            .boxed_local()
    }
}
