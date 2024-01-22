use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::database::models::Joke;
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, DeleteJokeError> {
    Joke::delete(&state.database(), id).await?;

    Ok(StatusCode::OK)
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteJokeError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

impl IntoResponse for DeleteJokeError {
    fn into_response(self) -> Response {
        let body = format!("{}", self);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
