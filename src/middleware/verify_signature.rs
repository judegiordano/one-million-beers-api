use axum::{
    body::{to_bytes, Body},
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use ed25519_dalek::{Signature, Verifier};
use reqwest::StatusCode;

use crate::types::AppState;

const MAX_BODY_BYTES: usize = 1024 * 1024;

pub async fn verify_signature(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let (parts, body) = request.into_parts();

    let signature_hex = parts
        .headers
        .get("x-signature-ed25519")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let timestamp = parts
        .headers
        .get("x-signature-timestamp")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let signature_bytes =
        hex::decode(signature_hex.as_bytes()).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let signature =
        Signature::from_slice(&signature_bytes).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let bytes = to_bytes(body, MAX_BODY_BYTES)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut message = Vec::with_capacity(timestamp.len() + bytes.len());
    message.extend_from_slice(timestamp.as_bytes());
    message.extend_from_slice(&bytes);

    state
        .verification_key
        .verify(&message, &signature)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let request = Request::from_parts(parts, Body::from(bytes));
    Ok(next.run(request).await)
}
