use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use helius::types::{
    Cluster, CreateSmartTransactionConfig, SmartTransaction, SmartTransactionConfig, Timeout,
};
use ore_boost_api::state::{Boost, Checkpoint, Stake};
use solana_account_decoder::UiAccountEncoding;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::{
    RpcAccountInfoConfig, RpcProgramAccountsConfig, RpcSendTransactionConfig,
};
use solana_client::rpc_filter::{Memcmp, RpcFilterType};
use solana_sdk::address_lookup_table::state::AddressLookupTable;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;
use solana_sdk::{signature::Keypair, signer::EncodableKey};
use steel::{sysvar, AccountDeserialize, Clock, Discriminator, Instruction};

use crate::error::Error::{
    EmptyJitoBundle, EmptyJitoBundleConfirmation, InvalidHeliusCluster,
    MissingHeliusSolanaAsyncClient, TooManyTransactionsInJitoBundle, UnconfirmedJitoBundle,
};

pub struct Client {
    pub rpc: helius::Helius,
    pub keypair: Arc<Keypair>,
}

impl Client {
    pub fn new() -> Result<Self> {
        let helius_api_key = helius_api_key()?;
        let helius_cluster = helius_cluster()?;
        let keypair = keypair()?;
        let rpc = helius::Helius::new_with_async_solana(helius_api_key.as_str(), helius_cluster)?;
        let client = Self {
            rpc,
            keypair: Arc::new(keypair),
        };
        Ok(client)
    }
    pub async fn send_transaction(&self, ixs: &[Instruction]) -> Result<Signature> {
        let signer = Arc::clone(&self.keypair);
        let signers: Vec<Arc<dyn Signer>> = vec![signer];
        let tx = SmartTransactionConfig::new(ixs.to_vec(), signers, Timeout::default());
        let sig = self.rpc.send_smart_transaction(tx).await?;
        Ok(sig)
    }
    #[allow(dead_code)]
    pub async fn send_transaction_with_luts(
        &self,
        ixs: &[Instruction],
        luts: &[Pubkey],
    ) -> Result<Signature> {
        let signer = Arc::clone(&self.keypair);
        let signers: Vec<Arc<dyn Signer>> = vec![signer];
        let lookup_tables = self.rpc.get_lookup_tables(luts).await?;
        let tx = CreateSmartTransactionConfig {
            instructions: ixs.to_vec(),
            signers,
            lookup_tables: Some(lookup_tables),
            fee_payer: None,
            priority_fee_cap: None,
        };
        let tx = SmartTransactionConfig {
            create_config: tx,
            send_options: RpcSendTransactionConfig::default(),
            timeout: Timeout::default(),
        };
        let sig = self.rpc.send_smart_transaction(tx).await?;
        Ok(sig)
    }
    /// returns bundle-id if confirmed
    pub async fn send_jito_bundle_with_luts(
        &self,
        ixs: &[&[Instruction]],
        luts: &[Pubkey],
    ) -> Result<String> {
        let jito_api_url = "https://mainnet.block-engine.jito.wtf/api/v1/bundles";
        if ixs.len().gt(&5) {
            return Err(anyhow::anyhow!(TooManyTransactionsInJitoBundle));
        }
        if ixs.is_empty() {
            return Err(anyhow::anyhow!(EmptyJitoBundle));
        }
        let mut transactions = vec![];
        for (index, slice) in ixs.iter().enumerate() {
            let tx = if index.eq(&(ixs.len() - 1)) {
                // last of n transactions in bundle, add tip
                self.create_jito_transaction_with_luts(slice, luts).await?
            } else {
                self.create_transaction_with_luts(slice, luts).await?
            };
            transactions.push(tx);
        }
        let bundle_id = self
            .rpc
            .send_jito_bundle(transactions, jito_api_url)
            .await?;
        log::info!("bundle id: {:?}", bundle_id);
        self.confirm_jito_bundle(bundle_id.as_str()).await?;
        Ok(bundle_id)
    }
    /// returns ok if confirmed
    pub async fn send_jito_bundle(&self, ixs: &[&[Instruction]]) -> Result<()> {
        let jito_api_url = "https://mainnet.block-engine.jito.wtf/api/v1/bundles";
        if ixs.len().gt(&5) {
            return Err(anyhow::anyhow!(TooManyTransactionsInJitoBundle));
        }
        if ixs.is_empty() {
            return Err(anyhow::anyhow!(EmptyJitoBundle));
        }
        let mut transactions = vec![];
        for (index, slice) in ixs.iter().enumerate() {
            let tx = if index.eq(&(ixs.len() - 1)) {
                // last of n transactions in bundle, add tip
                self.create_jito_transaction(slice).await?
            } else {
                self.create_transaction(slice).await?
            };
            transactions.push(tx);
        }
        let bundle_id = self
            .rpc
            .send_jito_bundle(transactions, jito_api_url)
            .await?;
        log::info!("bundle id: {:?}", bundle_id);
        self.confirm_jito_bundle(bundle_id.as_str()).await?;
        Ok(())
    }
    async fn confirm_jito_bundle(&self, bundle_id: &str) -> Result<()> {
        let mut retries = 0;
        let max_retires = 15;
        loop {
            match self
                .request_confirm_jito_bundle_inflight(bundle_id.to_string())
                .await
            {
                Ok(()) => {
                    return Ok(());
                }
                Err(err) => {
                    log::error!("{:?}", err);
                    retries += 1;
                    if retries == max_retires {
                        return Err(UnconfirmedJitoBundle).map_err(From::from);
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
    }
    async fn request_confirm_jito_bundle_inflight(&self, bundle_id: String) -> Result<()> {
        let jito_api_url = "https://mainnet.block-engine.jito.wtf/api/v1/getInflightBundleStatuses";
        let parsed_url = url::Url::parse(jito_api_url)?;
        #[derive(serde::Serialize, Debug)]
        pub struct BasicRequest {
            pub jsonrpc: String,
            pub id: u32,
            pub method: String,
            pub params: Vec<Vec<String>>,
        }
        let request: BasicRequest = BasicRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "getInflightBundleStatuses".to_string(),
            params: vec![vec![bundle_id.to_string()]],
        };
        let response: serde_json::Value = self
            .rpc
            .client
            .post(parsed_url)
            .json(&request)
            .send()
            .await
            .map_err(|err| anyhow::anyhow!(err))?
            .json::<serde_json::Value>()
            .await?;
        if let Some(error) = response.clone().get("error") {
            return Err(anyhow::anyhow!(error.to_string())).map_err(From::from);
        }
        #[derive(serde::Deserialize, Debug)]
        struct Inner {
            status: String,
        }
        #[derive(serde::Deserialize, Debug)]
        struct Middle {
            value: Vec<Inner>,
        }
        #[derive(serde::Deserialize, Debug)]
        struct Outer {
            result: Middle,
        }
        let response = serde_json::from_value(response)?;
        let response: Outer = serde_json::from_value(response)?;
        let first = response
            .result
            .value
            .first()
            .ok_or(anyhow::anyhow!(EmptyJitoBundleConfirmation))?;
        match first.status.as_str() {
            "Landed" => {
                log::info!("jito confirmation: {:?}", response);
                Ok(())
            }
            status => {
                log::info!("bundle status: {}", status);
                Err(anyhow::anyhow!(UnconfirmedJitoBundle))
            }
        }
    }
    /// returns base58 encoded transaction string
    async fn create_jito_transaction(&self, ixs: &[Instruction]) -> Result<String> {
        let signer = Arc::clone(&self.keypair);
        let signers: Vec<Arc<dyn Signer>> = vec![signer];
        let config = CreateSmartTransactionConfig::new(ixs.to_vec(), signers);
        let (tx, _) = self
            .rpc
            .create_smart_transaction_with_tip(config, Some(100_000))
            .await?;
        Ok(tx)
    }
    /// returns base58 encoded transaction string
    async fn create_transaction(&self, ixs: &[Instruction]) -> Result<String> {
        let signer = Arc::clone(&self.keypair);
        let signers: Vec<Arc<dyn Signer>> = vec![signer];
        let config = CreateSmartTransactionConfig::new(ixs.to_vec(), signers);
        let (tx, _) = self.rpc.create_smart_transaction(&config).await?;
        let bytes = match tx {
            SmartTransaction::Legacy(tx) => bincode::serialize(&tx)?,
            SmartTransaction::Versioned(tx) => bincode::serialize(&tx)?,
        };
        let string = solana_sdk::bs58::encode(bytes).into_string();
        Ok(string)
    }
    async fn create_transaction_with_luts(
        &self,
        ixs: &[Instruction],
        luts: &[Pubkey],
    ) -> Result<String> {
        let signer = Arc::clone(&self.keypair);
        let signers: Vec<Arc<dyn Signer>> = vec![signer];
        let lookup_tables = self.rpc.get_lookup_tables(luts).await?;
        let config = CreateSmartTransactionConfig {
            instructions: ixs.to_vec(),
            signers,
            lookup_tables: Some(lookup_tables),
            fee_payer: None,
            priority_fee_cap: None,
        };
        let (tx, _) = self.rpc.create_smart_transaction(&config).await?;
        let bytes = match tx {
            SmartTransaction::Legacy(tx) => bincode::serialize(&tx)?,
            SmartTransaction::Versioned(tx) => bincode::serialize(&tx)?,
        };
        let string = solana_sdk::bs58::encode(bytes).into_string();
        Ok(string)
    }
    async fn create_jito_transaction_with_luts(
        &self,
        ixs: &[Instruction],
        luts: &[Pubkey],
    ) -> Result<String> {
        let signer = Arc::clone(&self.keypair);
        let signers: Vec<Arc<dyn Signer>> = vec![signer];
        let lookup_tables = self.rpc.get_lookup_tables(luts).await?;
        let config = CreateSmartTransactionConfig {
            instructions: ixs.to_vec(),
            signers,
            lookup_tables: Some(lookup_tables),
            fee_payer: None,
            priority_fee_cap: None,
        };
        let (tx, _) = self
            .rpc
            .create_smart_transaction_with_tip(config, Some(100_000))
            .await?;
        Ok(tx)
    }
}

#[async_trait]
pub trait AsyncClient {
    fn get_async_client(&self) -> Result<Arc<RpcClient>>;
    async fn get_boost(&self, boost: &Pubkey) -> Result<Boost>;
    async fn get_boost_stake_accounts(&self, boost: &Pubkey) -> Result<Vec<(Pubkey, Stake)>>;
    async fn get_checkpoint(&self, checkpoint: &Pubkey) -> Result<Checkpoint>;
    async fn get_clock(&self) -> Result<Clock>;
    async fn get_lookup_table(&self, lut: &Pubkey) -> Result<AddressLookupTableAccount>;
    async fn get_lookup_tables(&self, luts: &[Pubkey]) -> Result<Vec<AddressLookupTableAccount>>;
}

#[async_trait]
impl AsyncClient for helius::Helius {
    fn get_async_client(&self) -> Result<Arc<RpcClient>> {
        let res = match &self.async_rpc_client {
            Some(rpc) => {
                let rpc = Arc::clone(rpc);
                Ok(rpc)
            }
            None => Err(MissingHeliusSolanaAsyncClient),
        };
        res.map_err(From::from)
    }
    async fn get_boost(&self, boost: &Pubkey) -> Result<Boost> {
        let data = self.get_async_client()?.get_account_data(boost).await?;
        let boost = Boost::try_from_bytes(data.as_slice())?;
        Ok(*boost)
    }
    async fn get_boost_stake_accounts(&self, boost: &Pubkey) -> Result<Vec<(Pubkey, Stake)>> {
        let accounts = get_program_accounts::<Stake>(
            self.get_async_client()?.as_ref(),
            &ore_boost_api::ID,
            vec![],
        )
        .await?;
        let accounts = accounts
            .into_iter()
            .filter(|(_, stake)| stake.boost.eq(boost))
            .collect();
        Ok(accounts)
    }
    async fn get_checkpoint(&self, checkpoint: &Pubkey) -> Result<Checkpoint> {
        let data = self
            .get_async_client()?
            .get_account_data(checkpoint)
            .await?;
        let checkpoint = Checkpoint::try_from_bytes(data.as_slice())?;
        Ok(*checkpoint)
    }
    async fn get_clock(&self) -> Result<Clock> {
        let data = self
            .get_async_client()?
            .get_account_data(&sysvar::clock::ID)
            .await?;
        let clock = bincode::deserialize::<Clock>(data.as_slice())?;
        Ok(clock)
    }
    async fn get_lookup_table(&self, lut: &Pubkey) -> Result<AddressLookupTableAccount> {
        let rpc = self.get_async_client()?;
        let data = rpc.get_account_data(lut).await?;
        let account = AddressLookupTable::deserialize(data.as_slice())?;
        let account = AddressLookupTableAccount {
            key: *lut,
            addresses: account.addresses.to_vec(),
        };
        Ok(account)
    }
    async fn get_lookup_tables(&self, luts: &[Pubkey]) -> Result<Vec<AddressLookupTableAccount>> {
        // need address for each account so fetch sequentially
        // get multiple accounts does not return the respective pubkeys
        let mut accounts = vec![];
        for lut in luts {
            let account = self.get_lookup_table(lut).await?;
            accounts.push(account);
        }
        Ok(accounts)
    }
}

async fn get_program_accounts<T>(
    client: &RpcClient,
    program_id: &Pubkey,
    filters: Vec<RpcFilterType>,
) -> Result<Vec<(Pubkey, T)>>
where
    T: AccountDeserialize + Discriminator + Copy,
{
    let mut all_filters = vec![RpcFilterType::Memcmp(Memcmp::new_raw_bytes(
        0,
        T::discriminator().to_le_bytes().to_vec(),
    ))];
    all_filters.extend(filters);
    let result = client
        .get_program_accounts_with_config(
            program_id,
            RpcProgramAccountsConfig {
                // filters: Some(all_filters),
                filters: None,
                account_config: RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .await?;
    let accounts = result
        .into_iter()
        .flat_map(|(pubkey, account)| {
            let account = T::try_from_bytes(&account.data)?;
            Ok::<_, anyhow::Error>((pubkey, *account))
        })
        .collect();
    Ok(accounts)
}

fn helius_api_key() -> Result<String> {
    let key = std::env::var("HELIUS_API_KEY")?;
    Ok(key)
}

fn helius_cluster() -> Result<Cluster> {
    let cluster_str = std::env::var("HELIUS_CLUSTER")?;
    let res = match cluster_str.as_str() {
        "mainnet" => Ok(Cluster::MainnetBeta),
        "mainnet-staked" => Ok(Cluster::StakedMainnetBeta),
        "devnet" => Ok(Cluster::Devnet),
        _ => Err(InvalidHeliusCluster),
    };
    res.map_err(From::from)
}

fn keypair() -> Result<Keypair> {
    let keypair_path = std::env::var("KEYPAIR_PATH")?;
    let keypair =
        Keypair::read_from_file(keypair_path).map_err(|err| anyhow::anyhow!(err.to_string()))?;
    Ok(keypair)
}
