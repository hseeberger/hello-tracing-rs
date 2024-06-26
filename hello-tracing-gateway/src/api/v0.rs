use crate::backend::Backend;
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use tracing::{error, instrument};

pub fn app(backend: Backend) -> Router {
    let app_state = AppState { backend };
    Router::new()
        .route("/hello", get(hello))
        .with_state(app_state)
}

#[derive(Debug, Clone)]
struct AppState {
    backend: Backend,
}

#[instrument(name = "hello-handler", skip(app_state))]
async fn hello(State(app_state): State<AppState>) -> impl IntoResponse {
    app_state.backend.hello().await.map_err(internal_error)
}

fn internal_error(error: anyhow::Error) -> StatusCode {
    error!(
        error = display(format!("{error:#}")),
        backtrace = %error.backtrace(),
        "internal error"
    );
    StatusCode::INTERNAL_SERVER_ERROR
}
