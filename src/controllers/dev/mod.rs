use crate::types::AppState;

mod controller;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().route("/ping", axum::routing::get(controller::ping))
}
