use std::str::FromStr;

use solana_sdk::signer::Signer;
use steel::Pubkey;

use crate::{args::OpenArgs, Cli};

impl Cli {
    pub async fn open(&self, args: OpenArgs) {
        let signer = self.signer();
        let mint = Pubkey::from_str(&args.mint).unwrap();
        let ix = ore_boost_api::sdk::open(signer.pubkey(), signer.pubkey(), mint);
        let sig = self.send_and_confirm(ix).await.unwrap();
        println!("sig: {}", sig);
    }
}
