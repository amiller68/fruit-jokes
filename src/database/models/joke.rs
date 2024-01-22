use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::PgPool;

#[derive(Debug, FromRow, Deserialize)]
pub struct NewJoke {
    content: String,
}

// Note: don't be afraid to create a new type, if you do in fact need a new type!
//  Also don't be afraid to start thinking about Api types -- often times DB models will become
//   unwieldy just for populating html content!
impl NewJoke {
    // Database operations
    pub async fn create(&self, pool: &PgPool) -> Result<Joke, sqlx::Error> {
        let joke = sqlx::query_as!(
            Joke,
            "INSERT INTO jokes (content) VALUES ($1) RETURNING *",
            self.content
        )
        .fetch_one(pool)
        .await?;

        Ok(joke)
    }
}

#[derive(Debug, FromRow, Serialize)]
pub struct Joke {
    id: i32,
    content: String,
}

impl Joke {
    // Getters
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    // Database operations
    pub async fn read_all(pool: &PgPool) -> Result<Vec<Joke>, sqlx::Error> {
        let jokes = sqlx::query_as!(Joke, "SELECT * FROM jokes")
            .fetch_all(pool)
            .await?;

        Ok(jokes)
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM jokes WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
