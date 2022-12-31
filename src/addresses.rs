use crate::hash::{hash160, hash256};
use crate::keys::Keychain;

pub fn from_keychain(keychain: &Keychain) -> String {
    let mut payload = hash160(&keychain.public_key_bytes());

    // Base58Check prefix version: 0x00 for addresses.
    payload.insert(0, 0x00);

    let checksum = &hash256(&payload)[0..4];
    payload.extend(checksum);

    bs58::encode(payload).into_string()
}
