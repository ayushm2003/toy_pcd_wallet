# Toy PCD Wallet

A minimal simulation of a wallet that maintains proof-carrying state across blocks. Instead of rescanning or rederiving from scratch each time, the wallet carries a short digest (“PCD”) that binds the previous digest, the applied block delta, and the wallet’s new commitment set. Inspired by [this](https://seanbowe.com/blog/tachyon-scaling-zcash-oblivious-synchronization/) blog post.

## What this demonstrates

* State as PCD:
Each transition computes
`proof_{h+1} = H(proof_h || serialize(block_diff) || wallet_commitment_{h+1})`
so the tip proof attests to the entire history by induction.

* Wallet state update:
The wallet holds a set of note commitments. For a new block (a `BlockDelta`), it removes spent commitments, adds newly created ones and recomputes the PCD digest.

## Run

```bash
cargo run
```

You should see a single 3-block run where:

1. An owned commitment appears alongside watch-only commitments.

2. At h=2, a nullifier derived from its ρ spends the owned commitment (it disappears).

3. The proof updates and each hop verifies, and `verify_chain` returns true.

## Tests

```bash
cargo test
```

Verifies commitment determinism, state transitions, proof chaining, spending behavior and chain verification.

## Limitations

No proving systems or Merkle trees here; the “PCD” is a hash chain (Keccak-256) used as a stand-in for recursive proofs. Nullifiers for unowned notes are ignored for removal but still affect the digest because the block diff is committed.