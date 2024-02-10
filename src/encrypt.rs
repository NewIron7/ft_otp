use hex;
use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;
use rand::RngCore;

use magic_crypt::{new_magic_crypt, MagicCryptTrait};

pub fn encrypt(key: &str, message: &str) -> String {
    let mc = new_magic_crypt!(key, 256);
    mc.encrypt_str_to_base64(message)
}

pub fn decrypt(key: &str, base64: &str) -> String {
    let mc = new_magic_crypt!(key, 256);
    mc.decrypt_base64_to_string(base64).unwrap()
}

/// Function that decrypt the message with the key
/// stored in the file ".encrypt.key"
/// if the file does not exist, it returns an empty string
/// Arguments:
/// - message: the message to decrypt
/// Returns:
/// - the decrypted message
pub fn decrypt_message(message: &str) -> String {
    let key = std::fs::read_to_string(".encrypt.key").unwrap_or_default();
    if key.is_empty() {
        return String::new();
    }
    decrypt(&key, message)
}

/// Function that encrypts the message with the key
/// stored in the file ".encrypt.key"
/// if the file does not exist, it returns an empty string
/// Arguments:
/// - message: the message to encrypt
/// Returns:
/// - the encrypted message
pub fn encrypt_message(message: &str) -> String {
    let mut key = std::fs::read_to_string(".encrypt.key").unwrap_or_default();
    if key.is_empty() {
        key = generate_key_encrypt();
    }
    encrypt(&key, message)
}

/// Function to get the decrypted key from a file
pub fn get_key_decrypted(path: &str) -> String {
    let key = std::fs::read_to_string(path);
    if key.is_err() {
        println!("❌ Error while reading the key");
        return String::new();
    }
    let key = key.unwrap();
    decrypt_message(&key)
}


/// Function that generate a key of 256 bits and save it 
/// in a file ".encrypt.key"
/// uses the ChaChaRng to generate the key
/// Arguments:
/// - none
/// Returns:
/// - key as string
fn generate_key_encrypt() -> String {
    println!("🔑 New key generated");
    let mut key = [0; 32];
    let mut rng = ChaChaRng::from_entropy();
    rng.fill_bytes(&mut key);
    let key = hex::encode(key);
    std::fs::write(".encrypt.key", &key).unwrap();
    key
}