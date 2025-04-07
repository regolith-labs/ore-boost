use ore_api::state::Proof;
use ore_boost_api::consts::CONFIG;
use ore_boost_api::instruction::Claim;
use ore_boost_api::state::{Boost, Config, Stake};
use steel::*;

/// Claim distributes rewards to a staker.
pub fn process_claim(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Claim::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts
    let clock = Clock::get()?;
    let [signer_info, beneficiary_info, boost_info, config_info, proof_info, rewards_info, stake_info, treasury_info, treasury_tokens_info, ore_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    beneficiary_info
        .is_writable()?
        .as_token_account()?
        .assert(|t| t.mint() == ore_api::consts::MINT_ADDRESS)?;
    let boost = boost_info.as_account_mut::<Boost>(&ore_boost_api::ID)?;
    let config = config_info.as_account_mut::<Config>(&ore_boost_api::ID)?;
    let proof = proof_info
        .as_account::<Proof>(&ore_api::ID)?
        .assert(|p| p.authority == *config_info.key)?;
    rewards_info
        .is_writable()?
        .as_associated_token_account(config_info.key, &ore_api::consts::MINT_ADDRESS)?;
    let stake = stake_info
        .as_account_mut::<Stake>(&ore_boost_api::ID)?
        .assert_mut(|s| s.authority == *signer_info.key)?
        .assert_mut(|s| s.boost == *boost_info.key)?;
    treasury_info.has_address(&ore_api::consts::TREASURY_ADDRESS)?;
    treasury_tokens_info.has_address(&ore_api::consts::TREASURY_TOKENS_ADDRESS)?;
    ore_program.is_program(&ore_api::ID)?;
    token_program.is_program(&spl_token::ID)?;

    // Claim rewards.
    let amount = stake.claim(amount, boost, &clock, config, &proof);

    // Claim aggregate boost rewards.
    invoke_signed(
        &ore_api::sdk::claim(*config_info.key, *rewards_info.key, proof.balance),
        &[
            config_info.clone(),
            rewards_info.clone(),
            proof_info.clone(),
            treasury_info.clone(),
            treasury_tokens_info.clone(),
            token_program.clone(),
            ore_program.clone(),
        ],
        &ore_boost_api::ID,
        &[CONFIG],
    )?;

    // Transfer tokens to beneficiary.
    transfer_signed(
        config_info,
        rewards_info,
        beneficiary_info,
        token_program,
        amount,
        &[CONFIG],
    )?;

    Ok(())
}
