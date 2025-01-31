use ore_boost_api::prelude::*;
use solana_program::{address_lookup_table, log::sol_log};
use steel::*;

/// Inserts stake account into lookup table.
pub fn process_extend_stake_lookup_table(
    accounts: &[AccountInfo<'_>],
    _data: &[u8],
) -> ProgramResult {
    // Load accounts.
    let [signer_info, boost_info, stake_info, stake_lookup_table_info, lookup_table_info, system_program, lookup_table_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    sol_log("boost");
    boost_info.as_account::<Boost>(&ore_boost_api::ID)?;
    sol_log("stake");
    let stake = stake_info
        .as_account::<Stake>(&ore_boost_api::ID)?
        .assert(|s| s.boost == *boost_info.key)?;
    system_program.is_program(&system_program::ID)?;

    // Extend lookup table.
    extend_stake_lookup_table(
        signer_info,
        boost_info,
        stake_info,
        stake,
        stake_lookup_table_info,
        lookup_table_info,
        system_program,
        lookup_table_program,
    )
}

/// Inserts stake account into lookup table.
///
/// Proccessor (instruction) is responsible for validating the
/// 1) signer
/// 2) stake account
/// 3) boost account
/// 4) system program
///
/// The legacy "open" stake account instruction did not insert into a lookup table.
/// So there is a stand alone processor for inserting these legacy stake accounts,
/// while new stake accounts are atomically inserted into a lookup table on creation.
#[allow(clippy::too_many_arguments)]
pub fn extend_stake_lookup_table<'a>(
    signer_info: &AccountInfo<'a>,
    boost_info: &AccountInfo<'a>,
    stake_info: &AccountInfo<'a>,
    stake: &Stake,
    stake_lookup_table_info: &AccountInfo<'a>,
    lookup_table_info: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    lookup_table_program: &AccountInfo<'a>,
) -> ProgramResult {
    // Validate lookup table.
    let lut_id = find_stake_lookup_table_id(stake.id).to_le_bytes();
    sol_log(format!("lut id: {:?}", lut_id).as_str());
    let stake_lookup_table_seeds = vec![
        STAKE_LOOKUP_TABLE,
        boost_info.key.as_ref(),
        lut_id.as_slice(),
    ];
    sol_log("stake lookup table");
    let stake_lookup_table = stake_lookup_table_info
        .has_seeds(stake_lookup_table_seeds.as_slice(), &ore_boost_api::ID)?
        .as_account::<StakeLookupTable>(&ore_boost_api::ID)?;
    sol_log("lookup table");
    lookup_table_info.has_address(&stake_lookup_table.lookup_table)?;
    lookup_table_program.is_program(&address_lookup_table::program::ID)?;

    sol_log("extend lookup table");
    // Extend lookup table.
    let extend_lookup_table_ix = address_lookup_table::instruction::extend_lookup_table(
        *lookup_table_info.key,
        *stake_lookup_table_info.key,
        Some(*signer_info.key),
        vec![*stake_info.key],
    );
    let extend_lookup_table_accounts = vec![
        lookup_table_info.clone(),
        stake_lookup_table_info.clone(),
        signer_info.clone(),
        system_program.clone(),
        lookup_table_program.clone(),
    ];
    invoke_signed_with_bump(
        &extend_lookup_table_ix,
        extend_lookup_table_accounts.as_slice(),
        stake_lookup_table_seeds.as_slice(),
        stake_lookup_table.bump,
    )?;

    Ok(())
}
