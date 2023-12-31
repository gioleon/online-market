use dotenv::dotenv;
use online_market_data::{CategoryRepository, RateRepository, UserRepository, CommentRepository};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{env, sync::Arc};

pub mod handler;
pub mod router;
pub mod swagger;

pub struct AppState {
    pub db: PgPool,
    pub category_repository: CategoryRepository,
    pub user_repository: UserRepository,
    pub rate_repository: RateRepository,
    pub comment_repository: CommentRepository,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();

    // Create database connection
    let pool = match PgPoolOptions::new().connect(&database_url).await {
        Ok(pool) => {
            println!("Database connection successfully established");
            pool
        }
        Err(error) => {
            println!(
                "Something went wrong while creating database connection. {}",
                error
            );

            std::process::exit(1);
        }
    };

    // Creating AppState that will be used in the whole app
    let app_state = Arc::new(AppState {
        db: pool,
        category_repository: CategoryRepository::new(),
        user_repository: UserRepository::new(),
        rate_repository: RateRepository::new(),
        comment_repository: CommentRepository::new(),
    });

    // Create router and passing the AppState that will be use in the whole app
    let router = router::build_router(app_state);

    // Start server
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
