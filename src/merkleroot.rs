use crate::hash::hash256;

// implementation comes from https://learnmeabitcoin.com/technical/merkle-root
pub fn merkleroot(txids: Vec<String>) -> String {
    let mut result: Vec<String> = vec![];

    for txid_chunk in txids.chunks(2) {
        let concat = if txid_chunk.len() == 2 {
            format!("{}{}", txid_chunk[0], txid_chunk[1])
        } else {
            format!("{}{}", txid_chunk[0], txid_chunk[0])
        };

        let bytes = hex::decode(concat).unwrap();
        result.push(hex::encode(hash256(&bytes)));
    };

    if result.len() == 1 {
        return result.first().unwrap().to_string();
    };

    merkleroot(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn merkleroot_test() {
        let txids = vec![
            "8c14f0db3df150123e6f3dbbf30f8b955a8249b62ac1d1ff16284aefa3d06d87",
            "fff2525b8931402dd09222c50775608f75787bd2b87e56995a7bdd30f79702c4",
            "6359f0868171b1d194cbee1af2f16ea598ae8fad666d9b012c8ed2b79a236ec4",
            "e9a66845e05d5abc0ad04ec80f774a7e585c6e8db975962d069a522137b80c1d",
        ];

        // transaction ids are in their searchable format. Reverse byte order first.
        let txids: Vec<_> = txids
            .into_iter()
            .map(|txid| hex::decode(txid).unwrap())
            .map(|bytes| bytes.into_iter().rev().collect::<Vec<u8>>())
            .map(|bytes| hex::encode(bytes))
            .collect();

        assert_eq!("6657a9252aacd5c0b2940996ecff952228c3067cc38d4885efb5a4ac4247e9f3", merkleroot(txids))
    }
}
