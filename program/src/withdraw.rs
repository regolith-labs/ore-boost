use ore_boost_api::{
    consts::BOOST,
    instruction::Withdraw,
    loaders::{load_boost, load_stake},
    state::{Boost, Stake},
};
use ore_utils::*;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    system_program,
};

/// Withdraw ...
pub fn process_withdraw(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Withdraw::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [signer, beneficiary_info, boost_info, boost_tokens_info, mint_info, stake_info, system_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_token_account(beneficiary_info, Some(signer.key), mint_info.key, true)?;
    load_boost(boost_info, mint_info.key, true)?;
    load_associated_token_account(boost_tokens_info, boost_info.key, mint_info.key, true)?;
    load_any_mint(mint_info, false)?;
    load_stake(stake_info, signer.key, true)?;
    load_program(system_program, system_program::id())?;
    load_program(token_program, spl_token::id())?;

    // Update the stake balance.
    let mut stake_data = stake_info.data.borrow_mut();
    let stake = Stake::try_from_bytes_mut(&mut stake_data)?;
    stake.balance = stake.balance.checked_sub(amount).unwrap();

    // Update the boost balance.
    let mut boost_data = boost_info.data.borrow_mut();
    let boost = Boost::try_from_bytes_mut(&mut boost_data)?;
    boost.total_stake = boost.total_stake.checked_sub(amount).unwrap();

    // Transfer tokens from signer to treasury
    let boost_bump = boost.bump as u8;
    drop(boost_data);
    transfer_signed(
        boost_info,
        boost_tokens_info,
        beneficiary_info,
        token_program,
        amount,
        &[&[BOOST, mint_info.key.as_ref(), &[boost_bump]]],
    )?;

    Ok(())
}
