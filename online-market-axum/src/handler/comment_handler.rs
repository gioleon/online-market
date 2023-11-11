use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use online_market_data::{Pagination, PaginationRequest};

use std::sync::Arc;

use serde_json;

use online_market_model::Comment;

use crate::AppState;

use super::{build_error_response, build_success_response};

#[utoipa::path(
    post,
    path="/comment",
    responses(
        (status=201, description = "Comment created"),
        (status=404, description = "Not found"),
        (status=500, description = "Internal error")
    )
)]
pub async fn save_comment(
    State(app): State<Arc<AppState>>,
    Json(comment): Json<Comment>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = app.comment_repository.save(comment, &app.db).await;

    match result {
        Ok(comment) => {
            let response = build_success_response(comment);

            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(error) => {
            let response = build_error_response(Box::new(error));
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response)))
        }
    }
}

#[utoipa::path(
    get,
    path="/comment/{id_commented}/{id_commentator}",
    responses(
        (status=200, description = "Get comment"),
        (status=404, description = "Not found"),
        (status=500, description = "Internal error")
    )
)]
pub async fn get_comment(
    State(app): State<Arc<AppState>>,
    Path(id_commentator): Path<String>,
    Path(id_commented): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = app
        .comment_repository
        .get_comment(id_commentator, id_commented, &app.db)
        .await;

    match result {
        Ok(comment) => {
            let response = build_success_response(comment);

            Ok((StatusCode::OK, Json(response)))
        }
        Err(error) => {
            let response = build_error_response(Box::new(error));
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response)))
        }
    }
}

#[utoipa::path(
    get,
    path="/comment/commentator/{id_commentator}",
    params(
        online_market_data::PaginationRequest
    ),
    responses(
        (status=200, description = "Get comment"),
        (status=404, description = "Not found"),
        (status=500, description = "Internal error")
    )
)]
pub async fn get_comments_by_commentator(
    State(app): State<Arc<AppState>>,
    Path(id_commentator): Path<String>,
    Query(pagination): Query<PaginationRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Creation of pagination
    // If no per_page or page is provided the default values will be used
    let pagination = Pagination::new(pagination);

    let result = app
        .comment_repository
        .get_comments_by_commentator(id_commentator, pagination, &app.db)
        .await;

    match result {
        Ok(comments) => {
            let response = build_success_response(comments);

            Ok((StatusCode::OK, Json(response)))
        }
        Err(error) => match error {
            sqlx::Error::RowNotFound => {
                let response = build_error_response(Box::new(error));
                Err((StatusCode::NOT_FOUND, Json(response)))
            }
            _ => {
                let response = build_error_response(Box::new(error));
                Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response)))
            }
        },
    }
}

#[utoipa::path(
    get,
    path="/comment/commented/{id_commented}",
    params(
        online_market_data::PaginationRequest
    ),
    responses(
        (status=200, description = "Get comment"),
        (status=404, description = "Not found"),
        (status=500, description = "Internal error")
    )
)]
pub async fn get_comments_by_commented(
    State(app): State<Arc<AppState>>,
    Path(id_commented): Path<String>,
    Query(pagination): Query<PaginationRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Create pagination
    // If no per_page or page is provided the default values will be used
    let pagination = Pagination::new(pagination);

    let result = app
        .comment_repository
        .get_comments_by_commented(id_commented, pagination, &app.db)
        .await;

    match result {
        Ok(comments) => {
            let response = build_success_response(comments);

            Ok((StatusCode::OK, Json(response)))
        }
        Err(error) => match error {
            sqlx::Error::RowNotFound => {
                let response = build_error_response(Box::new(error));
                Err((StatusCode::NOT_FOUND, Json(response)))
            }
            _ => {
                let response = build_error_response(Box::new(error));
                Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response)))
            }
        },
    }
}

#[utoipa::path(
    patch,
    path="/comment/update",
    responses(
        (status=200, description = "Comment updated"),
        (status=404, description = "Not found"),
        (status=500, description = "Internal error")
    )
)]
pub async fn update_comment(
    State(app): State<Arc<AppState>>,
    Json(comment): Json<Comment>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = app
        .comment_repository
        .update_comment(comment, &app.db)
        .await;

    match result {
        Ok(comment) => {
            let response = build_success_response(comment);

            Ok((StatusCode::OK, Json(response)))
        }
        Err(error) => {
            let response = build_error_response(Box::new(error));
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
    }
}

