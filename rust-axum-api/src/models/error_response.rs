use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
#[allow(dead_code)]
pub struct ErrorResponse {
    #[schema(example = "Invalid input provided")]
    pub message: String,
    #[schema(example = 400)]
    pub code: i32,
}
