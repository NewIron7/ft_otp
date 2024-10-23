use clap::{command, value_parser, Arg};

mod encrypt;
mod gui;
mod totp;
mod utils;

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
                .help("Path to the hex key file to ecrypt the key, 64 bytes min"),
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
                .help("GUI interface"),
        )
        .get_matches();

    let gui: &bool = matches.get_one::<bool>("gui").unwrap();
    let hex_key: &String = matches.get_one::<String>("generate").unwrap();
    let encrypted_key: &String = matches.get_one::<String>("key").unwrap();
    let qrcode: &String = matches.get_one::<String>("qrcode").unwrap();

    if *gui {
        let gui_state = gui::run(
            "key.hex".to_string(),
            encrypted_key.to_string(),
            qrcode.to_string(),
        );
        if gui_state.is_err() {
            println!("❌ Error while running the GUI");
            return;
        }
        return;
    }

    if !hex_key.is_empty() {
        if utils::save_key(hex_key, encrypted_key).is_err() {
            return;
        }
        totp::generate_qr_code(encrypted_key, qrcode);
        return;
    }

    let result_totp = utils::do_totp(encrypted_key);
    if result_totp.is_err() {
        return;
    }
    let result_totp = result_totp.unwrap();
    println!("🔒 TOTP: {result_totp}");
}
