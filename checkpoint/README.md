* Checkpoint Worker

- TODO
-- GPA for all boosts and spawn a thread per.

- Lookup Tables
-- Creates and extends Lookup Tables dynamically, between checkpoints.
-- Persists Lookup Table pubkeys to a local config file.

- Retries
-- Robust retries logic / error handling

- Twilio API
-- Uses [twilio](https://www.twilio.com/docs/usage/api) for sending a text to an admin whatsapp account, if the worker stalls

- Helius API
-- Uses the [helius smart transaction sdk](https://github.com/helius-labs/helius-rust-sdk) for priority fee quoting, CU estimation, stake-weighed connection, and confirmation polling.

** Run
```sh
# don't worry about the twilio env vars for local testing / running. for production use to notify when the worker is stalling.
HELIUS_API_KEY="" HELIUS_CLUSTER="mainnet-staked" KEYPAIR_PATH="" LUTS_PATH="./cache" MINT="" TWILIO_ACCOUNT_SID="" TWILIO_AUTH_TOKEN="" TWILIO_FROM="" TWILIO_TO="" RUST_LOG="info" cargo run
```
