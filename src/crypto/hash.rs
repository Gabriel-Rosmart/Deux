use argonautica::{config::Variant, Hasher, Verifier};

pub struct Iterations;

#[allow(unused)]
impl Iterations {
    pub const SLOW: u32 = 192;
    pub const MEDIUM: u32 = 64;
    pub const FAST: u32 = 32;
}

pub struct Hash;

impl Hash {
    pub fn generate(password: &str, iterations: u32) -> String {
        let key = std::env::var("HASHER_SECRET_KEY").expect("No HASHER_SECRET_KEY env variable");

        let mut hasher = Hasher::default();

        hasher
            .configure_iterations(iterations)
            .configure_variant(Variant::Argon2id);

        hasher
            .with_password(password)
            .with_secret_key(key)
            .hash()
            .unwrap()
    }

    pub fn verify(password: &str, hash: &str) -> bool {
        let key = std::env::var("HASHER_SECRET_KEY").expect("No HASHER_SECRET_KEY env variable");

        let mut verifier = Verifier::default();

        verifier
            .with_hash(hash)
            .with_password(password)
            .with_secret_key(key)
            .verify()
            .unwrap()
    }
}
