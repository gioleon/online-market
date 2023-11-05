use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

use online_market_model::Category;
use serde_json;

use crate::AppState;

use super::{build_error_response, build_success_multi_response, build_success_response};


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
                let result = build_error_response(Box::new(error));

                Err((StatusCode::NOT_FOUND, Json(result)))
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
                let result = build_error_response(Box::new(error));

                Err((StatusCode::NOT_FOUND, Json(result)))
            }
            _ => {
                let result = build_error_response(Box::new(error));

                Err((StatusCode::INTERNAL_SERVER_ERROR, Json(result)))
            }
        },
    }
}
