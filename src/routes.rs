use crate::handlers::{get_all_tokens_info, get_tokens, health_check};
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(health_check).post(health_check))
        .route("/tokens/:wallet", get(get_tokens))
        .route("/tokens_info", get(get_all_tokens_info))
}
