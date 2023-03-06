use argonautica::{ Hasher, Verifier, config::Variant };

pub struct Hash;

impl Hash {
    /* Consume the provided password preventing further usage after calling function */
    pub fn generate(password: String) ->String {
        let key = std::env::var("HASHER_SECRET_KEY").expect("No HASHER_SECRET_KEY env variable");

        let mut hasher = Hasher::default();

        hasher.configure_iterations(64).configure_variant(Variant::Argon2id);

        hasher
            .with_password(password)
            .with_secret_key(key)
            .hash().unwrap()
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