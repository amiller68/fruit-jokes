use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
};

use crate::database::models::Joke;
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ReadAllJokesError> {
    let jokes = Joke::read_all(&state.database()).await?;

    Ok(Records { jokes })
}

#[derive(Template)]
#[template(path = "jokes.html")]
struct Records {
    jokes: Vec<Joke>,
}

#[derive(Debug, thiserror::Error)]
pub enum ReadAllJokesError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

impl IntoResponse for ReadAllJokesError {
    fn into_response(self) -> Response {
        let body = format!("{}", self);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
