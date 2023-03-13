use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}