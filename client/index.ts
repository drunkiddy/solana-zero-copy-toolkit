import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

export class ToolkitClient {
    constructor(private program: Program<any>) {}
    async initialize(owner: anchor.web3.Keypair, data: number) {
        // Implementation of initialize instruction
    }
}
