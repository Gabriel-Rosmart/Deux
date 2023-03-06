use serde::{Deserialize, Serialize};
use validator::Validate;

/* Password is not validated to prevent leaking information */

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct JsonLogin {
    #[validate(email(message = "Email must be valid"))]
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct JsonRegister {
    #[validate(email(message = "Email must be valid"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}
