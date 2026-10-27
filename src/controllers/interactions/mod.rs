use axum::{extract::State, middleware};

use crate::{middleware::verify_signature, types::AppState};

mod controller;

pub fn router(State(state): State<AppState>) -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", axum::routing::post(controller::handle_interaction))
        .route_layer(middleware::from_fn_with_state(state, verify_signature))
}
