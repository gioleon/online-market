use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_macros::debug_handler;
use std::sync::Arc;

use online_market_model::{Category, CategoryResponse};
use serde::Serialize;
use serde_json;

use crate::AppState;

#[debug_handler]
pub async fn save_category(
    State(app): State<Arc<AppState>>,
    Json(category): Json<Category>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = app.category_repository.save(category, &app.db).await;

    match result {
        Ok(category) => {
            let response = build_success_response(category);

            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(error) => {
            let response = build_error_response(Box::new(error));
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response)))
        }
    }
}

pub async fn find_category_by_id(
    Path(id): Path<i64>,
    State(app): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = app.category_repository.get_by_id(id, &app.db).await;

    match result {
        Ok(category) => {
            let response = build_success_response(category);

            Ok((StatusCode::OK, Json(response)))
        }
        Err(error) => match error {
            sqlx::Error::RowNotFound => {
                let result = build_success_multi_response(Vec::<CategoryResponse>::new());

                Ok((StatusCode::NOT_FOUND, Json(result)))
            }
            _ => {
                let result = build_error_response(Box::new(error));

                Err((StatusCode::INTERNAL_SERVER_ERROR, Json(result)))
            }
        },
    }
}

pub async fn get_all_categories(
    State(app): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = app.category_repository.get_all(&app.db).await;

    match result {
        Ok(categories) => {
            let response = build_success_multi_response(categories);

            Ok((StatusCode::OK, Json(response)))
        }
        Err(error) => match error {
            sqlx::Error::RowNotFound => {
                let result = build_success_multi_response(Vec::<CategoryResponse>::new());

                Ok((StatusCode::NOT_FOUND, Json(result)))
            }
            _ => {
                let result = build_error_response(Box::new(error));

                Err((StatusCode::INTERNAL_SERVER_ERROR, Json(result)))
            }
        },
    }
}

/// Returns a Json with status keys and payload for successful operations
///
/// # Argument
///
/// * payload - Object will be the value of the payload key. It must implements trait Serialize
///
fn build_success_response<T>(payload: T) -> serde_json::Value
where
    T: Serialize,
{
    serde_json::json!({
        "status": "success",
        "result": payload
    })
}

/// Returns a Json with status keys and payload for successful operations
///
/// # Argument
///
/// * payload - List of objects that will be the value of the payload key. Objects must implements trait Serialize
///
fn build_success_multi_response<T>(payload: Vec<T>) -> serde_json::Value
where
    T: Serialize,
{
    serde_json::json!({
        "status": "success",
        "result": payload
    })
}

/// Returns a Json with status keys and payload for failed operations
///
/// # Argument
///
/// * payload - Object will be the value of the payload key. It must implements trait Serialize
///
fn build_error_response(error: Box<dyn std::error::Error>) -> serde_json::Value {
    serde_json::json!({
        "status": "fail",
        "result": format!("{}", error)
    })
}
