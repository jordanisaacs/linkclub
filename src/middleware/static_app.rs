use axum::{
    body::{boxed, Body, Bytes, HttpBody},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    BoxError,
};
use futures::future::BoxFuture;
use std::{
    convert::Infallible,
    path::Path,
    task::{Context, Poll},
};
use tower::{Layer, Service, ServiceExt};
use tower_http::services::ServeDir;
use tracing::Level;

/// [`Layer`] for adding file serving to a router
#[derive(Debug, Clone)]
pub struct StaticAppLayer {
    dir_path: String,
}

impl StaticAppLayer {
    /// Create a new [`StaticAppLayer`].
    pub fn new(dir_path: String) -> Self {
        Self { dir_path }
    }
}

impl<S> Layer<S> for StaticAppLayer {
    type Service = StaticApp<S>;

    fn layer(&self, inner: S) -> Self::Service {
        StaticApp {
            inner,
            dir_path: self.dir_path.clone(),
        }
    }
}

/// Middleware for adding static file service to a router
#[derive(Debug, Clone)]
pub struct StaticApp<S> {
    inner: S,
    dir_path: String,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for StaticApp<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send,
    ReqBody: Send + 'static,
    ResBody: HttpBody<Data = Bytes> + Send + 'static,
    ResBody::Error: Into<BoxError>,
{
    type Response = Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let file_req = Request::builder()
            .uri(req.uri())
            .body(Body::empty())
            .unwrap();

        let dir_path = self.dir_path.clone();
        let clone = self.inner.clone();
        let inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            match ServeDir::new(dir_path).oneshot(file_req).await {
                Ok(res) => {
                    if res.status() == StatusCode::NOT_FOUND {
                        let new_res = inner.oneshot(req).await?;
                        Ok(new_res.map(boxed))
                    } else {
                        Ok(res.map(boxed))
                    }
                }
                Err(_) => Ok((StatusCode::INTERNAL_SERVER_ERROR, ()).into_response()),
            }
        })
    }
}
