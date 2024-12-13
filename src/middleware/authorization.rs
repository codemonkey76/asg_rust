use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{app_state::SharedAppState, auth::jwt::decode_jwt};

pub async fn authorization(
    State(state): State<SharedAppState>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let auth_header = req.headers().get("Authorization");

    if let Some(header_value) = auth_header {
        if let Ok(header_str) = header_value.to_str() {
            if let Some(token) = header_str.strip_prefix("Bearer ") {
                // Validate the JWT token
                match decode_jwt(token, &state.app_key) {
                    Ok(claims) => {
                        // Attach claims (e.g. email) to request extensions
                        req.extensions_mut().insert(claims.sub.clone());
                        return next.run(req).await;
                    }
                    Err(_) => {
                        return Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .body("Invalid or expired token".into())
                            .unwrap();
                    }
                }
            }
        }
    }

    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body("Missing or invalid Authorization header".into())
        .unwrap()
}
