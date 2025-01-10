use ore_boost_api::consts::BOOST;
use ore_boost_api::instruction::Claim;
use ore_boost_api::state::{Boost, Stake};
use steel::*;

/// Claim distributes rewards to a staker.
pub fn process_claim(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Claim::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts
    let [signer_info, beneficiary_info, boost_info, boost_rewards_info, stake_info, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    beneficiary_info
        .is_writable()?
        .as_token_account()?
        .assert(|t| t.mint == ore_api::consts::MINT_ADDRESS)?;
    let boost = boost_info
        .as_account::<Boost>(&ore_boost_api::ID)?;
    boost_rewards_info
        .is_writable()?
        .as_token_account()?
        .assert(|t| t.mint == ore_api::consts::MINT_ADDRESS)?
        .assert(|t| t.owner == *boost_info.key)?
        .assert(|t| t.amount >= amount)?;
    let stake = stake_info
        .as_account_mut::<Stake>(&ore_boost_api::ID)?
        .assert_mut(|s| s.authority == *signer_info.key)?
        .assert_mut(|s| s.boost == *boost_info.key)?
        .assert_mut(|s| s.rewards >= amount)?;
    token_program.is_program(&spl_token::ID)?;

    // Update rewards.
    stake.rewards = stake.rewards.checked_sub(amount).unwrap();

    // Transfer tokens from boost to beneficiary.
    transfer_signed(
        boost_info,
        boost_rewards_info,
        beneficiary_info,
        token_program,
        amount,
        &[BOOST, boost.mint.as_ref()]
    )?;

    Ok(())
}
