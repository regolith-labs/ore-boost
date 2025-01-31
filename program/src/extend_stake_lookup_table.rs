use ore_boost_api::prelude::*;
use solana_program::address_lookup_table;
use steel::*;

pub fn process_extend_stake_lookup_table(
    accounts: &[AccountInfo<'_>],
    _data: &[u8],
) -> ProgramResult {
    // Load accounts.
    let [signer_info, stake_lookup_table_info, lookup_table_info, boost_info, stake_info, system_program, lookup_table_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    boost_info.as_account::<Boost>(&ore_boost_api::ID)?;
    let stake = stake_info
        .as_account_mut::<Stake>(&ore_boost_api::ID)?
        .assert_mut(|s| s.boost == *boost_info.key)?;
    let lut_id = find_stake_lookup_table_id(stake.id).to_le_bytes();
    let stake_lookup_table_seeds = vec![
        STAKE_LOOKUP_TABLE,
        boost_info.key.as_ref(),
        lut_id.as_slice(),
    ];
    let stake_lookup_table = stake_lookup_table_info
        .has_seeds(stake_lookup_table_seeds.as_slice(), &ore_boost_api::ID)?
        .as_account::<StakeLookupTable>(&ore_boost_api::ID)?;
    lookup_table_info.has_address(&stake_lookup_table.lookup_table)?;

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
