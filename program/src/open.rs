use ore_boost_api::{
    consts::STAKE,
    state::{Boost, Stake},
};
use solana_program::system_program;
use steel::*;

/// Open creates a new stake account.
pub fn process_open(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, payer_info, boost_info, mint_info, stake_info, system_program] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    payer_info.is_signer()?;
    boost_info
        .as_account::<Boost>(&ore_boost_api::ID)?
        .assert(|b| b.mint == *mint_info.key)?;
    mint_info.as_mint()?;
    stake_info.is_empty()?.is_writable()?.has_seeds(
        &[STAKE, signer_info.key.as_ref(), boost_info.key.as_ref()],
        &ore_boost_api::ID,
    )?;
    system_program.is_program(&system_program::ID)?;

    // Initialize the stake account.
    create_account::<Stake>(
        stake_info,
        system_program,
        payer_info,
        &ore_boost_api::ID,
        &[STAKE, signer_info.key.as_ref(), boost_info.key.as_ref()],
    )?;
    let clock = Clock::get()?;
    let stake = stake_info.as_account_mut::<Stake>(&ore_boost_api::ID)?;
    stake.authority = *signer_info.key;
    stake.balance = 0;
    stake.boost = *boost_info.key;
    stake.last_stake_at = clock.unix_timestamp;

    Ok(())
}
