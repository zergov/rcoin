use crate::block::Block;

pub fn to_json(block: &Block) -> String {
    serde_json::to_string(&serialize_to_json(block)).unwrap()
}

pub fn to_json_pretty(block: &Block) -> String {
    serde_json::to_string_pretty(&serialize_to_json(block)).unwrap()
}

fn serialize_to_json(block: &Block) -> serde_json::Value {
    serde_json::json!({
        "hash": block.hash_hex(),
        "version": block.header.version,
        "previous_block_hash": block.prev_block_hash_hex(),
        "merkleroot": hex::encode(block.header.merkle_root.to_be_bytes()),
        "bits": block.header.bits,
        "nounce": block.header.nounce,
        "tx": block.transactions.iter().map(|tx| tx.id()).collect::<Vec<String>>(),
    })
}
