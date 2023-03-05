use serde::{ Serialize, Deserialize };
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct JsonLogin {
    #[validate(email(message = "Email must be valid"))]
    email: String,
    password: String,
}