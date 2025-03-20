use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::{database::BuildsRedisDatabase, traits::BuildsRepository};

pub async fn get_build(db: State<Arc<BuildsRedisDatabase>>, id: String) -> Result<String, AppError> {
    let build = db.0.get_build(id).await?;
    Ok(build)
}

pub async fn create_build(db: State<Arc<BuildsRedisDatabase>>, build: String) -> Result<String, AppError> {
    let id = db.0.insert_build(build).await?;
    Ok(id)
}


pub struct AppError(anyhow::Error);

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to process your request: {}", self.0),
        )
            .into_response()
    }
}
