fn main() {
    let keychain = rcoin::keys::generate_keychain();

    println!("Rcoin address: {}", rcoin::addresses::from_keychain(&keychain));
    println!("==================================");
    println!("private key:\t{}", keychain.private_key_hex());
    println!("public key:\t{}", keychain.public_key_hex());
}
