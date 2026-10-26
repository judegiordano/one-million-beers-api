use crate::types::AppState;

mod controller;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().route("/", axum::routing::post(controller::handle_interaction))
}
