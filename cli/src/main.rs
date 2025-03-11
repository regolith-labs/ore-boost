mod args;
mod boost;
mod deactivate;
mod initialize;
mod new;
mod update_boost;

use std::sync::Arc;

use args::*;
use clap::{command, Parser, Subcommand};
use solana_client::{client_error::Result as ClientResult, nonblocking::rpc_client::RpcClient};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    compute_budget::ComputeBudgetInstruction,
    instruction::Instruction,
    signature::{read_keypair_file, Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};

struct Cli {
    pub keypair_filepath: Option<String>,
    pub rpc_client: Arc<RpcClient>,
}

#[derive(Parser, Debug)]
#[command(about, version)]
struct Args {
    #[arg(
        long,
        value_name = "NETWORK_URL",
        help = "Network address of your RPC provider",
        global = true
    )]
    rpc: Option<String>,

    #[clap(
        global = true,
        short = 'C',
        long = "config",
        id = "PATH",
        help = "Filepath to config file."
    )]
    pub config_file: Option<String>,

    #[arg(
        long,
        value_name = "KEYPAIR_FILEPATH",
        help = "Filepath to keypair to use",
        global = true
    )]
    keypair: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Fetch a boost account")]
    Boost(GetBoostArgs),

    #[command(about = "Update a boost")]
    UpdateBoost(UpdateBoostArgs),

    #[command(about = "Create a new boost")]
    New(NewArgs),

    #[command(about = "Initialize the boost program")]
    Initialize(InitializeArgs),

    #[command(about = "Deactivate a boost")]
    Deactivate(DeactivateArgs),

    #[command(about = "Fetch the config")]
    Config(ConfigArgs),
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Load the config file from custom path, the default path, or use default config values
    let cli_config = if let Some(config_file) = &args.config_file {
        solana_cli_config::Config::load(config_file).unwrap_or_else(|_| {
            eprintln!("error: Could not find config file `{}`", config_file);
            std::process::exit(1);
        })
    } else if let Some(config_file) = &*solana_cli_config::CONFIG_FILE {
        solana_cli_config::Config::load(config_file).unwrap_or_default()
    } else {
        solana_cli_config::Config::default()
    };

    // Initialize client
    let cluster = args.rpc.unwrap_or(cli_config.json_rpc_url);
    let default_keypair = args.keypair.unwrap_or(cli_config.keypair_path);
    let rpc_client = RpcClient::new_with_commitment(cluster, CommitmentConfig::confirmed());
    let cli = Arc::new(Cli::new(Arc::new(rpc_client), Some(default_keypair)));

    // Execute user command
    match args.command {
        Commands::Boost(args) => {
            cli.boost(args).await.unwrap();
        }
        Commands::Initialize(_) => {
            cli.initialize().await.unwrap();
        }
        Commands::New(args) => {
            cli.new_boost(args).await.unwrap();
        }
        Commands::UpdateBoost(args) => {
            cli.update_boost(args).await.unwrap();
        }
        Commands::Deactivate(args) => {
            cli.deactivate(args).await.unwrap();
        }
        Commands::Config(_) => {
            cli.config().await.unwrap();
        }
    };
}

impl Cli {
    pub fn new(rpc_client: Arc<RpcClient>, keypair_filepath: Option<String>) -> Self {
        Self {
            rpc_client,
            keypair_filepath,
        }
    }

    pub fn signer(&self) -> Keypair {
        match self.keypair_filepath.clone() {
            Some(filepath) => read_keypair_file(filepath).unwrap(),
            None => panic!("No keypair provided"),
        }
    }

    pub async fn send_and_confirm(&self, ix: Instruction) -> ClientResult<Signature> {
        let signer = self.signer();
        let client = self.rpc_client.clone();
        let compute_budget_ix = ComputeBudgetInstruction::set_compute_unit_limit(1_000_000);
        let compute_price_ix = ComputeBudgetInstruction::set_compute_unit_price(100_000);
        let mut tx = Transaction::new_with_payer(
            &[compute_budget_ix, compute_price_ix, ix],
            Some(&signer.pubkey()),
        );
        let blockhash = client.get_latest_blockhash().await?;
        tx.sign(&[&signer], blockhash);
        client.send_transaction(&tx).await
    }
}
