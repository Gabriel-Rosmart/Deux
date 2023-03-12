use serde::{Deserialize, Serialize};

/* Password is not validated to prevent leaking information */

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateProfileRequest {
    pub password: String,
}