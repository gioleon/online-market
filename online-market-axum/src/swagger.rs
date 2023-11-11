use online_market_model::{User, Service, Modality, Roles, Comment, Rate, Category};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
       crate::handler::user_handler::get_all_user,
       crate::handler::user_handler::get_user_by_dni,
       crate::handler::user_handler::save_user,
       crate::handler::user_handler::update_user,
       crate::handler::category_handler::save_category,
       crate::handler::category_handler::get_all_categories,
       crate::handler::category_handler::get_category_by_id,
       crate::handler::rate_handler::save_rate,
       crate::handler::rate_handler::get_rate,
       crate::handler::rate_handler::get_rates_by_rated,
       crate::handler::rate_handler::get_rates_by_rater,
       crate::handler::comment_handler::save_comment,
       crate::handler::comment_handler::get_comment,
       crate::handler::comment_handler::get_comments_by_commented,
       crate::handler::comment_handler::get_comments_by_commentator
    ),
    components(schemas(User, Service, Modality, Roles, Comment, Rate, Category)),
)]
pub struct ApiDoc;