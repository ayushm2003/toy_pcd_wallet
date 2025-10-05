use anyhow::Result;
use serde::{Deserialize, Serialize};
use tiny_keccak::{Hasher, Keccak};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Note {
    pub commitment: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockDelta {
    pub height: u64,
    pub new_notes: Vec<Note>,
    pub nullifiers: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletState {
    pub anchor_height: u64,
    pub notes: Vec<Note>,
    pub proof: String,
}

pub fn hash_bytes(input: &[u8]) -> String {
    let mut keccak = Keccak::v256();
    let mut hash = [0u8; 32];
    keccak.update(input);
    keccak.finalize(&mut hash);

    hex::encode(hash)
}

pub fn wallet_commitment(notes: &[Note]) -> String {
    let mut commitments = notes
        .iter()
        .map(|note| note.commitment.as_str())
        .collect::<Vec<_>>();
    commitments.sort_unstable();
    let commitments = commitments.join(";");

    hash_bytes(commitments.as_bytes())
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
