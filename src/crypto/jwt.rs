use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use mongodb::bson::oid::ObjectId;
use sha2::Sha256;

use serde::{Deserialize, Serialize};

use crate::errors::server::AppError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    _id: ObjectId,
    email: String,
}

impl Claims {
    pub fn new(id: ObjectId, email: &str) -> Self {
        Self {
            _id: id,
            email: email.to_string(),
        }
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }
}

pub struct JWT;

impl JWT {
    pub fn generate(claims: Claims) -> Result<String, AppError> {
        let jwt_secret = std::env::var("JWT_SECRET").expect("No JWT_SECRET provided in env file");
        let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes())?;
        let token_str = claims.sign_with_key(&key)?;
        Ok(token_str)
    }

    pub fn validate(token: &str) -> Result<Claims, AppError> {
        let jwt_secret = std::env::var("JWT_SECRET").expect("No JWT_SECRET provided in env file");
        let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes())?;

        /* For some reason, if you don't store it in a variable,
        the program gives an error
        */
        let claims: Claims = token.verify_with_key(&key)?;
        Ok(claims)
    }
}
