use clap::Parser;

#[derive(Parser, Debug)]
pub struct InitializeArgs {}

#[derive(Parser, Debug)]
pub struct NewArgs {
    pub mint: String,
    pub expires_at: i64,
    pub multiplier: u64,
}

#[derive(Parser, Debug)]
pub struct GetBoostArgs {
    pub mint: String,
}