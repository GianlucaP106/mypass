use core::panic;

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub struct MasterKey {
    value: Vec<u8>,
}

impl MasterKey {
    pub fn new(value: Vec<u8>) -> MasterKey {
        validate_length(&value, 32, "MasterKey");
        MasterKey { value }
    }
}

pub struct LightNonce {
    value: Vec<u8>,
}

impl LightNonce {
    pub fn new(value: Vec<u8>) -> LightNonce {
        validate_length(&value, 12, "LightNonce");
        LightNonce { value }
    }
}

pub struct Salt {
    value: Vec<u8>,
}

impl Salt {
    pub fn new(value: Vec<u8>) -> Salt {
        validate_length(&value, 12, "Salt");
        Salt { value }
    }
}

fn validate_length(v: &[u8], size: usize, var_name: &str) {
    let len = v.len();
    if len != size {
        panic!(
            "Unable to create {}. Length must be {} not {}.",
            var_name, size, len
        )
    }
}

pub fn encrypt(master_key: MasterKey, nonce: LightNonce, data: String) -> Result<Vec<u8>, String> {
    let key = Key::<Aes256Gcm>::from_slice(&master_key.value);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce.value);
    match cipher.encrypt(nonce, data.as_bytes().as_ref()) {
        Ok(ciphertext) => Ok(ciphertext.to_vec()),
        Err(_) => Err("Error encrypting data".to_owned()),
    }
}

pub fn decrypt(master_key: MasterKey, nonce: LightNonce, data: Vec<u8>) -> Result<String, String> {
    let key = Key::<Aes256Gcm>::from_slice(&master_key.value);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce.value);
    match cipher.decrypt(nonce, data.as_ref()) {
        Ok(plaintext) => match String::from_utf8(plaintext) {
            Ok(text) => Ok(text),
            Err(_) => Err("Unable to convert to string".to_owned()),
        },
        Err(_) => Err("Error decrypting data: {}".to_owned()),
    }
}

pub fn derive_master_key(master_password: String, salt: Vec<u8>) -> Result<MasterKey, String> {
    let master_password: &[u8] = master_password.as_bytes();
    let mut output_key_material = [0u8; 32];
    if let Err(e) =
        Argon2::default().hash_password_into(master_password, &salt, &mut output_key_material)
    {
        println!("Error: {}", e);
        return Err("Error deriving key password.".to_owned());
    }
    Ok(MasterKey::new(output_key_material.to_vec()))
}

pub fn hash_password(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("pass word hash error")
        .to_string();
    password_hash.to_string()
}

pub fn verify_password(password: String, hash: String) -> bool {
    let parsed_hash: PasswordHash = PasswordHash::new(&hash).expect("pass hash new");
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
