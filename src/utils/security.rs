//! # Security Module
//!
//! This module provides cryptographic operations for Raze, including:
//! - Password-based key derivation using Argon2id.
//! - Authenticated encryption and decryption using AES-256-GCM.
//! - Secure chunk-based streaming for large files.

use crate::utils::errors::RazeError;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{password_hash::SaltString, Argon2, Params};
use rand::{rngs::OsRng, RngCore};
use std::io::{Read, Write};
use zeroize::Zeroize;

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const CHUNK_SIZE: usize = 64 * 1024; // 64KB chunks
const MAGIC_ENCRYPTED: &[u8] = b"RZCR"; // Raze CRypt

/// Derives a 32-byte key from a password and salt using Argon2id.
pub fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32], RazeError> {
    let mut password_bytes = password.as_bytes().to_vec();
    let params =
        Params::new(65536, 3, 4, Some(32)).map_err(|e| RazeError::CryptoError(e.to_string()))?;
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

    let mut key = [0u8; 32];
    let salt_str =
        SaltString::encode_b64(salt).map_err(|e| RazeError::CryptoError(e.to_string()))?;

    argon2
        .hash_password_into(
            password_bytes.as_slice(),
            salt_str.as_str().as_bytes(),
            &mut key,
        )
        .map_err(|e| RazeError::CryptoError(e.to_string()))?;

    password_bytes.zeroize();
    Ok(key)
}

/// Encrypts a stream using AES-256-GCM with a password.
pub fn encrypt_stream<R: Read, W: Write>(
    mut reader: R,
    mut writer: W,
    password: &str,
) -> Result<(), RazeError> {
    // 1. Write Magic Header
    writer.write_all(MAGIC_ENCRYPTED)?;

    // 2. Generate and write Salt
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);
    writer.write_all(&salt)?;

    // 3. Derive Key
    let key = derive_key(password, &salt)?;
    let cipher =
        Aes256Gcm::new_from_slice(&key).map_err(|e| RazeError::CryptoError(e.to_string()))?;

    // 4. Generate and write base Nonce
    let mut base_nonce = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut base_nonce);
    writer.write_all(&base_nonce)?;

    // 5. Encrypt chunks
    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut chunk_index: u64 = 0;

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }

        // Create a unique nonce for each chunk using the base nonce and chunk index
        let mut nonce_bytes = base_nonce;
        let index_bytes = chunk_index.to_le_bytes();
        for i in 0..8 {
            nonce_bytes[i] ^= index_bytes[i];
        }
        let nonce = Nonce::from_slice(&nonce_bytes);

        let encrypted_data = cipher
            .encrypt(nonce, &buffer[..n])
            .map_err(|e| RazeError::CryptoError(e.to_string()))?;

        // Write chunk length (4 bytes) and then encrypted data
        writer.write_all(&(encrypted_data.len() as u32).to_le_bytes())?;
        writer.write_all(&encrypted_data)?;

        chunk_index += 1;
    }

    Ok(())
}

/// Decrypts a stream using AES-256-GCM with a password.
pub fn decrypt_stream<R: Read, W: Write>(
    mut reader: R,
    mut writer: W,
    password: &str,
) -> Result<(), RazeError> {
    // 1. Read and verify Magic Header
    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic)?;
    if magic != MAGIC_ENCRYPTED {
        return Err(RazeError::CryptoError(
            "Invalid encrypted archive format".to_string(),
        ));
    }

    // 2. Read Salt
    let mut salt = [0u8; SALT_LEN];
    reader.read_exact(&mut salt)?;

    // 3. Derive Key
    let key = derive_key(password, &salt)?;
    let cipher =
        Aes256Gcm::new_from_slice(&key).map_err(|e| RazeError::CryptoError(e.to_string()))?;

    // 4. Read base Nonce
    let mut base_nonce = [0u8; NONCE_LEN];
    reader.read_exact(&mut base_nonce)?;

    // 5. Decrypt chunks
    let mut chunk_index: u64 = 0;
    loop {
        let mut len_bytes = [0u8; 4];
        match reader.read_exact(&mut len_bytes) {
            Ok(_) => (),
            Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e.into()),
        }
        let len = u32::from_le_bytes(len_bytes) as usize;

        let mut encrypted_data = vec![0u8; len];
        reader.read_exact(&mut encrypted_data)?;

        // Recreate the unique nonce for this chunk
        let mut nonce_bytes = base_nonce;
        let index_bytes = chunk_index.to_le_bytes();
        for i in 0..8 {
            nonce_bytes[i] ^= index_bytes[i];
        }
        let nonce = Nonce::from_slice(&nonce_bytes);

        let decrypted_data = cipher
            .decrypt(nonce, encrypted_data.as_slice())
            .map_err(|e| {
                RazeError::CryptoError(format!("Authentication failed (wrong password?): {}", e))
            })?;

        writer.write_all(&decrypted_data)?;
        chunk_index += 1;
    }

    Ok(())
}
