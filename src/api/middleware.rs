use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::env;

pub async fn api_key_middleware(req: Request<axum::body::Body>, next: Next) -> Response {
    let expected_api_key = env::var("API_KEY").expect("API_KEY must be set");

    match req.headers().get("x-api-key") {
        Some(header_value) if header_value == expected_api_key.as_str() => next.run(req).await,
        _ => Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(axum::body::Body::from("Forbidden"))
            .unwrap(),
    }
}
