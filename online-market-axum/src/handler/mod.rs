use serde::Serialize;


pub mod category_handler;
pub mod user_handler;


/// Returns a Json with status keys and payload for successful operations
///
/// # Argument
///
/// * payload - Object will be the value of the payload key. It must implements trait Serialize
///
pub fn build_success_response<T>(payload: T) -> serde_json::Value
where
    T: Serialize,
{
    serde_json::json!({
        "status": "success",
        "result": payload
    })
}

/// Returns a Json with status keys and payload for successful operations
///
/// # Argument
///
/// * payload - List of objects that will be the value of the payload key. Objects must implements trait Serialize
///
pub fn build_success_multi_response<T>(payload: Vec<T>) -> serde_json::Value
where
    T: Serialize,
{
    serde_json::json!({
        "status": "success",
        "result": payload
    })
}

/// Returns a Json with status keys and payload for failed operations
///
/// # Argument
///
/// * payload - Object will be the value of the payload key. It must implements trait Serialize
///
pub fn build_error_response(error: Box<dyn std::error::Error>) -> serde_json::Value {
    serde_json::json!({
        "status": "fail",
        "result": format!("{}", error)
    })
}