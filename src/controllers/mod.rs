use crate::types::AppState;

mod dev;
mod interactions;

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/dev", dev::router())
        .nest("/interactions", interactions::router())
}
