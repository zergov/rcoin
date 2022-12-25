
use rcoin::keys::{generate_keychain};

fn main() {
    let keychain = generate_keychain();

    println!("Rcoin address: {}", keychain.address());
    println!("==================================");
    println!("private key:\t{}", keychain.private_key_to_hex());
    println!("public key:\t{}", keychain.public_key_to_hex());
}
