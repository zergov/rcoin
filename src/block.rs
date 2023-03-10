use crate::hash::hash256;
use crate::transactions::Transaction;
use crate::u256;

pub struct Block {
    pub header: Header,
    pub transactions: Vec<Transaction>,
}

pub struct Header {
    pub version: u32,
    pub prev_block_hash: u256,
    pub merkle_root: u256,
    pub time: u32,
    pub bits: u32,
    pub nounce: u32,
}

pub fn genesis() -> Block {
    Block {
        header: Header{
            version: 0x1,
            prev_block_hash: u256::new(0),
            merkle_root: u256::from_str_radix("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b", 16).unwrap(),
            time: 1231006505,
            bits: 486604799,
            nounce: 2083236893,
        },
        transactions: vec![],
    }
}

impl Block {
    pub fn hash(&self) -> u256 {
        let mut payload: Vec<u8> = Vec::new();

        payload.extend(self.header.version.to_le_bytes());
        payload.extend(self.header.prev_block_hash.to_le_bytes());
        payload.extend(self.header.merkle_root.to_le_bytes());
        payload.extend(self.header.time.to_le_bytes());
        payload.extend(self.header.bits.to_le_bytes());
        payload.extend(self.header.nounce.to_le_bytes());

        u256::from_le_bytes(hash256(&payload).try_into().unwrap())
    }

    pub fn hash_hex(&self) -> String {
        hex::encode(self.hash().to_be_bytes())
    }

    pub fn prev_block_hash_hex(&self) -> String {
        hex::encode(self.header.prev_block_hash.to_be_bytes())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn genesis_header_values_test() {
        let header = genesis().header;

        assert_eq!(0x1, header.version);
        assert_eq!(0, header.prev_block_hash);
        assert_eq!(
            u256::from_str_radix("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b", 16).unwrap(),
            header.merkle_root
        );
        assert_eq!(1231006505, header.time);
        assert_eq!(486604799, header.bits);
        assert_eq!(2083236893, header.nounce);
    }

    #[test]
    fn genesis_hash_test() {
        assert_eq!(
            u256::from_str_radix("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f", 16).unwrap(),
            genesis().hash()
        );
    }

    #[test]
    fn genesis_hash_hex_test() {
        assert_eq!(
            "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
            genesis().hash_hex()
        );
    }

    #[test]
    fn prev_block_hash_hex_test() {
        assert_eq!(
            "0000000000000000000000000000000000000000000000000000000000000000",
            genesis().prev_block_hash_hex()
        );
    }
}
