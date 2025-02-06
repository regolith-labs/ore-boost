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
pub struct OpenArgs {
    pub mint: String,
}

#[derive(Parser, Debug)]
pub struct UpdateBoostArgs {
    pub mint: String,

    #[arg(long, short, value_name = "UNIX_TIME")]
    pub expires_at: Option<i64>,

    #[arg(long, short, value_name = "MULTIPLIER")]
    pub multiplier: Option<u64>,
}

#[derive(Parser, Debug)]
pub struct GetBoostArgs {
    pub mint: String,
}

#[derive(Parser, Debug)]
pub struct DeactivateArgs {
    pub mint: String,
}

#[derive(Parser, Debug)]
pub struct DirectoryArgs {}
