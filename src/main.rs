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
    New { path: String }
}

fn main() {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::Wallet(wallet) => wallet_command(&wallet)
    }

}

fn wallet_command(wallet: &Wallet) {
    match &wallet.commands {
        WalletCommands::New{ path } => {
            println!("creating new wallet at: {}", path);
            let keychain = rcoin::keys::generate_keychain();

            println!("Rcoin address: {}", rcoin::addresses::from_keychain(&keychain));
            println!("==================================");
            println!("private key:\t{}", keychain.private_key_hex());
            println!("public key:\t{}", keychain.public_key_hex());
        }
    }
}
