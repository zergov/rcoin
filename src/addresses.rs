use crate::keys::Keychain;
use crate::hash::hash256;

use openssl::hash::{hash, MessageDigest};

pub fn from_keychain(keychain: &Keychain) -> String {
    let payload = keychain.public_key_bytes();
    let payload = hash(MessageDigest::sha256(), &payload).unwrap();
    let payload = hash(MessageDigest::ripemd160(), &payload).unwrap();
    let mut payload = payload.to_vec();

    // Base58Check prefix version: 0x00 for addresses.
    payload.insert(0, 0x00);

    let checksum = &hash256(&payload)[0..4];
    payload.extend(checksum);

    bs58::encode(payload).into_string()
}
