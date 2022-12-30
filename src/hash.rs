use openssl::hash::{hash, MessageDigest};

pub fn hash256(data: &[u8]) -> Vec<u8> {
    let data = hash(MessageDigest::sha256(), &data).unwrap();
    let data = hash(MessageDigest::sha256(), &data).unwrap();
    data.to_vec()
}
