use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Debug, Deserialize, PartialEq, ToSchema)]
pub struct Message {
    pub message: String,
}
