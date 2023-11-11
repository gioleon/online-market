use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use axum_typed_websockets::{Message, WebSocket, WebSocketUpgrade};

use futures::{SinkExt, StreamExt};
use online_market_data::{Pagination, PaginationRequest};
use std::sync::Arc;
use tokio::sync::mpsc;

use online_market_model::{User, UserLocation};
use serde_json;

use crate::AppState;

use super::{build_error_response, build_success_multi_response, build_success_response};

pub async fn handler_user_location(
    ws: WebSocketUpgrade<i16, UserLocation>,
    State(app): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| update_user_location_socket(socket, app))
}

pub async fn update_user_location_socket(socket: WebSocket<i16, UserLocation>, app: Arc<AppState>) {
    // create channel if time this function is called
    let (tx, mut rx) = mpsc::unbounded_channel::<UserLocation>();

    // split the new web socket connection in sender and receiver
    let (mut sender, mut receiver) = socket.split();

    tokio::spawn(async move {
        while let Some(user_location) = rx.recv().await {
            let result = app
                .user_repository
                .update_location(user_location, &app.db)
                .await;

            match result {
                Ok(_) => {
                    let _ = sender.send(Message::Item(200));
                }
                Err(_) => {
                    let _ = sender.send(Message::Item(500));
                }
            }
        }
    });

    while let Some(message) = receiver.next().await {
        match message {
            Ok(message) => match message {
                Message::Item(user_location) => {
                    tx.send(user_location).unwrap();
                }
                _ => {}
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

#[utoipa::path(
    post,
    path="/user",
    responses(
        (status=201, description = "User saved"),
        (status=500, description = "Internal error")
    )
)]
pub async fn save_user(
    State(app): State<Arc<AppState>>,
    Json(user): Json<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("{:?}", user);
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

#[utoipa::path(
    get,
    path="/user/{dni}",
    responses(
        (status=200, description = "Get user by id"),
        (status=404, description = "No user found")
    )
)]
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

#[utoipa::path(
    get,
    path="/user/all",
    params(
        online_market_data::PaginationRequest
    ),
    responses(
        (status=200, description = "Get all users"),
        (status=404, description = "No user found")
    )
)]
pub async fn get_all_user(
    State(app): State<Arc<AppState>>,
    Query(pagination): Query<PaginationRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Creation of pagination
    // If no per_page or page is provided the default values will be used
    let pagination = Pagination::new(pagination);

    let result = app.user_repository.get_all(pagination, &app.db).await;

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

#[utoipa::path(
    patch,
    path="/user/update",
    responses(
        (status=200, description = "User updated"),
        (status=404, description = "No user found")
    )
)]
pub async fn update_user(
    State(app): State<Arc<AppState>>,
    Json(user): Json<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = app.user_repository.update_user(user, &app.db).await;

    match result {
        Ok(user) => {
            let response = build_success_response(user);
            Ok((StatusCode::OK, Json(response)))
        }
        Err(error) => {
            let response = build_error_response(Box::new(error));
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
    }
}
