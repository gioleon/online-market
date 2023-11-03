use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{find_category_by_id, save_category, get_all_categories},
    AppState,
};

pub fn build_router(state: Arc<AppState>) -> Router {
    let router = Router::new()
        .route("/category/", post(save_category))
        .route("/category/:id", get(find_category_by_id))
        .route("/category/all", get(get_all_categories))
        .with_state(state);

    router
}
