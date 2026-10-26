use one_million_beers_api::{
    controllers::routes,
    errors::AppError,
    logger,
    types::{ApiResponse, AppState},
};

#[tokio::main]
pub async fn main() -> Result<(), lambda_http::Error> {
    logger::init()?;
    let state = AppState {};
    let app = axum::Router::new()
        .merge(routes())
        .fallback(axum::routing::any(|| async {
            ApiResponse::Err(AppError::NotFound("route not found".to_string()))
        }))
        .with_state(state);
    if cfg!(debug_assertions) {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
        tracing::info!("listening on {:?}", listener.local_addr()?);
        return Ok(axum::serve(listener, app).await?);
    }
    lambda_http::run(app).await
}
