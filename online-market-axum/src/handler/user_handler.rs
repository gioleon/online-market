use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

use online_market_model::User;
use serde_json;

use crate::AppState;

use super::{build_error_response, build_success_multi_response, build_success_response};

pub async fn save_user(
    State(app): State<Arc<AppState>>,
    Json(user): Json<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = app.user_repository.save(user, &app.db).await;

    match result {
        Ok(user) => {
            let response = build_success_response(user);
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(error) => {
            let response = build_error_response(Box::new(error));
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response)))
        }
    }
}

pub async fn get_user_by_dni(
    Path(dni): Path<String>,
    State(app): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = app.user_repository.get_by_dni(dni, &app.db).await;

    match result {
        Ok(user) => {
            let response = build_success_response(user);

            Ok((StatusCode::OK, Json(response)))
        }
        Err(error) => match error {
            sqlx::Error::RowNotFound => {
                let error = build_error_response(Box::new(error));
                Err((StatusCode::NOT_FOUND, Json(error)))
            }
            _ => {
                let error = build_error_response(Box::new(error));
                Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)))
            }
        },
    }
}

pub async fn get_all_user(
    State(app): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = app.user_repository.get_all(&app.db).await;

    match result {
        Ok(user) => {
            let response = build_success_multi_response(user);
            println!("{}", response);
            Ok((StatusCode::OK, Json(response)))
        }
        Err(error) => match error {
            sqlx::Error::RowNotFound => {
                let error = build_error_response(Box::new(error));
                Err((StatusCode::NOT_FOUND, Json(error)))
            }
            _ => {
                let error = build_error_response(Box::new(error));
                Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)))
            }
        },
    }
}

pub async fn update_user(
    State(app): State<Arc<AppState>>,
    Json(user): Json<User>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result  = app.user_repository.update_user(user, &app.db).await;


    match result {
        Ok(user) => {
            let response = build_success_response(user);
            Ok((StatusCode::OK, Json(response)))
        },
        Err(error) => {
            let response = build_error_response(Box::new(error));
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
    }
}
