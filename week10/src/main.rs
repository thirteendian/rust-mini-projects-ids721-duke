use anyhow::{Context, Result};
use sodiumoxide::crypto::secretbox;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn main() -> Result<()> {
    sodiumoxide::init().expect("Failed to initialize sodiumoxide");

    let input_path = Path::new("input.txt");
    let encrypted_path = Path::new("encrypted.bin");
    let decrypted_path = Path::new("decrypted.txt");

    // Generate a new key and nonce for encryption
    let key = secretbox::gen_key();
    let nonce = secretbox::gen_nonce();

    // Read the input file
    let input_data = read_file(input_path)?;

    // Encrypt the file
    let encrypted_data = encrypt_data(&input_data, &key, &nonce);
    write_file(encrypted_path, &encrypted_data)?;

    // Decrypt the file
    let decrypted_data = decrypt_data(&encrypted_data, &key, &nonce)?;
    write_file(decrypted_path, &decrypted_data)?;

    Ok(())
}

fn read_file(path: &Path) -> Result<Vec<u8>> {
    let mut file = File::open(path).with_context(|| format!("Failed to open file: {:?}", path))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .with_context(|| format!("Failed to read file: {:?}", path))?;
    Ok(data)
}

fn write_file(path: &Path, data: &[u8]) -> Result<()> {
    let mut file = File::create(path).with_context(|| format!("Failed to create file: {:?}", path))?;
    file.write_all(data)
        .with_context(|| format!("Failed to write file: {:?}", path))?;
    Ok(())
}

fn encrypt_data(data: &[u8], key: &secretbox::Key, nonce: &secretbox::Nonce) -> Vec<u8> {
    secretbox::seal(data, nonce, key)
}

fn decrypt_data(data: &[u8], key: &secretbox::Key, nonce: &secretbox::Nonce) -> Result<Vec<u8>> {
    secretbox::open(data, nonce, key)
        .map_err(|_| anyhow::Error::msg("Failed to decrypt data"))
}