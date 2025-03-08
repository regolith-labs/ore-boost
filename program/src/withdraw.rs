use ore_api::state::Proof;
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
    let clock = Clock::get()?;
    let [signer_info, beneficiary_info, boost_info, boost_deposits_info, boost_proof_info, boost_rewards_info, mint_info, stake_info, treasury_info, treasury_tokens_info, ore_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    beneficiary_info
        .is_writable()?
        .as_token_account()?
        .assert(|t| t.mint() == *mint_info.key)?;
    let boost = boost_info
        .as_account_mut::<Boost>(&ore_boost_api::ID)?
        .assert_mut(|b| b.mint == *mint_info.key)?;
    boost_deposits_info
        .is_writable()?
        .as_associated_token_account(boost_info.key, mint_info.key)?;
    let proof = boost_proof_info
        .as_account::<Proof>(&ore_api::ID)?
        .assert(|p| p.authority == *boost_info.key)?;
    boost_rewards_info
        .is_writable()?
        .as_associated_token_account(boost_info.key, &ore_api::consts::MINT_ADDRESS)?;
    mint_info.as_mint()?;
    let stake = stake_info
        .as_account_mut::<Stake>(&ore_boost_api::ID)?
        .assert_mut(|s| s.authority == *signer_info.key)?
        .assert_mut(|s| s.boost == *boost_info.key)?;
    ore_program.is_program(&ore_api::ID)?;
    token_program.is_program(&spl_token::ID)?;

    // Accumulate personal stake rewards.
    boost.rewards_cumulative += Numeric::from_fraction(proof.balance, boost.total_deposits);
    stake.accumulate_rewards(boost, &clock);
    invoke_signed(
        &ore_api::sdk::claim(*boost_info.key, *boost_rewards_info.key, proof.balance),
        &[
            boost_info.clone(),
            boost_rewards_info.clone(),
            boost_proof_info.clone(),
            treasury_info.clone(),
            treasury_tokens_info.clone(),
            token_program.clone(),
            ore_program.clone(),
        ],
        &ore_boost_api::ID,
        &[BOOST, boost.mint.as_ref()],
    )?;

    // Withdraw deposits to beneficiary.
    stake.balance -= amount;
    boost.total_deposits -= amount;
    transfer_signed(
        boost_info,
        boost_deposits_info,
        beneficiary_info,
        token_program,
        amount,
        &[BOOST, mint_info.key.as_ref()],
    )?;

    Ok(())
}
