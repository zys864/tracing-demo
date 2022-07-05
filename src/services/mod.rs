use axum::{
    body::Bytes,
    handler::Handler,
    http::{HeaderMap, Request, StatusCode},
    response::{IntoResponse, Response},
    Extension, Router,
};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tower_http::{request_id::MakeRequestUuid, ServiceBuilderExt};

use tracing::{Span, Level};

use crate::db::db_pool;

use self::router::short_links;

pub mod short_link_service;
pub mod user;
pub mod router;


pub async fn router() -> Router {
    // *********************************************************************
    // trace config
    let trace = TraceLayer::new_for_http()
    .make_span_with(|request: &Request<_>| {
        tracing::span!(Level::INFO,"http-service",request_id = ?request.headers().get("x-request-id"))
    })
    .on_request(|request: &Request<_>, _span: &Span| {
        tracing::info!(message = "request started",
        request.method = %request.method(), 
        request.headers = ?request.headers(),
        request.uri.path = %request.uri().path())
    })
    .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
        tracing::info!(message = "response generated", 
        status = %response.status(),
        ?latency)
    })
    .on_body_chunk(|chunk: &Bytes, _latency: Duration, _span: &Span| {
        tracing::info!(message = "sending bytes:", len = chunk.len())
    })
    .on_eos(|_trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span| {
        tracing::info!(message = "stream closed after ", duation = ?stream_duration)
    })
    .on_failure(|error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
        tracing::info!(message = "InternalServerError",error_message = %error)
    });
    // *********************************************************************
    // middleware
    let middleware = ServiceBuilder::new()
        // .buffer(1000)
        .concurrency_limit(100)
        .compression()
        .set_x_request_id(MakeRequestUuid)
        .layer(trace);
    // *********************************************************************
    // db connection pool
    let pool = db_pool().await.expect("connect to db pool failed");
    // *********************************************************************
    // router
    Router::new()
        .nest("/shortlink", short_links())
        .fallback(handler_404.into_service())
        .layer(middleware)
        .layer(Extension(pool))
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 not found")
}
