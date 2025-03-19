# ORE Boost

**ORE Boost is a liquidity mining program for earning yield on staked assets.**


## API
- [`Consts`](api/src/consts.rs) – Program constants.
- [`Error`](api/src/error.rs) – Custom program errors.
- [`Event`](api/src/error.rs) – Custom program events.
- [`Instruction`](api/src/instruction.rs) – Declared instructions and arguments.

## Instructions
User
- [`Claim`](program/src/claim.rs) – Allows a staker to claim their rewards.
- [`Close`](program/src/close.rs) – Closes a stake account.
- [`Deposit`](program/src/deposit.rs) – Deposits tokens into a stake account.
- [`Open`](program/src/open.rs) – Opens a new stake account.
- [`Rotate`](program/src/rotate.rs) – Rotates the reservation to a randomly selected boost according to their unclaimed ORE weight.
- [`Withdraw`](program/src/withdraw.rs) – Withdraws tokens from a stake account.

Admin  
- [`Activate`](program/src/activate.rs) – Activate adds a boost to the directory.
- [`Deactivate`](program/src/deactivate.rs) – Removes a boost from the directory.
- [`Initialize`](program/src/initialize.rs) – Initializes the program and creates the global accounts.
- [`New`](program/src/new.rs) – Creates a new boost account.
- [`UpdateAdmin`](program/src/update_admin.rs) – Updates the admin key.
- [`UpdateBoost`](program/src/update_boost.rs) – Updates the data on a boost.

## State
 - [`Boost`](api/src/state/boost.rs) - An account (1 per mint) which tracks the priority, deposits, and rewards of a staking incentive.
 - [`Config`](api/src/state/config.rs) – A singleton account which manages program-wide variables.
 - [`Stake`](api/src/state/stake.rs) - An account (1 per user per mint) which records how many tokens of a given mint a user has staked. 

## Tests

To run the test suite, use the Solana toolchain: 

```
cargo test-sbf
```

For line coverage, use llvm-cov:

```
cargo llvm-cov
```
