# FOKS Freight Escrow

**Reserve payment before the load. Release it after delivery approval.**

A development prototype for freight payment escrow on Solana, intended for shippers and carriers who agree to settle in USDC. It includes an Anchor contract draft and a runnable, dependency-free reference model.

## What exists

- Anchor source for funding, acceptance, disputes, approved payout, arbitration and expired unaccepted-order refunds.
- Role, PDA, token-account ownership and state-transition constraints.
- Executable JavaScript reference model with eight passing test cases, including sampled conservation checks.
- Product scope, audit objectives and a proposed roadmap in `PROJECT.md`.

## What has not been verified

The Rust/Anchor program has not been compiled or run in a validator in the authoring environment, which has no Rust, Anchor or Solana toolchain. JavaScript tests validate the reference model, **not Solana execution or the Rust implementation**. There is no devnet/mainnet deployment, IDL, wallet-integrated UI, completed audit or live customer trial.

The token mint is selected at creation and must have six decimals. This allows local test tokens but does not enforce USDC identity. A production deployment must use an explicit USDC mint allowlist and display the mint address. The program ID is a development placeholder, not deployment evidence.

## Run the reference tests

Node.js 20+ is sufficient; there are no package dependencies.

```sh
npm test
```

## Contract development

Use Anchor 0.31.1 and a compatible Solana/Rust toolchain. Generate a development wallet and deployment key outside this repository; synchronize the program ID in `lib.rs` and `Anchor.toml` before building.

```sh
anchor keys sync
anchor build
```

Next, add real local-validator integration tests for each instruction and all invalid account/signer substitutions. No claim of successful Anchor build or deployment is made here.

## Trust and custody

The contract cannot independently determine physical delivery. Shipper approval and a named arbitrator remain trust assumptions. The arbitrator can allocate the entire disputed principal to either party. The initial prototype has no governance, automated timeout after acceptance or recovery for tokens donated after final settlement. The original order/vault accounts are retained to prevent order ID reuse and incur rent costs. A mutable Solana program's upgrade authority is a separate custody risk that must be addressed before mainnet.

Delivery documents and personal data belong off-chain. Store only agreement/evidence hashes on-chain. Do not use real customer funds in this development version.

## Audit scope

Review authorization, account substitution, token mint/program identity, PDA signing, settlement conservation, concurrent instruction ordering, replay, expiration boundaries, upgrade authority, frozen USDC account behavior, donations, and rent/account lifecycle. Independent audit and remediation are mainnet gates.

## Competition status

Prepared for consideration for Crypto World's Fair and CertiK audit credits. No submission or organizer endorsement has been obtained. Founder/team, fundraising and contact details must be confirmed before submission. The parent toolkit README contains earlier performance claims that are not validated by this freight prototype.
