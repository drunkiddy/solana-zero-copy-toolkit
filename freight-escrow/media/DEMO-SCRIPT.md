# Product demo recording plan — under 3 minutes

Show `demo/index.html` in a browser, with the simulation notice visible. This is a reference UI, not a deployed Solana application. No wallet connection or blockchain confirmation should be implied.

0:00–0:20: Introduce FOKS and the simulation notice. Explain the three roles and 2,500 demo-token load.
0:20–0:45: Click Fund load. Show Funded and 2,500 reserved. Attempt Accept load as Shipper to show the authorization block.
0:45–1:10: Select Carrier and accept. Select Shipper and approve delivery. Show Settled, zero vault and 2,500 paid to carrier. Try a second payout to show the state block.
1:10–1:55: Reset, fund, select Carrier and accept, raise dispute. Show Disputed. Select Arbitrator and resolve with a 2,000 award. Show 2,000 to carrier and 500 to shipper.
1:55–2:20: Reset, fund, advance past expiry and refund as Shipper. Show Refunded and full principal returned.
2:20–2:40: State that Rust host compilation and eight model tests pass; validator tests, SBF build, wallet UI, deployment and audit remain pending. Explain that real delivery approval and arbitrator trust are off-chain assumptions.

Upload to YouTube, Loom or Vimeo with access available to judges. The pitch video is a separate recording.
