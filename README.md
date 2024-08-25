# ORE Boost

**ORE Boost is a proof-of-stake community rewards system for ORE mining.**

This program allows users to stake community tokens to earn extra ORE while mining.

An admin can register new tokens and assign a boost amount for stakers of the token.


## API
- [`Consts`](api/src/consts.rs) – Program constants.
- [`Error`](api/src/error.rs) – Custom program errors.
- [`Event`](api/src/error.rs) – Custom program events.
- [`Instruction`](api/src/instruction.rs) – Declared instructions and arguments.

## Instructions
- [`Initialize`](program/src/initialize.rs) – Initializes the program and creates the global accounts.
- [`New`](program/src/new.rs) – New ...
- [`Open`](program/src/new.rs) – Open ...
- [`Stake`](program/src/new.rs) – Stake ...
- [`Unstake`](program/src/new.rs) – Unstake ...
- [`Update`](program/src/update.rs) – Update ...

## State
 - [`Boost`](api/src/state/boost.rs) - ...
 - [`Config`](api/src/state/config.rs) – A singleton account which manages program-wide variables.
 - [`Stake`](api/src/state/stake.rs) - ...


## Tests

To run the test suite, use the Solana toolchain: 

```
cargo test-sbf
```

For line coverage, use llvm-cov:

```
cargo llvm-cov
```
