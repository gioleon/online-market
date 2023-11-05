use std::sync::Arc;

use axum::{
    routing::{get, post, patch},
    Router,
};

use crate::{
    handler::{category_handler::{find_category_by_id, get_all_categories, save_category}, user_handler::{save_user, get_all_user, get_user_by_dni, update_user}},
    AppState,
};

pub fn build_router(state: Arc<AppState>) -> Router {
    let router = Router::new()
        .route("/category", post(save_category))
        .route("/category/:id", get(find_category_by_id))
        .route("/category/all", get(get_all_categories))
        .route("/user", post(save_user))
        .route("/user/:dni", get(get_user_by_dni))
        .route("/user/all", get(get_all_user))
        .route("/user/update/", patch(update_user))
        .with_state(state);

    router
}
