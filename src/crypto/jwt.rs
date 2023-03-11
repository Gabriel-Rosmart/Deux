use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    email: String,
}

impl Claims {
    pub fn new(email: &str) -> Self {
        Self {
            email: email.to_string()
        }
    }
}

pub struct JWT;

impl JWT {
    pub fn generate(claims: Claims) -> String {
        /*
        let claims = Claims {
            email: "test@gmail.com".to_string(),
        };*/
        let jwt_secret = std::env::var("JWT_SECRET").expect("No JWT_SECRET provided in env file");
        let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
        let token_str = claims.sign_with_key(&key).unwrap();
        token_str
    }

    pub fn validate(token: &str) -> Result<Claims, jwt::error::Error> {
        let jwt_secret = std::env::var("JWT_SECRET").expect("No JWT_SECRET provided in env file");
        let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();

        /* For some reason, if you don't store it in a variable,
        the program gives an error
        */
        let claims: Claims = token.verify_with_key(&key)?;
        Ok(claims)
    }
}
