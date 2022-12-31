use std::time::SystemTime;

use crate::block::{Block, Header};
use crate::difficulty;
use crate::merkleroot;
use crate::transactions::{Transaction, TxIn, TxOut};
use crate::u256;

pub struct Miner {}

pub fn new() -> Miner { Miner{} }

impl Miner {
    pub fn next(&self, previous_block: &Block, target_bits: u32) -> Block {
        let target = difficulty::bits_to_target(target_bits);
        let mut candidate_block = self.build_candidate_block(previous_block, target_bits);

        loop {
            candidate_block.header.nounce += 1;

            if candidate_block.hash() < target {
                return candidate_block;
            }
        }
    }

    fn build_coinbase_transaction(&self) -> Transaction {
        Transaction {
            version: 1,
            inputs: vec![
                TxIn {
                    txid: u256::new(0),
                    vout: 0xffffffff,
                    script_sig: hex::encode("rcoin miner"),
                    sequence: 0xffffffff,
                }
            ],
            outputs: vec![
                TxOut {
                    value: 25 * 100_000_000,
                    script_pub_key: hex::encode("todo: locking script :)")
                }
            ],
            lock_time: 0,
        }
    }

    fn build_candidate_block(&self, previous_block: &Block, target_bits: u32) -> Block {
        let transactions = vec![self.build_coinbase_transaction()];
        let merkleroot = merkleroot::from_transactions(&transactions);
        let merkleroot = hex::decode(merkleroot).unwrap();

        Block {
            header: Header {
                version: 0x1,
                prev_block_hash: previous_block.hash(),
                merkle_root: u256::from_le_bytes(merkleroot.try_into().unwrap()),
                time: current_time(),
                bits: target_bits,
                nounce: 0,
            },
            transactions,
        }
    }
}

fn current_time() -> u32 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}
