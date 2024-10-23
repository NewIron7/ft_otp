use crate::encrypt;
use crate::totp;

/// Function that gets the content of the file and encrypts it
/// and saves the encrypted content in a file `ft_otp.key`
/// Arguments:
/// - path: the path to the file to encrypt
/// Returns:
/// - none
pub fn save_key(path: &str, path_encrypted_key: &str) -> Result<(), ()> {
    let key = std::fs::read_to_string(path);
    if key.is_err() {
        println!("❌ Error while reading the key");
        return Err(());
    }
    let key = key.unwrap();
    if key.len() < 128 {
        println!("❌ The key is too short, 64 bytes are required so 128 characters in hex format");
        return Err(());
    }
    if encrypt::hex_key_to_vec(&key).is_err() {
        println!("❌ The key is not in the correct format.");
        return Err(());
    }
    let encrypted = encrypt::encrypt_message(&key);
    let result_write = std::fs::write(path_encrypted_key, encrypted);
    if result_write.is_err() {
        println!("❌ Error while writing the key");
        return Err(());
    }
    println!("✅ Key saved");
    Ok(())
}

pub fn do_totp(key_path: &str) -> Result<String, ()> {
    let key = encrypt::get_key_decrypted(key_path);
    if key.is_empty() {
        return Err(());
    }
    let key = encrypt::hex_key_to_vec(&key);
    if key.is_err() {
        return Err(());
    }
    let key = key.unwrap();
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let totp = totp::generate(&key, time);

    let totp = format!("{:03} {:03}", totp / 1_000, totp % 1_000);
    Ok(totp)
}
