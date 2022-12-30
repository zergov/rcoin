use openssl::hash::{hash, MessageDigest};
use ethnum::{u256};

pub struct Transaction {
    pub version: u32,
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub lock_time: u32,
}

impl Transaction {
    pub fn id(&self) -> String {
        let data = self.to_bytes();
        let data = hash(MessageDigest::sha256(), &data).unwrap();
        let data = hash(MessageDigest::sha256(), &data).unwrap();
        hex::encode(data)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();

        payload.extend(self.version.to_le_bytes());

        payload.push(self.inputs.len() as u8);
        for txin in &self.inputs {
            payload.extend(txin.to_bytes());
        };

        payload.push(self.outputs.len() as u8);
        for txout in &self.outputs {
            payload.extend(txout.to_bytes());
        };

        payload.extend(self.lock_time.to_le_bytes());
        payload
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }
}

pub struct TxIn {
    pub txid: u256,
    pub vout: u32,
    pub script_sig: String,
    pub sequence: u32,
}

impl TxIn {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();

        payload.extend(self.txid.to_le_bytes());
        payload.extend(self.vout.to_le_bytes());

        let script_sig = hex::decode(&self.script_sig).unwrap();
        payload.push(script_sig.len() as u8);
        payload.extend(script_sig);

        payload.extend(self.sequence.to_le_bytes());
        payload
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }
}

pub struct TxOut {
    pub value: u64,
    pub script_pub_key: String,
}

impl TxOut {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();

        payload.extend(self.value.to_le_bytes());

        let script_pub_key = hex::decode(&self.script_pub_key).unwrap();
        payload.push(script_pub_key.len() as u8); // won't work for variable int
        payload.extend(script_pub_key);
        payload
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn txin_to_hex_test() {
        let tx = test_transaction();
        let txin = tx.inputs.get(0).unwrap();
        let expected_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9\
                            efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064ef\
                            fada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec\
                            5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba0\
                            98408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";

        assert_eq!(expected_hex, txin.to_hex())
    }

    #[test]
    pub fn txout_to_hex_test() {
        let tx = test_transaction();
        let txout = tx.outputs.get(0).unwrap();
        assert_eq!("4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac", txout.to_hex())
    }

    #[test]
    pub fn transaction_to_hex_test() {
        let tx = test_transaction();
        let expected_hex = "01000000017967a5185e907a25225574544c31f7b059c1a191d65b53dcc15\
                            54d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67\
                            684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff94\
                            8e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9\
                            a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff0\
                            14baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd36\
                            8b0e88ac00000000";

        assert_eq!(expected_hex, tx.to_hex())
    }

    #[test]
    pub fn transaction_id_test() {
        let tx = test_transaction();

        // tx id is in the searchable format. Reverse byte order to get the same txid
        let expected_txid: Vec<u8> = hex::decode("c1b4e695098210a31fe02abffe9005cffc051bbe86ff33e173155bcbdc5821e3")
            .unwrap()
            .into_iter()
            .rev()
            .collect();

        assert_eq!(hex::encode(expected_txid), tx.id());
    }

    fn test_transaction() -> Transaction {
        // example input from https://learnmeabitcoin.com/technical/input
        let txid_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc";
        let script_sig_hex = "47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925\
                              c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e905\
                              29c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e6\
                              3e5b7e3cf90835cceb19868f54f8961a825";

        let txin = TxIn{
            txid: u256::from_le_bytes(hex::decode(txid_hex).unwrap().try_into().unwrap()),
            vout: 1,
            script_sig: String::from(script_sig_hex),
            sequence: 0xffffffff,
        };

        // example output from https://learnmeabitcoin.com/technical/output
        let txout = TxOut{
            value: u64::from_le_bytes(hex::decode("4baf210000000000").unwrap().try_into().unwrap()),
            script_pub_key: String::from("76a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac"),
        };

        // transaction from https://learnmeabitcoin.com/explorer/transaction/c1b4e695098210a31fe02abffe9005cffc051bbe86ff33e173155bcbdc5821e3
        Transaction{
            version: 1,
            inputs: vec![txin],
            outputs: vec![txout],
            lock_time: 0x00,
        }
    }
}
