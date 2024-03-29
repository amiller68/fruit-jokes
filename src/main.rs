use askama::Template;
use axum::{
    response::IntoResponse,
    routing::{delete, get},
    Router,
};
use sqlx::PgPool;

use tower_http::services::ServeDir;

mod api;
mod database;

#[derive(Clone)]
pub struct AppState {
    database: PgPool,
}

impl AppState {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }

    pub fn database(&self) -> PgPool {
        self.database.clone()
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:postgres@localhost:5432/postgres"
    )]
    db: PgPool,
) -> shuttle_axum::ShuttleAxum {
    // Run migrations
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Looks like something went wrong with migrations :(");
    // Setup State
    let state = AppState::new(db);

    // Register panics as they happen
    register_panic_logger();

    // Setup Router
    let router = Router::new()
        // Home page
        .route("/", get(index))
        .route(
            "/jokes",
            get(api::read_all_jokes::handler).post(api::create_joke::handler),
        )
        .route("/jokes/:joke_id", delete(api::delete_joke::handler))
        .with_state(state)
        // Static assets
        .nest_service("/static", ServeDir::new("static"));

    // Run!
    Ok(router.into())
}

async fn index() -> impl IntoResponse {
    IndexTemplate
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

/// Sets up system panics to use the tracing infrastructure to log reported issues. This doesn't
/// prevent the panic from taking out the service but ensures that it and any available information
/// is properly reported using the standard logging mechanism.
fn register_panic_logger() {
    std::panic::set_hook(Box::new(|panic| match panic.location() {
        Some(loc) => {
            tracing::error!(
                message = %panic,
                panic.file = loc.file(),
                panic.line = loc.line(),
                panic.column = loc.column(),
            );
        }
        None => tracing::error!(message = %panic),
    }));
}
