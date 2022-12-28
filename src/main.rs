use std::fs;
use std::path::Path;
use std::time::{SystemTime};

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rcoin")]
#[command(author = "Jonathan Lalande. <lalandej.gg@gmail.com>")]
#[command(version = "0.0")]
#[command(about = "client to interact with the rcoin network.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands
}

#[derive(Subcommand)]
enum Commands {
    Wallet(Wallet),
    Miner(Miner),
}

#[derive(Args)]
struct Wallet {
    #[command(subcommand)]
    commands: WalletCommands,
}

#[derive(Subcommand)]
enum WalletCommands {
    /// Generate a new rcoin wallet at the given path.
    New { path: String },
    Info { path: String },
}

#[derive(Args)]
struct Miner {
    address: String
}

fn main() {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::Wallet(wallet) => {
            match &wallet.commands {
                WalletCommands::New{ path } => create_wallet(path),
                WalletCommands::Info{ path } => show_wallet_info(path),
            }
        },
        Commands::Miner(miner) => start_miner(miner),
    }

}

fn create_wallet(path: &String) {
    if let Err(error) = fs::create_dir(path) {
        println!("could not create wallet at: {}", path);
        println!("{}", error);
        return
    }

    let path = Path::new(path);
    let keychain = rcoin::keys::generate_keychain();

    if let Err(error) = fs::write(path.join("key"), keychain.private_key_pem()) {
        println!("could not create private key file: {}", error);
        return
    }

    if let Err(error) = fs::write(path.join("key.pub"), keychain.public_key_pem()) {
        println!("could not create public key file: {}", error);
        return
    }

    println!("Rcoin address: {}", rcoin::addresses::from_keychain(&keychain));
    println!("==================================");
    println!("private key:\t{}", keychain.private_key_hex());
    println!("public key:\t{}", keychain.public_key_hex());
}

fn show_wallet_info(path: &String) {
    let path = Path::new(path);
    let private_key_pem = fs::read(path.join("key")).expect("could not read private key file.");
    let keychain = rcoin::keys::from_pem(&private_key_pem);

    println!("Rcoin address: {}", rcoin::addresses::from_keychain(&keychain));
    println!("==================================");
    println!("private key:\t{}", keychain.private_key_hex());
    println!("public key:\t{}", keychain.public_key_hex());
}

fn start_miner(miner: &Miner) {
    println!("miner started for address: {}", miner.address);

    // 1. define a target u256 number for our blocks. (challenge is to generate a hash smaller than that)
    // 2. generate blocks until one of them has a hash smaller than that target.
    // 3. print that u256 value, its hash, and its details.
    let target_bits = 0x1e0696f4;
    let target = rcoin::difficulty::bits_to_target(target_bits);

    let previous_block = rcoin::block::genesis();
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;

    let mut candidate_block = rcoin::block::Block {
        header: rcoin::block::Header {
            version: 0x1,
            prev_block_hash: previous_block.hash(),
            merkle_root: ethnum::U256::new(0), // TODO: implement merkle root hash computation
            time: now,
            bits: target_bits,
            nounce: 0,
        },
    };

    println!("current target:\t\t{}", target);
    println!("current target hex:\t{}", hex::encode(target.to_be_bytes()));
    println!("");
    println!("mining started...");

    loop {
        candidate_block.header.nounce += 1;

        if candidate_block.hash() < target {
            break;
        }
    }

    println!("");
    println!("--- block found! ---");
    println!("hash:\t\t{}", candidate_block.hash());
    println!("hash hex:\t{}", candidate_block.hash_hex());
    println!("version: {}", candidate_block.header.version);
    println!("prev_block_hash: {}", candidate_block.header.prev_block_hash);
    println!("prev_block_hash hex: {}", candidate_block.prev_block_hash_hex());
    println!("merkle_root: {}", candidate_block.header.merkle_root);
    println!("time: {}", candidate_block.header.time);
    println!("bits: {}", candidate_block.header.bits);
    println!("nounce: {}", candidate_block.header.nounce);
}
