# 🔐 TOTP

## 📄 Overview
This project is a simple command-line tool created as a school project using Rust. 🎓  
It's designed to generate Time-based One-Time Passwords (TOTP) by encrypting a key and generating a QR code for TOTP app registration. 🕒

## ✨ Features
- **Encrypt Key**: 🔒 Securely encrypts a provided hex key and saves it for TOTP generation.
- **Generate QR Code**: 📱 Creates a QR code to easily add the encrypted key to a TOTP app.
- **Generate TOTP**: 🗝️ Produces a TOTP code using the encrypted key and current time.

## 📋 Requirements
- Rust Programming Language 🦀

## 🚀 Installation
To set up this project, follow these steps:
```bash
git clone <repository-url>
cd ft_otp
cargo build --release
```

## 🛠️ Usage
Execute the program with options to encrypt a key, generate a QR code, or produce a TOTP code:
```bash
./target/release/ft_otp [OPTIONS]
```

### Options
- `-g, --generate <path>`: 🔐 Encrypt and save a hex key from a specified path.
- `-q, --qrcode <file>`: 🖼️ Specify the output file for the QR code (default `qr.png`).
- `-k, --key <path>`: 🗝️ Use an encrypted key file to generate the TOTP code (default `ft_otp.key`).

## 📝 Examples
- To encrypt a key and generate a QR code:
  ```bash
  ./target/release/ft_otp --generate your_key_path --qrcode your_qr_code.png
  ```
- To generate a TOTP code:
  ```bash
  ./target/release/ft_otp --key your_encrypted_key_file
  ```

## Comparison
After generating the TOTP code with both your Rust tool and the `oathtool` Docker container, you can compare the results to verify the accuracy and consistency of your implementation.

```
docker build -t oathtool .
cargo run && docker run oathtool $(cat key.hex)
```
