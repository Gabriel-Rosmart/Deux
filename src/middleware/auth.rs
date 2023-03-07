use axum::{
    response::{Response, IntoResponse},
    body::Body,
    http::{Request, StatusCode},
};
use futures::future::BoxFuture;
use tower::{Service, Layer};
use std::task::{Context, Poll};

#[derive(Clone)]
pub struct Auth;

impl<S> Layer<S> for Auth {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for AuthMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        println!("{:#?}", request.headers().get("Authorization"));
        let future = self.inner.call(request);
        Box::pin(async move {
            let response: Response = future.await?;
            //let response = StatusCode::FOUND.into_response();
            Ok(response)
        })
    }
}