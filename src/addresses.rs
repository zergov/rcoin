use crate::keys::Keychain;

use openssl::hash::{hash, MessageDigest};

pub fn from_keychain(keychain: &Keychain) -> String {
    let payload = keychain.public_key_bytes();
    let payload = hash(MessageDigest::sha256(), &payload).unwrap();
    let payload = hash(MessageDigest::ripemd160(), &payload).unwrap();
    let mut payload = payload.to_vec();

    // Base58Check prefix version: 0x00 for addresses.
    payload.insert(0, 0x00);

    let checksum = hash(MessageDigest::sha256(), &payload).unwrap();
    let checksum = hash(MessageDigest::sha256(), &checksum).unwrap();
    let mut checksum = checksum
        .to_vec()
        .into_iter()
        .take(4)
        .collect();

    payload.append(&mut checksum);

    bs58::encode(payload).into_string()
}
