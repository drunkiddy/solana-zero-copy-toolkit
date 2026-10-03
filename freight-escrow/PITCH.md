# FOKS Freight Escrow — Pitch Outline

## 1. The problem
Small carriers commit time and operating costs before payment. Shippers need a clear, agreed delivery-approval process. Both need visibility into reserved payment and a path for disputes.

## 2. The product
A per-load Solana escrow: reserve stablecoins, name the carrier and arbitrator, accept the order, approve payout or raise a dispute. Blockchain records funds and settlement; people verify delivery.

## 3. Who it serves
Freight operators, carriers and shippers who already agree to stablecoin settlement. Initial scope excludes fiat conversion, factoring and lending.

## 4. Workflow
Shipper funds → carrier accepts → shipper approves delivery → carrier is paid. If disputed, payout pauses until the agreed arbitrator allocates funds. If nobody accepts before expiry, the shipper can reclaim funds.

## 5. Prototype today
Anchor contract draft plus an executable reference model. Eight model tests pass. Validator integration, wallet UI and devnet deployment remain to be completed. No live funds or traction claimed.

## 6. Why an audit is central
The intended program controls user funds. A signer, token-account or state-transition flaw could misdirect them. Requested audit scope includes PDA signing, token identity, authorization, settlement arithmetic, replay and dispute controls.

## 7. Proposed next milestones
October: validator tests, devnet demo and recorded pitch. November–December: operational review and prospective-user testing. Q1 2027: external audit, remediation and a conditional limited mainnet pilot. Subsequent releases depend on pilot evidence.

## 8. Team and funding
GitHub owner: @drunkiddy. Public founder identity, team roles, time commitment, financing history and fundraising plan are awaiting founder confirmation. CertiK credits would support an independent security review; they are not development cash.
