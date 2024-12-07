use ore_boost_api::{
    consts::BOOST,
    instruction::Withdraw,
    state::{Boost, Stake},
};
use steel::*;

/// Withdraw unstakes tokens from a stake account.
pub fn process_withdraw(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Withdraw::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [signer_info, beneficiary_info, boost_info, boost_tokens_info, mint_info, stake_info, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    beneficiary_info
        .is_writable()?
        .as_token_account()?
        .assert(|t| t.mint == *mint_info.key)?;
    let boost = boost_info
        .as_account_mut::<Boost>(&ore_boost_api::ID)?
        .assert_mut(|b| b.mint == *mint_info.key)?;
    boost_tokens_info
        .is_writable()?
        .as_associated_token_account(boost_info.key, mint_info.key)?;
    mint_info.as_mint()?;
    let stake = stake_info
        .as_account_mut::<Stake>(&ore_boost_api::ID)?
        .assert_mut(|s| s.authority == *signer_info.key)?
        .assert_mut(|s| s.boost == *boost_info.key)?;
    token_program.is_program(&spl_token::ID)?;

    // TODO Withdraw pending stake first, then committed stake.
    
    // Update the stake balance.
    stake.balance = stake.balance.checked_sub(amount).unwrap();

    // Update the boost balance.
    boost.total_stake = boost.total_stake.checked_sub(amount).unwrap();

    // Transfer tokens from signer to treasury
    transfer_signed(
        boost_info,
        boost_tokens_info,
        beneficiary_info,
        token_program,
        amount,
        &[BOOST, mint_info.key.as_ref()],
    )?;

    Ok(())
}
