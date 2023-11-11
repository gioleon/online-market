use std::sync::Arc;

use axum::{
    routing::{get, patch, post},
    Router,
};

use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    handler::{
        category_handler::{get_all_categories, get_category_by_id, save_category},
        comment_handler::{
            get_comment, get_comments_by_commentator, get_comments_by_commented, save_comment,
            update_comment,
        },
        rate_handler::{get_rate, get_rates_by_rated, get_rates_by_rater, save_rate, update_rate},
        user_handler::{get_all_user, get_user_by_dni, save_user, update_user, handler_user_location},
    },
    AppState, swagger::ApiDoc,
};


pub fn build_router(state: Arc<AppState>) -> Router {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "online_market_axum=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router = Router::new()
        .route("/category", post(save_category))
        .route("/category/:id", get(get_category_by_id))
        .route("/category/all", get(get_all_categories))
        .route("/user", post(save_user))
        .route("/ws/user/update/location", get(handler_user_location))
        .route("/user/:dni", get(get_user_by_dni))
        .route("/user/all", get(get_all_user))
        .route("/user/update", patch(update_user))
        .route("/rate", post(save_rate))
        .route("/rate/rater/:id_rater", get(get_rates_by_rater))
        .route("/rate/rated/:id_rated", get(get_rates_by_rated))
        .route("/rate/:id_rater/:id_rated", get(get_rate))
        .route("/rate/update", patch(update_rate))
        .route("/comment", post(save_comment))
        .route(
            "/comment/commented/:id_commented",
            get(get_comments_by_commented),
        )
        .route(
            "/comment/commentator/:id_commentator",
            get(get_comments_by_commentator),
        )
        .route("/comment/update", patch(update_comment))
        .route("/comment/:id_commented/:id_commentator", get(get_comment))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(state)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    router
}
