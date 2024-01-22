use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Form,
};

use crate::database::models::{Joke, NewJoke};
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
    Form(new_joke): Form<NewJoke>,
) -> Result<impl IntoResponse, CreateJokeError> {
    let joke = new_joke.create(&state.database()).await?;

    Ok(NewJokeTemplate { joke })
}

#[derive(Template)]
#[template(path = "joke.html")]
struct NewJokeTemplate {
    joke: Joke,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateJokeError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

impl IntoResponse for CreateJokeError {
    fn into_response(self) -> Response {
        let body = format!("{}", self);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
