use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use online_market_data::{Pagination, PaginationRequest};

use std::sync::Arc;

use serde_json;

use online_market_model::Rate;

use crate::AppState;

use super::{build_error_response, build_success_response};

#[utoipa::path(
    post,
    path="/rate",
    responses(
        (status=201, description = "Rate saved"),
        (status=404, description = "Not found"),
        (status=500, description = "Internal error")
    )
)]
pub async fn save_rate(
    State(app): State<Arc<AppState>>,
    Json(rate): Json<Rate>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = app.rate_repository.save(rate, &app.db).await;

    match result {
        Ok(rate) => {
            let response = build_success_response(rate);

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
    path="/rate/{id_rater}/{id_rated}",
    responses(
        (status=200, description = "Get rates by rated"),
        (status=404, description = "Not found"),
        (status=500, description = "Internal error")
    )
)]
pub async fn get_rate(
    State(app): State<Arc<AppState>>,
    Path(id_rater): Path<String>,
    Path(id_rated): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = app
        .rate_repository
        .get_rate(id_rater, id_rated, &app.db)
        .await;

    match result {
        Ok(rate) => {
            let response = build_success_response(rate);

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
    path="/rate/rater/{id_rater}",
    params(
        online_market_data::PaginationRequest
    ),
    responses(
        (status=200, description = "Get rates by rater"),
        (status=404, description = "Not found"),
        (status=500, description = "Internal error")
    )
)]
pub async fn get_rates_by_rater(
    State(app): State<Arc<AppState>>,
    Path(rater_id): Path<String>,
    Query(pagination): Query<PaginationRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Creation of pagination
    // If no per_page or page is provided the default values will be used
    let pagination = Pagination::new(pagination);

    let result = app
        .rate_repository
        .get_rates_by_rater(rater_id, pagination, &app.db)
        .await;

    match result {
        Ok(rates) => {
            let response = build_success_response(rates);

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
    path="/rate/rated/{id_rated}",
    params(
        online_market_data::PaginationRequest
    ),
    responses(
        (status=200, description = "Get rates by rated"),
        (status=404, description = "Not found"),
        (status=500, description = "Internal error")
    )
)]
pub async fn get_rates_by_rated(
    State(app): State<Arc<AppState>>,
    Path(rated_id): Path<String>,
    Query(pagination): Query<PaginationRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Create pagination
    // If no per_page or page is provided the default values will be used
    let pagination = Pagination::new(pagination);

    let result = app
        .rate_repository
        .get_rates_by_rated(rated_id, pagination, &app.db)
        .await;

    match result {
        Ok(rates) => {
            let response = build_success_response(rates);

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
    path="/rate/update",
    responses(
        (status=200, description = "Update rate"),
        (status=404, description = "Not found"),
        (status=500, description = "Internal error")
    )
)]
pub async fn update_rate(
    State(app): State<Arc<AppState>>,
    Json(rate): Json<Rate>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = app
        .rate_repository
        .update_rate(rate, &app.db)
        .await;

    match result {
        Ok(rate) => {
            let response = build_success_response(rate);

            Ok((StatusCode::OK, Json(response)))
        }
        Err(error) => {
            let response = build_error_response(Box::new(error));
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
    }
}