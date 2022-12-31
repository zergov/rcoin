use openssl::hash::{hash, MessageDigest};

pub fn sha256(data: &[u8]) -> Vec<u8> {
    let data = hash(MessageDigest::sha256(), &data).unwrap();
    data.to_vec()
}

pub fn hash160(data: &[u8]) -> Vec<u8> {
    let data = hash(MessageDigest::sha256(), &data).unwrap();
    let data = hash(MessageDigest::ripemd160(), &data).unwrap();
    data.to_vec()
}

pub fn hash256(data: &[u8]) -> Vec<u8> {
    let data = hash(MessageDigest::sha256(), &data).unwrap();
    let data = hash(MessageDigest::sha256(), &data).unwrap();
    data.to_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sha256() {
        let data = hex::decode("72636f696e").unwrap();
        let expected = hex::decode("e49dc62d36294343898b5a0b29335600c1106b70a2827371fe1321013d764a85").unwrap();
        assert_eq!(expected, sha256(&data))
    }
}
