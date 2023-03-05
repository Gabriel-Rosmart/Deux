use argonautica::Hasher;

pub struct Hash;

impl Hash {
    pub fn generate(password: &str) -> (String, String) {
        let key = std::env::var("HASHER_SECRET_KEY").expect("No HASHER_SECRET_KEY env variable");

        let mut hasher = Hasher::default();

        let salt = std::str::from_utf8(hasher.salt().as_bytes()).unwrap().to_string();

        let hash = hasher
            .with_password(password)
            .with_secret_key(key)
            .hash()
            .unwrap();


        (hash, salt)
    }
}