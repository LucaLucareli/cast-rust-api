use axum::{
    body::{to_bytes, Body},
    http::Response,
    middleware::Next,
    response::IntoResponse,
};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct HttpResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

pub async fn transform_middleware(req: axum::http::Request<Body>, next: Next) -> impl IntoResponse {
    let response = next.run(req).await;

    let status = response.status();
    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();

    let original_data: Value = match serde_json::from_slice(&body_bytes) {
        Ok(v) => v,
        Err(_) => Value::String(String::from_utf8_lossy(&body_bytes).to_string()),
    };

    let (message, data) = match &original_data {
        Value::Object(map) => {
            let message = map.get("message").cloned();
            let data = map.get("result").cloned().or(Some(original_data.clone()));
            (message, data)
        }
        _ => (None, Some(original_data.clone())),
    };

    let body = serde_json::to_string(&HttpResponse { message, data }).unwrap();

    Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(Body::from(body))
        .unwrap()
}
