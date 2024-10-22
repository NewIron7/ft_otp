

use clap::{command, Arg, value_parser};

mod totp;
use totp::*;

mod encrypt;
use encrypt::*;

mod gui;

/// Main function that runs TOTP with a key and the current time
/// it takes the key as command line argument
/// and the current time is obtained using the time crate
/// it then prints the TOTP
fn main() {

    let matches = command!()
        .version("1.0")
        .author("hboissel")
        .about("Generate TOTP codes from a key")
        .arg(
            Arg::new("generate")
                .short('g')
                .long("generate")
                .default_value("")
                .help("Path to the hex key file to ecrypt the key, 64 bytes min. Hex format: 1234567890abcdef"),
        )
        .arg(
            Arg::new("qrcode")
                .short('q')
                .long("qrcode")
                .default_value("qr.png")
                .help("Output file for the QR code to register the key in a TOTP app"),
        )
        .arg(
            Arg::new("key")
                .short('k')
                .long("key")
                .default_value("ft_otp.key")
                .help("Path to the encrypted key file to generate the TOTP code from"),
        )
        .arg(
            Arg::new("gui")
                .short('i')
                .long("gui")
                .default_value("false")
                .num_args(0..=1)
                .require_equals(true)
                .default_missing_value("true")
                .value_parser(value_parser!(bool))
                .help("Recursively download images"),
        )
        .get_matches();
    
    let gui: &bool = matches.get_one::<bool>("gui").unwrap();

    if *gui {
        gui::run();
        return;
    }

    let hex_key: &String = matches.get_one::<String>("generate").unwrap();
    let encrypted_key: &String = matches.get_one::<String>("key").unwrap();
    let qrcode: &String = matches.get_one::<String>("qrcode").unwrap();
    
    if !hex_key.is_empty() {
        if save_key(&hex_key).is_err() {
            return;
        }
        generate_qr_code("ft_otp.key", &qrcode);
        return;
    }
    
    do_totp(&encrypted_key)
}

fn do_totp(key_path: &str) {
    let key = get_key_decrypted(key_path);
    if key.len() == 0 {
        return;
    }
    let key = hex_key_to_vec(&key);
    if key.is_err() {
        return;
    }
    let key = key.unwrap();
    let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let totp = totp(&key, time);

    println!("🔒 TOTP: {:03} {:03}", totp / 1_000, totp % 1_000);
}

/// Function that gets the content of the file and encrypts it
/// and saves the encrypted content in a file "ft_otp.key"
/// Arguments:
/// - path: the path to the file to encrypt
/// Returns:
/// - none
fn save_key(path: &str) -> Result<(), ()> {
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
    if hex_key_to_vec(&key).is_err() {
        println!("❌ The key is not in the correct format. Use hex format: 1234567890abcdef");
        return Err(());
    }
    let encrypted = encrypt_message(&key);
    let result_write = std::fs::write("ft_otp.key", encrypted);
    if result_write.is_err() {
        println!("❌ Error while writing the key");
        return Err(());
    }
    println!("✅ Key saved");
    Ok(())
}
