use sha1::{Digest, Sha1};

use crate::encrypt;

/// Function that modifies the key to be the correct length
/// for the HMAC-SHA1 algorithm
/// Arguments:
/// - key: the key to modify as vec<u8>
/// Returns:
/// - the modified key
pub fn prepare_key(key: &Vec<u8>) -> Vec<u8> {
    match key.len() {
        64 => key.clone(),
        0..=63 => {
            let mut key = key.clone();
            key.resize(64, 0);
            key
        }
        _ => {
            let mut hasher = Sha1::new();
            hasher.update(key);
            let result = hasher.finalize();
            let mut key = vec![0; 20];
            key.copy_from_slice(&result);
            key.resize(64, 0);
            key
        }
    }
}

/// Function that implements the HMAC-SHA1 algorithm
/// it uses the sha1 crate to compute the hash
/// and the HMAC logic is implemented manually
/// Arguments:
/// - key: the key to use for the HMAC
/// - message: the message to hash
/// Returns:
/// - the HMAC
pub fn hmac_sha1(key: &Vec<u8>, message: &[u8]) -> Vec<u8> {
    let key = prepare_key(key);
    let mut outer_key = vec![0x5c; 64];
    let mut inner_key = vec![0x36; 64];
    for i in 0..64 {
        outer_key[i] ^= key[i];
        inner_key[i] ^= key[i];
    }

    let mut hasher = Sha1::new();
    hasher.update(&inner_key);
    hasher.update(message);
    let result = hasher.finalize();
    let mut hmac = vec![0; 20];
    hmac.copy_from_slice(&result);

    let mut hasher = Sha1::new();
    hasher.update(&outer_key);
    hasher.update(&hmac);
    let result = hasher.finalize();
    let mut hmac = vec![0; 20];
    hmac.copy_from_slice(&result);
    hmac
}

/// Function that implements the TOTP algorithm
/// Arguments:
/// - key: the key to use for the HMAC
/// - time: the time to use for the TOTP
/// Returns:
/// - the TOTP
/// it uses the hmac_sha1 function to compute the HMAC
/// and the TOTP logic is implemented manually
pub fn generate(key: &Vec<u8>, time: u64) -> u32 {
    let time = time / 30;
    let time = time.to_be_bytes();
    let hmac = hmac_sha1(key, &time);
    let offset = (hmac[19] & 0xf) as usize;
    let value = ((hmac[offset] as u32 & 0x7f) << 24)
        | ((hmac[offset + 1] as u32) << 16)
        | ((hmac[offset + 2] as u32) << 8)
        | (hmac[offset + 3] as u32);
    value % 1_000_000
}

/// Function that generate a qr code for registering the key
/// in a TOTP app like Google Authenticator
/// Arguments:
/// - key: the key to use for the TOTP
/// Returns:
/// - none
/// it uses the qrcode crate to generate the qr code
/// and the image crate to save the qr code to a file
/// it saves the qr code to the file "qr.png"
/// the url to use for registering the key is "otpauth://totp/otp?secret=key
// Add the missing import statement for the base32 crate

pub fn generate_qr_code(path: &str, path_qr: &str) {
    let key = encrypt::get_key_decrypted(path);
    if key.is_empty() {
        return;
    }
    let key = encrypt::hex_key_to_vec(&key);
    if key.is_err() {
        return;
    }
    let key = key.unwrap();

    let key = base32::encode(base32::Alphabet::RFC4648 { padding: false }, &key);

    let url = format!(
        "otpauth://totp/ft_otp?secret={}&issuer=hboissel&algorithm=SHA1&digits=6&period=30",
        key
    );

    let code = qrcode::QrCode::new(url).unwrap();
    let image = code.render::<image::Luma<u8>>().build();
    let result_image = image.save(path_qr);
    if result_image.is_err() {
        println!("❌ Error while saving the qr code");
    }
}
