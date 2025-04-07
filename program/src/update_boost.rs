use ore_api::state::Proof;
use ore_boost_api::{
    consts::CONFIG,
    instruction::UpdateBoost,
    state::{Boost, Config},
};
use steel::*;

/// UpdateBoost updates the multiplier or expiry date on a boost.
pub fn process_update_boost(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = UpdateBoost::try_from_bytes(data)?;
    let weight = u64::from_le_bytes(args.weight);
    let expires_at = i64::from_le_bytes(args.expires_at);

    // Load accounts.
    let [signer_info, boost_info, config_info, proof_info, rewards_info, treasury_info, treasury_tokens_info, ore_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let boost = boost_info.as_account_mut::<Boost>(&ore_boost_api::ID)?;
    let config = config_info
        .as_account_mut::<Config>(&ore_boost_api::ID)?
        .assert_mut(|c| c.admin == *signer_info.key)?;
    let proof = proof_info
        .as_account::<Proof>(&ore_api::ID)?
        .assert(|c| c.authority == *config_info.key)?;
    rewards_info
        .is_writable()?
        .as_associated_token_account(config_info.key, &ore_api::consts::MINT_ADDRESS)?;
    treasury_info.has_address(&ore_api::consts::TREASURY_ADDRESS)?;
    treasury_tokens_info.has_address(&ore_api::consts::TREASURY_TOKENS_ADDRESS)?;
    ore_program.is_program(&ore_api::ID)?;
    token_program.is_program(&spl_token::ID)?;

    // Collect rewards
    boost.collect_rewards(config, &proof);

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

    // Update the boost multiplier.
    let old_weight = boost.weight;
    boost.weight = weight;
    boost.expires_at = expires_at;

    // Update the total weight.
    if config.boosts.contains(boost_info.key) {
        if weight > old_weight {
            config.total_weight += weight - old_weight;
        } else {
            config.total_weight -= old_weight - weight;
        }
    }

    Ok(())
}
