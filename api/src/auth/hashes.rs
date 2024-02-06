/*
    This is where we define the hashing functions for the user passwords.

    password_hash and password_hash_async are the functions that hash a password
    password_test and password_test_async are for testing a password against a hash
    they use the expensive argon2 algorithm

    the lazy functions are for testing purposes, they use the murmur3 hashing algorithm
    (which is not secure) and are much faster than the argon2 functions

*/
use rocket::tokio::task;
use anyhow::Result;
use std::env;

// hashing some stuff
use ::argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use std::io::Cursor;
use murmur3::murmur3_x86_128;

pub fn password_hash(password: &str) -> Result<String> {
    if env::var("USE_INSECURE_HASHES").unwrap_or_else(|_| "false".to_string()) == "true" {
        println!("using insecure hash");
        return lazy_password_hash(password);
    }
    let peppered: String = format!("{}-{}-{}", password, env::var("GROOVELET_PEPPER").unwrap_or_else(|_| "peppa".to_string()), "SPUDJIBMSPLQPFFSPLBLBlBLBLPRT");
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed_password = argon2.hash_password(peppered.as_bytes(), &salt).expect("strings should be hashable").to_string();
    Ok(hashed_password)
}

pub async fn password_hash_async(password: &str) -> Result<String> {
    let password = password.to_string();
    let result = task::spawn_blocking(move || {
        password_hash(&password)
    }).await?;

    result
}

pub fn password_test(password: &str, hashed_password: &str) -> Result<bool> {
    if env::var("USE_INSECURE_HASHES").unwrap_or_else(|_| "false".to_string()) == "true" {
        println!("using insecure hash");
        return lazy_password_test(password, hashed_password);
    }
    let peppered: String = format!("{}-{}-{}", password, env::var("GROOVELET_PEPPER").unwrap_or_else(|_| "peppa".to_string()), "SPUDJIBMSPLQPFFSPLBLBlBLBLPRT");
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(hashed_password).unwrap();
    let is_valid = argon2
        .verify_password(peppered.as_bytes(), &password_hash)
        .is_ok();
    Ok(is_valid)
}

pub async fn password_test_async(password: &str, hashed_password: &str) -> Result<bool> {
    let password = password.to_string();
    let hashed_password = hashed_password.to_string();
    let result = task::spawn_blocking(move || {
        password_test(&password, &hashed_password)
    }).await?;

    result
}

pub fn lazy_password_hash(password: &str) -> Result<String> {
    let hash_result = murmur3_x86_128(&mut Cursor::new(password), 0).expect("hashing works");
    Ok(hash_result.to_string())
}

pub fn lazy_password_test(password: &str, hashed_password: &str) -> Result<bool> {
    let hash_result = murmur3_x86_128(&mut Cursor::new(password), 0).expect("hashing works");
    Ok(hash_result.to_string() == hashed_password)
}