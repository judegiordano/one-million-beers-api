use axum::extract::State;

use crate::types::AppState;

mod dev;
mod interactions;

pub fn routes(State(state): State<AppState>) -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/dev", dev::router())
        .nest("/interactions", interactions::router(State(state)))
}
