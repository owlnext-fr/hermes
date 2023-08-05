use ulid::Ulid;

pub struct Cipher;

impl Cipher {
    pub fn hash(message: &str) -> String {
        sha256::digest(message)
    }

    pub fn cipher(msg: &str) -> String {
        let salt = Ulid::new().to_string();
        let config = argon2::Config {
            variant: argon2::Variant::Argon2id,
            ..Default::default()
        };

        let hash = argon2::hash_encoded(msg.as_bytes(), salt.as_bytes(), &config).unwrap();

        hash
    }

    pub fn check(msg: &str, hash: &str) -> bool {
        argon2::verify_encoded(hash, msg.as_bytes()).unwrap()
    }

    pub fn validate_password_complexity(msg: &str) -> bool {
        let check = zxcvbn::zxcvbn(msg, &[]);

        if check.is_err() {
            return false;
        }

        check.unwrap().score() >= 3
    }
}
