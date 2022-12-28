use clap::{Args, Parser, Subcommand};
use std::fs;
use std::path::Path;

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
    Wallet(Wallet)
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

fn main() {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::Wallet(wallet) => {
            match &wallet.commands {
                WalletCommands::New{ path } => create_wallet(path),
                WalletCommands::Info{ path } => show_wallet_info(path),
            }
        }
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
