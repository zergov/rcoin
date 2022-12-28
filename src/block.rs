use openssl::hash::{hash, MessageDigest};

pub struct Block {
    pub header: Header,
}

pub struct Header {
    pub version: u32,
    pub prev_block_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub time: u32,
    pub bits: u32,
    pub nounce: u32,
}

pub fn genesis() -> Block {
    let merkle_root = hex::decode("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b").unwrap();

    Block {
        header: Header{
            version: 0x1,
            prev_block_hash: [0; 32],
            merkle_root: merkle_root.try_into().unwrap(),
            time: 1231006505,
            bits: 486604799,
            nounce: 2083236893,
        }
    }
}

impl Block {
    pub fn hash(&self) -> [u8; 32] {
        let mut payload: Vec<u8> = Vec::new();

        let mut prev_block_hash = self.header.prev_block_hash.clone();
        let mut merkle_root = self.header.merkle_root.clone();
        prev_block_hash.reverse();
        merkle_root.reverse();

        payload.extend(self.header.version.to_le_bytes());
        payload.extend(prev_block_hash);
        payload.extend(merkle_root);
        payload.extend(self.header.time.to_le_bytes());
        payload.extend(self.header.bits.to_le_bytes());
        payload.extend(self.header.nounce.to_le_bytes());

        let payload = hash(MessageDigest::sha256(), &payload).unwrap();
        let payload = hash(MessageDigest::sha256(), &payload).unwrap();

        let mut payload = payload.to_vec();
        payload.reverse();
        payload.try_into().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn genesis_header_values_test() {
        let header = genesis().header;

        assert_eq!(0x1, header.version);
        assert_eq!([0x0; 32], header.prev_block_hash);
        assert_eq!(
            hex::decode("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b").unwrap(),
            header.merkle_root
        );
        assert_eq!(1231006505, header.time);
        assert_eq!(486604799, header.bits);
        assert_eq!(2083236893, header.nounce);
    }

    #[test]
    fn genesis_hash_test() {
        assert_eq!(
            "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
            hex::encode(genesis().hash().to_vec())
        );
    }
}
