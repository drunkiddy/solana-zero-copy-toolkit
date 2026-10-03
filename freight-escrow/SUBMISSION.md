# FOKS Freight Escrow — submission package

Status: draft project created in Colosseum (project 15585); neither final hackathon submission nor CertiK submission confirmed.

## CertiK description
FOKS Freight Escrow is an early Solana freight-payment escrow prototype for shippers and carriers who agree to stablecoin settlement. A shipper reserves the payment before a load, a named carrier accepts before expiry, and the shipper releases principal after delivery review. Either party can dispute an accepted order; a pre-agreed arbitrator allocates the disputed principal. An expired, unaccepted order can be refunded. Physical delivery verification and documents remain off-chain, with agreement and evidence hashes stored on-chain.

## Current implementation
Anchor 0.31.1 Rust instruction handlers and account constraints; a dependency-free JavaScript reference model; an interactive reference demo; eight passing model tests; successful Rust host cargo check. No SBF build, validator integration, wallet-integrated client, devnet/mainnet deployment or audit is claimed. The model demo simulates tokens and does not transact on-chain.

## Source
Repository: https://github.com/drunkiddy/solana-zero-copy-toolkit
Project branch/directory: https://github.com/drunkiddy/solana-zero-copy-toolkit/tree/foks-freight-escrow/freight-escrow
The parent main branch is an older scaffold. Freight work and accurate maturity disclosures are in the linked directory.

## Mainnet and roadmap
Proposed limited-mainnet target: Q1 2027, conditional on validator coverage, explicit USDC mint allowlist, verified build/deployment, independent audit and remediation, upgrade-authority policy and operational review.

- October 2026: local-validator tests, account substitution cases, devnet workflow and recorded demo.
- November–December 2026: freight-operator feedback, delivery evidence policy, arbitrator process and client iteration.
- Q1 2027: independent audit, remediation and conditional limited mainnet pilot with agreed transaction limits.
- Q2–Q3 2027: assess pilot evidence and settlement failures, improve operational tooling, and expand only after security and demand gates.

These are proposed milestones, not completed work or commitments to customer launch.

## Audit scope
Authorization and PDA signing; account substitution and recipient ownership; SPL Token mint/program identity; settlement arithmetic and conservation; state transitions, ordering, expiration and replay; arbitrator authority; upgrade authority; frozen accounts, vault donations and account lifecycle. The prototype allows a six-decimal test mint; production must restrict verified USDC mint addresses. Credits would fund independent security review, not development cash.

## Team/contact
Owner: GitHub @drunkiddy; product and freight operations. Substantial ChatGPT/Codex assistance was used for code, model, tests and submission materials.
Contact email will be supplied privately to the organizers.
Full founder name, Telegram, school/education, other team members, full-time status, funding raised and funding plans require founder confirmation. Do not invent these answers.

## Remaining submission gates
1. Confirm founder profile and required Telegram.
2. Upload logo and two videos: product demo (up to 3 minutes) and separate founder pitch (up to 2 minutes), on YouTube/Loom/Vimeo.
3. Review exact terms and obtain action-time confirmation if the final browser action accepts legal agreements.
4. Colosseum final submission opens October 6, 2026 at 4:00 AM PDT (11:00 UTC, 16:00 Tashkent). Save and review before that date; a draft is not a submitted entry.
5. Obtain final Colosseum submission link, then complete CertiK form with accurate team/funding answers, deck link and eligibility attestation. Verify success on each platform before reporting submitted.
