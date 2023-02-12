use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use argon2::password_hash::{Error, SaltString};
use argon2::password_hash::rand_core::OsRng;

pub fn hash<T: ToString>(password: T) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let params = Params::new(65536, 8, 4, None);
    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        params?,
    );

    let password_hash = argon2.hash_password(
        password.to_string().as_bytes(),
        &salt,
    )?;

    Ok(password_hash.to_string())
}

pub fn verify<T: ToString, S: ToString>(
    password: T,
    password_hash: S,
) -> Result<(), Error> {
    let password_hash = password_hash.to_string();
    let password_hash = PasswordHash::new(&password_hash)?;

    Argon2::default().verify_password(
        password.to_string().as_bytes(),
        &password_hash,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let password = "SuPeR SeCrEt pAsSw0rD";
        let password_hash = hash(password).expect("Failed to hash password");

        verify(password, password_hash).expect("Entered password is incorrect");
    }
}
