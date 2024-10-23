slint::include_modules!();

use slint::Image;
use std::path::Path;

use crate::totp;
use crate::utils;

pub fn run(
    default_hex_key_path: String,
    default_encryted_key_path: String,
    default_qr_code_path: String,
) -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    let default_encryted_key_path_copy = default_encryted_key_path.clone();

    let main_generate_handle = main_window.as_weak();
    main_window.on_generate_otp_key(
        move |hex_key_path: slint::SharedString, qr_code_path: slint::SharedString| {
            let ui = main_generate_handle.unwrap();
            let hex_key_path = if hex_key_path.is_empty() {
                default_hex_key_path.clone()
            } else {
                hex_key_path.to_string()
            };
            let qr_code_path = if qr_code_path.is_empty() {
                default_qr_code_path.clone()
            } else {
                qr_code_path.to_string()
            };

            if utils::save_key(&hex_key_path, &default_encryted_key_path).is_err() {
                ui.set_info_generate("❌ Error while saving the key".into());
                return;
            }
            totp::generate_qr_code(&default_encryted_key_path, &qr_code_path);
            let qr_code_path = Path::new(&qr_code_path);
            let qr_code_image = Image::load_from_path(qr_code_path);
            if qr_code_image.is_err() {
                ui.set_info_generate("❌ Error while loading the qr code".into());
                return;
            }
            let qr_code_image = qr_code_image.unwrap();
            ui.set_qr_code_image(qr_code_image);
            ui.set_info_generate("✅ Key saved".into());
        },
    );

    let default_encryted_key_path = default_encryted_key_path_copy;
    let main_totp_handle = main_window.as_weak();
    main_window.on_generate_totp(move |encrypted_key_path: slint::SharedString| {
        let ui = main_totp_handle.unwrap();
        let encrypted_key_path = if encrypted_key_path.is_empty() {
            default_encryted_key_path.clone()
        } else {
            encrypted_key_path.to_string()
        };

        let totp = utils::do_totp(&encrypted_key_path);
        if totp.is_err() {
            ui.set_info_totp("❌ Error while generating the TOTP".into());
            return;
        }
        let totp = totp.unwrap();
        ui.set_totp_code(totp.into());
        ui.set_info_totp("".into());
    });

    main_window.run()
}
