use bcrypt::{DEFAULT_COST, hash, verify};

pub fn hash_password(password: &str) -> Result<String, String> {
    hash(password, DEFAULT_COST).map_err(|e| e.to_string())
}

pub fn verify_password(password: &str, hashed: &str) -> Result<bool, String> {
    verify(password, hashed).map_err(|e| e.to_string())
}
