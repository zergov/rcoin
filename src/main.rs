use std::fs;
use std::path::Path;

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
    Keys { path: String },
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
                WalletCommands::Keys{ path } => show_wallet_keys(path),
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

fn show_wallet_keys(path: &String) {
    let path = Path::new(path);
    let private_key_pem = fs::read(path.join("key")).expect("could not read private key file.");
    let keychain = rcoin::keys::Keychain::from_pem(&private_key_pem);

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

    println!("current target difficulty:\t{}", target);
    println!("current target difficulty hex:\t{}", hex::encode(target.to_be_bytes()));
    println!("");

    let miner = rcoin::miner::new();
    let mut previous_block = rcoin::block::genesis();

    println!("starting miner...");

    loop {
        let new_block = miner.next(&previous_block, target_bits);

        println!("");
        println!("--- block found! ---");
        println!("{}", rcoin::serializers::block_serializer::to_json_pretty(&new_block));

        previous_block = new_block;
    }
}
