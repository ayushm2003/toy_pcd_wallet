use anyhow::Result;
use serde::{Deserialize, Serialize};
use tiny_keccak::{Hasher, Keccak};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Note {
    id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct BlockDelta {
    height: u64,
    new_notes: Vec<Note>,
    nullifiers: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct WalletState {
	anchor_height: u64,
	notes: Vec<Note>,
	proof: String,
}

pub fn hash_bytes(input: &[u8]) -> String {
    let mut keccak = Keccak::v256();
    let mut hash = [0u8; 32];
    keccak.update(input);
    keccak.finalize(&mut hash);

    hex::encode(hash)
}

pub fn wallet_commitment(notes: &[Note]) -> String {
    unimplemented!()
}

pub fn apply_block(prev: &WalletState, delta: &BlockDelta) -> Result<WalletState> {
	unimplemented!()
}

pub fn verify_transaction(prev: &WalletState, next: &WalletState, delta: &BlockDelta) -> bool {
	unimplemented!()
}

pub fn verify_chain(states: &[WalletState], deltas: &[BlockDelta]) -> bool {
	unimplemented!()
}