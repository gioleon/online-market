use std::error::Error;

use online_market_model::{
    Category, CategoryResponse, Comment, CommentResponse, Modality, Rate, RateResponse, Roles,
    Service, ServiceResponse, User, UserResponse, UserLocation,
};
use serde::Deserialize;
use sqlx::PgPool;
use utoipa::IntoParams;
use uuid::Uuid;

use errors::NoIdProvided;

mod errors;

#[derive(Deserialize, IntoParams)]
pub struct PaginationRequest {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug)]
pub struct Pagination {
    pub page: i64,
    pub per_page: i64,
}

impl Pagination {
    pub fn new(pagination_request: PaginationRequest) -> Self {
        Pagination {
            page: pagination_request.page.unwrap_or(1),
            per_page: pagination_request.per_page.unwrap_or(25),
        }
    }
}

pub struct CategoryRepository {}

impl CategoryRepository {
    pub fn new() -> Self {
        CategoryRepository {}
    }

    pub async fn save(
        &self,
        category: Category,
        conn: &PgPool,
    ) -> Result<CategoryResponse, sqlx::Error> {
        // saving it to the database
        let category = sqlx::query_as!(
            CategoryResponse,
            r#"INSERT INTO categories (name) VALUES ($1)
            RETURNING id, name"#,
            category.name
        )
        .fetch_one(conn)
        .await?;

        Ok(category)
    }

    pub async fn get_by_id(
        &self,
        category_id: i64,
        conn: &PgPool,
    ) -> Result<CategoryResponse, sqlx::Error> {
        // saving it to the database
        let category = sqlx::query_as!(
            CategoryResponse,
            r#"SELECT * FROM categories WHERE id = $1"#,
            category_id as i64
        )
        .fetch_optional(conn)
        .await?;

        match category {
            Some(category) => Ok(category),
            None => Err(sqlx::Error::RowNotFound),
        }
    }

    pub async fn get_all(
        &self,
        pagination: Pagination,
        conn: &PgPool,
    ) -> Result<Vec<CategoryResponse>, sqlx::Error> {
        // saving it to the database
        let categories: Vec<CategoryResponse> = sqlx::query_as!(
            CategoryResponse,
            r#"SELECT * FROM categories LIMIT $1 OFFSET $2"#,
            pagination.per_page as i64,
            (pagination.page - 1) * pagination.per_page as i64
        )
        .fetch_all(conn)
        .await?;

        if categories.len() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(categories)
    }
}

pub struct UserRepository {}

impl UserRepository {
    pub fn new() -> Self {
        UserRepository {}
    }

    pub async fn save(&self, user: User, conn: &PgPool) -> Result<UserResponse, sqlx::Error> {
        let user = sqlx::query_as!(
            UserResponse,
            r#"INSERT INTO users (dni, email, password, name, date_of_birth, registered_at, contact_number, rol)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, dni, email, password, name, date_of_birth, registered_at, is_seller, updated_at, latitude, longitude, contact_number, category_id, rol as "rol: Roles"
            "#,
            user.dni as String,
            user.email as String,
            user.password as String,
            user.name as String,
            user.date_of_birth,
            chrono::Utc::now(),
            user.contact_number as String,
            Roles::User as Roles
        )
        .fetch_one(conn)
        .await?;

        Ok(user)
    }

    pub async fn get_by_dni(
        &self,
        dni: String,
        conn: &PgPool,
    ) -> Result<UserResponse, sqlx::Error> {
        let user = sqlx::query_as!(
            UserResponse,
            r#"SELECT id, dni, email, password, name, date_of_birth, registered_at, is_seller, updated_at, latitude, longitude, contact_number, category_id, rol as "rol: Roles" FROM users WHERE dni = $1"#,
            dni.to_string()
        ).fetch_optional(conn)
        .await?;

        match user {
            Some(user) => Ok(user),
            None => Err(sqlx::Error::RowNotFound),
        }
    }

    pub async fn get_all(
        &self,
        pagination: Pagination,
        conn: &PgPool,
    ) -> Result<Vec<UserResponse>, sqlx::Error> {
        let user = sqlx::query_as!(
            UserResponse,
            r#"SELECT id, dni, email, password, name, date_of_birth, registered_at, is_seller, updated_at, latitude, longitude, contact_number, category_id, rol as "rol: Roles" FROM users LIMIT $1 OFFSET $2"#,
            pagination.per_page as i64,
            (pagination.page - 1) * pagination.per_page as i64
        ).fetch_all(conn)
        .await?;

        if user.len() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(user)
    }

    pub async fn update_user(
        &self,
        user: User,
        conn: &PgPool,
    ) -> Result<UserResponse, sqlx::Error> {
        let user = sqlx::query_as!(
            UserResponse,
            r#"
                UPDATE users
                SET 
                email = $1, 
                password = $2, 
                name = $3, 
                date_of_birth = $4,  
                updated_at = $5, 
                contact_number = $6
                WHERE dni = $7 
                RETURNING id, dni, email, password, name, date_of_birth, registered_at, is_seller, updated_at, latitude, longitude, contact_number, category_id, rol as "rol: Roles"
            "#,
            user.email as String,
            user.password as String,
            user.name as String,
            user.date_of_birth as chrono::NaiveDate,
            chrono::Utc::now() as chrono::DateTime<chrono::Utc>,
            user.contact_number as String,
            user.dni as String
        ).fetch_optional(conn)
        .await?;

        match user {
            Some(user) => Ok(user),
            None => Err(sqlx::Error::RowNotFound),
        }
    }

    pub async fn update_location(
        &self,
        user_location: UserLocation,
        conn: &PgPool
    ) -> Result<(), Box<dyn std::error::Error>> {
        
        let sql = r#"UPDATE users
        SET 
        latitude = $1, 
        longitude = $2
        WHERE dni = $3"#;

        sqlx::query(sql)
            .bind(user_location.latitude)
            .bind(user_location.longitude)
            .bind(user_location.dni)
            .execute(conn).await?;

        Ok(())
    }
}

pub struct ServiceRepository {}

impl ServiceRepository {
    pub fn new() -> Self {
        ServiceRepository {}
    }

    pub async fn save(
        &self,
        service: Service,
        conn: &PgPool,
    ) -> Result<ServiceResponse, sqlx::Error> {
        let service = sqlx::query_as!(
            ServiceResponse,
            r#"INSERT INTO services (user_id, category_id, price, description, modality) VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, category_id, price, description, modality as "modality: Modality"
            "#,
            service.user_id as String,
            service.category_id as i64,
            service.price as f32,
            service.description as String,
            service.modality as Modality
        ).fetch_one(conn)
        .await?;

        Ok(service)
    }

    pub async fn get_by_dni(
        &self,
        dni: String,
        conn: &PgPool,
    ) -> Result<ServiceResponse, sqlx::Error> {
        let service = sqlx::query_as!(
            ServiceResponse,
            r#"
            SELECT id, user_id, category_id, price, description, modality as "modality: Modality" FROM services WHERE user_id = $1 
            "#,
            dni as String
        ).fetch_one(conn)
        .await?;

        Ok(service)
    }

    pub async fn update_service(
        &self,
        service: Service,
        conn: &PgPool,
    ) -> Result<ServiceResponse, Box<dyn Error>> {
        match service.id {
            Some(id) => {
                let service = sqlx::query_as!(
                    ServiceResponse,
                    r#"UPDATE services
                    SET 
                    category_id = $1,
                    price = $2,
                    description = $3,
                    modality = $4
                    WHERE id = $5
                    RETURNING id, user_id, category_id, price, description, modality as "modality: Modality"
                    "#,
                    service.category_id as i64,
                    service.price as f64,
                    service.description as String,
                    service.modality as Modality,
                    id as Uuid
                ).fetch_one(conn)
                .await?;

                Ok(service)
            }
            None => Err(Box::new(NoIdProvided::new(
                "NO ID PROVIDED TO UPDATE THE SERVICE",
            ))),
        }
    }
}

pub struct RateRepository {}

impl RateRepository {
    pub fn new() -> Self {
        RateRepository {}
    }

    pub async fn save(&self, rate: Rate, conn: &PgPool) -> Result<RateResponse, sqlx::Error> {
        let rate = sqlx::query_as!(
            RateResponse,
            r#"INSERT INTO rates (rater, rated, rate, created_at)VALUES ($1, $2, $3, $4)
            RETURNING rater, rated, rate, created_at, updated_at"#,
            rate.rater,
            rate.rated,
            rate.rate,
            chrono::Utc::now()
        )
        .fetch_one(conn)
        .await?;

        Ok(rate)
    }

    pub async fn get_rate(
        &self,
        rater: String,
        rated: String,
        conn: &PgPool,
    ) -> Result<RateResponse, sqlx::Error> {
        let rates = sqlx::query_as!(
            RateResponse,
            r#"SELECT * FROM rates WHERE rated = $1 AND rater = $2"#,
            rated as String,
            rater as String
        )
        .fetch_optional(conn)
        .await?;

        match rates {
            Some(rates) => Ok(rates),
            None => Err(sqlx::Error::RowNotFound),
        }
    }

    pub async fn get_rates_by_rated(
        &self,
        rated: String,
        pagination: Pagination,
        conn: &PgPool,
    ) -> Result<Vec<RateResponse>, sqlx::Error> {
        let rates = sqlx::query_as!(
            RateResponse,
            r#"SELECT * FROM rates WHERE rated = $1
            LIMIT $2 OFFSET $3"#,
            rated as String,
            pagination.per_page as i64,
            (pagination.page - 1) * pagination.per_page as i64
        )
        .fetch_all(conn)
        .await?;

        Ok(rates)
    }

    pub async fn get_rates_by_rater(
        &self,
        rater: String,
        pagination: Pagination,
        conn: &PgPool,
    ) -> Result<Vec<RateResponse>, sqlx::Error> {
        let rates = sqlx::query_as!(
            RateResponse,
            r#"SELECT * FROM rates WHERE rater = $1 LIMIT $2 OFFSET $3"#,
            rater as String,
            pagination.per_page as i64,
            (pagination.page - 1) * pagination.per_page as i64
        )
        .fetch_all(conn)
        .await?;

        Ok(rates)
    }

    pub async fn update_rate(
        &self,
        rate: Rate,
        conn: &PgPool,
    ) -> Result<RateResponse, sqlx::Error> {
        let rate = sqlx::query_as!(
            RateResponse,
            r#"UPDATE rates
            SET
            rate = $1,
            updated_at = $2
            WHERE rater = $3
            AND rated = $4
            RETURNING rater, rated, rate, created_at, updated_at"#,
            rate.rate as f32,
            chrono::Utc::now(),
            rate.rater as String,
            rate.rated as String
        )
        .fetch_one(conn)
        .await?;

        Ok(rate)
    }
}

pub struct CommentRepository {}

impl CommentRepository {
    pub fn new() -> Self {
        CommentRepository {}
    }

    pub async fn save(
        &self,
        comment: Comment,
        conn: &PgPool,
    ) -> Result<CommentResponse, sqlx::Error> {
        let comment = sqlx::query_as!(
            CommentResponse,
            r#"INSERT INTO comments (commentator, commented, comment, created_at)
            VALUES ($1, $2, $3, $4) 
            RETURNING commentator, commented, comment, created_at, updated_at"#,
            comment.commentator as String,
            comment.commented as String,
            comment.comment as String,
            chrono::Utc::now()
        )
        .fetch_one(conn)
        .await?;

        Ok(comment)
    }

    pub async fn get_comment(
        &self,
        commentator: String,
        commented: String,
        conn: &PgPool,
    ) -> Result<CommentResponse, sqlx::Error> {
        let comment = sqlx::query_as!(
            CommentResponse,
            r#"SELECT * FROM comments WHERE commented = $1 AND commentator = $2"#,
            commentator as String,
            commented as String
        )
        .fetch_optional(conn)
        .await?;

        match comment {
            Some(comment) => Ok(comment),
            None => Err(sqlx::Error::RowNotFound),
        }
    }

    pub async fn get_comments_by_commented(
        &self,
        commented: String,
        pagination: Pagination,
        conn: &PgPool,
    ) -> Result<Vec<CommentResponse>, sqlx::Error> {
        let comments = sqlx::query_as!(
            CommentResponse,
            r#"SELECT * FROM comments WHERE commented = $1 LIMIT $2 OFFSET $3"#,
            commented as String,
            pagination.per_page as i64,
            (pagination.page - 1) * pagination.per_page as i64
        )
        .fetch_all(conn)
        .await?;

        Ok(comments)
    }

    pub async fn get_comments_by_commentator(
        &self,
        commentator: String,
        pagination: Pagination,
        conn: &PgPool,
    ) -> Result<Vec<CommentResponse>, sqlx::Error> {
        let comments = sqlx::query_as!(
            CommentResponse,
            r#"SELECT * FROM comments WHERE commentator = $1 LIMIT $2 OFFSET $3"#,
            commentator as String,
            pagination.per_page as i64,
            (pagination.page - 1) * pagination.per_page as i64
        )
        .fetch_all(conn)
        .await?;

        Ok(comments)
    }

    pub async fn update_comment(
        &self,
        comment: Comment,
        conn: &PgPool,
    ) -> Result<CommentResponse, sqlx::Error> {
        let comment = sqlx::query_as!(
            CommentResponse,
            r#"UPDATE comments
            SET
            comment = $1,
            updated_at = $2
            WHERE commentator = $3
            AND commented = $4
            RETURNING commentator, commented, comment, created_at, updated_at"#,
            comment.comment,
            chrono::Utc::now(),
            comment.commentator,
            comment.commented
        )
        .fetch_one(conn)
        .await?;

        Ok(comment)
    }
}
