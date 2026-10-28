use one_million_beers_api::{
    errors::AppError,
    logger,
    models::{server::Server, user::User},
};
use tokio::try_join;

#[tokio::main]
pub async fn main() -> Result<(), AppError> {
    logger::init()?;
    let indexes =
        try_join!(User::migrate(), Server::migrate()).map_err(AppError::internal_server_error)?;
    tracing::info!("{:?}", indexes);
    Ok(())
}
