use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    email: String,
}

pub struct JWT;

impl JWT {
    pub fn generate() -> String {
        let claims = Claims {
            email: "test@gmail.com".to_string(),
        };
        let jwt_secret = std::env::var("JWT_SECRET").expect("No JWT_SECRET provided in env file");
        let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
        let token_str = claims.sign_with_key(&key).unwrap();
        token_str
    }
}
