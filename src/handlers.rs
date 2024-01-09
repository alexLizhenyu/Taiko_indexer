use axum::{
    extract::{Path, Query},
    response::Response,
    Json,
};
use http::StatusCode;
use serde_json::{json, Value};

use crate::models::{Deployment, Mint};

pub async fn health_check() -> Response<String> {
    Response::new(String::from("ok"))
}

pub async fn get_tokens(Path(wallet): Path<String>) -> (StatusCode, Json<Value>) {
    let result = match Mint::owner(&wallet.to_lowercase()) {
        Ok(data) => data,
        Err(e) => {
            println!("🚨 failed to find mints: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(Value::Null)));
        }
    };

    (StatusCode::OK, Json(json!(result)))
}

pub async fn get_all_tokens_info() -> (StatusCode, Json<Value>) {
    let result = match Deployment::all() {
        Ok(data) => data,
        Err(e) => {
            println!("🚨 failed to find mints: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(Value::Null)));
        }
    };

    (StatusCode::OK, Json(json!(result)))
}
