# zkvm-sample

A simple Zero-Knowledge Proof example using RISC Zero zkVM.

This project demonstrates how to prove the sum of two numbers without revealing the inputs.

## How it works
- The guest program runs inside the zkVM and commits the sum.
- A proof is generated that the sum is correct.
- The proof can be verified on-chain or off-chain to confirm correctness.

## Files
- `guest/src/main.rs` — guest program for zkVM.
- `host/src/main.rs` — host program that runs the guest and generates proof.

## Setup
1. Install Rust and the RISC Zero SDK.
2. Build the guest program.
3. Run the host to generate and verify proof.
