use crate::errors::AppError;

pub type ApiResponse = Result<axum::response::Response, AppError>;

#[derive(Clone, Debug)]
pub struct AppState {}
