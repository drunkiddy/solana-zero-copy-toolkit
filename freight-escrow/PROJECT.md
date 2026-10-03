# FOKS Freight Escrow
## Fund the load. Verify delivery. Settle with confidence.

### Status — October 3, 2026
Development contract draft plus an executable reference model with eight passing tests. The Rust contract has not been compiled or run in a validator. No deployed escrow program, funded vault, completed audit, live customer integration, or measured performance is claimed. The freight-escrow directory is the new product; the parent repository contains an earlier toolkit scaffold.

### Problem and users
Independent carriers and small freight operators need visibility into whether payment is reserved before performing a load. Brokers and shippers need an agreed process for confirming delivery and resolving disputes. The initial product is intended for counterparties who already agree to settle in stablecoins; fiat conversion and factoring are outside the first release.

### Proposed product
A Solana escrow program that holds SPL USDC for an individual freight order. A shipper funds the order, a named carrier accepts it, and the shipper approves payment after reviewing delivery evidence. Either party can flag a dispute before settlement; a named arbitrator can then allocate the reserved amount between them.

Blockchain records payment reservation and settlement. It does not independently prove physical delivery. Documents stay off-chain; a hash links the signed order and evidence to the escrow without publishing driver details, freight documents, or precise locations.

### MVP boundaries
- Standard SPL Token program; current development code accepts a six-decimal mint per order. Fixed USDC mint allowlisting is a required production task.
- Unique order PDA derived from shipper and order ID; escrow PDA controls the vault.
- Immutable shipper, carrier, arbitrator, amount, and acceptance deadline.
- Atomic order creation and funding.
- Carrier acceptance before deadline.
- Shipper-approved release only after acceptance.
- Either party can raise a dispute after acceptance.
- Arbitrator resolution conserves the reserved amount.
- If never accepted, shipper can reclaim funds after the deadline.
- No unilateral refund after carrier acceptance; settlement or arbitration is required.
- No automatic delivery oracle, token launch, lending, yield, or platform withdrawal key.
- No platform fee in the MVP.

### Product milestones and proposed roadmap
Dates are targets, conditional on technical validation, security review, and founder confirmation.
1. October 2026: implement contract, client and demo; run local-validator integration and adversarial tests; prepare hackathon submission and recorded walkthrough.
2. November–December 2026: devnet pilot, freight-document workflow, usability review with prospective carrier/shipper users.
3. Q1 2027: external audit, remediation and limited mainnet pilot if audit and operational review permit.
4. Q2–Q3 2027: evaluate dispatch/TMS integration, multisig governance and additional settlement workflows based on pilot feedback.

### Security scope
Audit must cover PDA derivation, signer and role authorization, exact mint/token-program validation, vault authority, destination-account ownership, atomic funding, state transitions, expiry boundaries, arbitration limits, replay/double settlement, rent handling and unsolicited vault deposits.

### Mainnet readiness gates
Reproducible build, verified local-validator tests, real devnet transactions and explorer links, reviewed custody/dispute process, audit and remediation, limited exposure caps, monitoring and emergency operating procedures. No real customer funds before these gates.

### Founder information required before submission
Confirm public founder name and role, contact email/Telegram, team members and time commitment, actual funding status, fundraising intent and timeline, and any prior submission of the source toolkit to Colosseum.

### Competition prerequisites
CertiK's track requests a Colosseum hackathon submission link, repository access, presentation, contract scope/line count, mainnet target, roadmap, team and fundraising details. Submission must describe the actual implementation status, not this specification as completed software.
