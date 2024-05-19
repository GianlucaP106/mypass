use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key as AesKey, Nonce as AesNonce,
};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::{distributions::Alphanumeric, Rng};

use crate::error::Error;

pub struct FixedLengthKey {
    value: Vec<u8>,
}

impl FixedLengthKey {
    pub fn new(value: Vec<u8>, size: usize) -> FixedLengthKey {
        assert!(
            value.len() == size,
            "Nonce must be exactly {} long not {}",
            size,
            value.len()
        );
        FixedLengthKey { value }
    }

    pub fn shrink_from(value: String, size: usize) -> FixedLengthKey {
        assert!(value.len() >= size);
        let value = value.as_bytes();
        let value = &value[0..size];
        FixedLengthKey::new(value.to_vec(), size)
    }

    pub fn shrink(value: Vec<u8>, size: usize) -> FixedLengthKey {
        assert!(value.len() >= size);
        let value = &value[0..size];
        FixedLengthKey::new(value.to_vec(), size)
    }
}

pub fn decrypt_password(
    master_password: String,
    password: Vec<u8>,
    nonce: String,
    salt: String,
) -> Result<String, Error> {
    let master_key = derive_master_key(master_password, FixedLengthKey::shrink_from(salt, 12))?;
    let key = AesKey::<Aes256Gcm>::from_slice(&master_key.value);
    let cipher = Aes256Gcm::new(key);
    let nonce = FixedLengthKey::shrink_from(nonce, 12);
    let nonce = AesNonce::from_slice(&nonce.value);
    let err = "Failed to decrypt password".to_owned();
    cipher
        .decrypt(nonce, password.as_ref())
        .map_err(|_| err.to_owned())
        .and_then(|plaintext| String::from_utf8(plaintext).map_err(|_| err))
}

pub fn encrypt_password(
    master_password: String,
    password: String,
    nonce: String,
    salt: String,
) -> Result<Vec<u8>, Error> {
    let master_key = derive_master_key(master_password, FixedLengthKey::shrink_from(salt, 12))?;
    let key = AesKey::<Aes256Gcm>::from_slice(&master_key.value);
    let cipher = Aes256Gcm::new(key);
    let nonce = FixedLengthKey::shrink_from(nonce, 12);
    let nonce = AesNonce::from_slice(&nonce.value);
    cipher
        .encrypt(nonce, password.as_bytes().as_ref())
        .map_err(|_| "Failed to encrypt password".to_owned())
        .map(|ciphertext| ciphertext.to_vec())
}

pub fn derive_master_key(
    master_password: String,
    salt: FixedLengthKey,
) -> Result<FixedLengthKey, Error> {
    let master_password: &[u8] = master_password.as_bytes();
    let mut output_key_material = [0u8; 32];
    Argon2::default()
        .hash_password_into(master_password, &salt.value, &mut output_key_material)
        .map_err(|_| "Failed to derive key from master password.")?;
    Ok(FixedLengthKey::new(output_key_material.to_vec(), 32))
}

pub fn hash_password(password: String) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| "Faild to hash password")?
        .to_string();
    Ok(password_hash.to_string())
}

pub fn verify_password(password: String, hash: String) -> Result<bool, Error> {
    let parsed_hash: PasswordHash =
        PasswordHash::new(&hash).map_err(|_| "Failed to validate password")?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn generate_password() -> String {
    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    password
}
