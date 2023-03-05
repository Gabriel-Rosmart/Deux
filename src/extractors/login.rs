use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonLogin {
    email: String,
    password: String,
}