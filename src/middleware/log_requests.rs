use axum::{
    body::{self, Body, Bytes},
    http::Request,
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tracing::{debug, info};

pub async fn log_requests(req: Request<Body>, next: Next) -> Response {
    const MAX_BODY_SIZE: usize = 1 * 1024 * 1024; // 1MB
    const SENSITIVE_KEYS: &[&str] = &["password", "api_key", "token"];
    const TRUNCATE_LIMIT: usize = 500;
    const USER_AGENT_HEADER: &str = "User-Agent";
    const UNKNOWN_USER_AGENT: &str = "[Unknown]";

    // Extract and clone the request body for logging
    let (parts, body) = req.into_parts();
    let parts_clone = parts.clone();
    let user_agent = parts_clone
        .headers
        .get(USER_AGENT_HEADER)
        .map(|ua| ua.to_str().unwrap_or(UNKNOWN_USER_AGENT));
    let body_bytes = body::to_bytes(body, MAX_BODY_SIZE)
        .await
        .unwrap_or(Bytes::new());
    let req_body = redact_sensitive_info(&String::from_utf8_lossy(&body_bytes), SENSITIVE_KEYS);

    // Reconstruct the request
    let req = Request::from_parts(parts, Body::from(body_bytes.clone()));

    let method = req.method().clone();
    let uri = req.uri().clone();

    // Start timing the request
    let start = Instant::now();
    let response = next.run(req).await;
    let duration = start.elapsed();

    // Take ownership of the response body
    let (parts, body) = response.into_parts();
    let resp_body_bytes = body::to_bytes(body, MAX_BODY_SIZE)
        .await
        .unwrap_or_else(|_| Bytes::new());
    let resp_body =
        redact_sensitive_info(&String::from_utf8_lossy(&resp_body_bytes), SENSITIVE_KEYS);

    debug!(
        "Request: {} {} | User-Agent: {:?} | Body: {}\nResponse: {} | Body: {}\nTime: {:.2?}",
        method,
        uri,
        user_agent.unwrap_or("[Unknown]"),
        truncate(&req_body, TRUNCATE_LIMIT),
        parts.status,
        truncate(&resp_body, TRUNCATE_LIMIT),
        duration
    );

    info!(
        "Request {} {} | User-Agent: {:?} | Status: {} | Time: {:.2?}",
        method,
        uri,
        user_agent.unwrap_or("[Unknown]"),
        parts.status,
        duration
    );

    Response::from_parts(parts, Body::from(resp_body_bytes))
}

fn redact_sensitive_info(payload: &str, sensitive_keys: &[&str]) -> String {
    // Parse the payload as JSON
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(mut json) => {
            redact_json(&mut json, sensitive_keys); // Redact keys in-place
            json.to_string() // return redacted JSON as a string
        }
        Err(_) => payload.to_string(), // If parsing fails, return the original payload
    }
}

/// Recursively redact sensitive keys in JSON
fn redact_json(value: &mut serde_json::Value, sensitive_keys: &[&str]) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, val) in map.iter_mut() {
                if sensitive_keys.contains(&key.as_str()) {
                    *val = serde_json::Value::String("[REDACTED]".to_string());
                } else {
                    redact_json(val, sensitive_keys); // Recurse for nested objects
                }
            }
        }
        serde_json::Value::Array(array) => {
            for item in array {
                redact_json(item, sensitive_keys); // Recurse for array elements
            }
        }
        _ => {}
    }
}

fn truncate(payload: &str, limit: usize) -> String {
    if payload.len() > limit {
        format!("{}...", &payload[..limit])
    } else {
        payload.to_string()
    }
}
