use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name="modality", rename_all="lowercase")]
pub enum Modality {
    Domicilio,
    Presencial,
    Hibrido
}

#[derive(sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name="roles", rename_all="lowercase")]
pub enum Roles {
    Admin,
    User
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lon: f64
}

#[derive(Serialize, Deserialize)]
pub struct LocationResponse {
    pub lat: f64,
    pub lon: f64
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct CategoryResponse {
    pub id: i64,
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct Rate {
    pub rater: String,
    pub rated: String,
    pub rate: f32,
}

#[derive(Serialize, Deserialize)]
pub struct RateResponse {
    pub rater: String,
    pub rated: String,
    pub rate: f32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub commentator: String,
    pub commented: String,
    pub comment: String
}

#[derive(Serialize, Deserialize)]
pub struct CommentResponse {
    pub commentator: String,
    pub commented: String,
    pub comment: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub dni: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub date_of_birth: chrono::NaiveDate,
    pub is_seller: bool,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub latitude: f32,
    pub longitude: f32,
    pub contact_number: String,
    pub category_id: Option<i64>,  
    pub rol: Roles
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub dni: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub date_of_birth: chrono::NaiveDate,
    pub registered_at: chrono::DateTime<chrono::Utc>,
    pub is_seller: bool,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub latitude: f32,
    pub longitude: f32,
    pub contact_number: String,
    pub category_id: Option<i64>,  
    pub rol: Roles
}


#[derive(Serialize, Deserialize)]
pub struct Service {
    pub id: Option<Uuid>,
    pub user_id: String,
    pub category_id: i64,
    pub price: f64,
    pub description: String,
    pub modality: Modality
}

#[derive(Serialize, Deserialize)]
pub struct ServiceResponse {
    pub id: Uuid,
    pub user_id: String,
    pub category_id: i64,
    pub price: f64,
    pub description: String,
    pub modality: Modality
}



