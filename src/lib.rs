use std::collections::HashMap;

use anyhow::{Result, anyhow};
use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};
use serde_json::to_vec;
use tiny_keccak::{Hasher, Keccak};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NoteCommitment {
    pub commitment: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockDelta {
    pub height: u64,
    pub new_notes: Vec<NoteCommitment>,
    pub nullifiers: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletState {
    pub anchor_height: u64,
    pub notes: Vec<NoteCommitment>,
    pub proof: String,
    pub secrets: HashMap<String, [u8; 32]>,
}

pub fn hash_bytes(input: &[u8]) -> String {
    let mut keccak = Keccak::v256();
    let mut hash = [0u8; 32];
    keccak.update(input);
    keccak.finalize(&mut hash);

    hex::encode(hash)
}

pub fn cm_from_rho(rho: &[u8; 32]) -> String {
    let mut buf = Vec::with_capacity(2 + 32);
    buf.extend_from_slice(b"cm");
    buf.extend_from_slice(rho);
    hash_bytes(&buf)
}

pub fn nf_from_rho(rho: &[u8; 32]) -> String {
    let mut buf = Vec::with_capacity(2 + 32);
    buf.extend_from_slice(b"nf");
    buf.extend_from_slice(rho);
    hash_bytes(&buf)
}

pub fn new_owned_note(state: &mut WalletState) -> NoteCommitment {
    let mut rho = [0u8; 32];
    OsRng.fill_bytes(&mut rho);

    let cm = cm_from_rho(&rho);
    state.secrets.insert(cm.clone(), rho);

    NoteCommitment { commitment: cm }
}

pub fn wallet_commitment(notes: &[NoteCommitment]) -> String {
    let mut commitments = notes
        .iter()
        .map(|note| note.commitment.as_str())
        .collect::<Vec<_>>();
    commitments.sort_unstable();
    let commitments = commitments.join(";");

    hash_bytes(commitments.as_bytes())
}

// hashing as a standin for proofs
pub fn compute_next_proof(
    prev_proof: &str,
    delta: &BlockDelta,
    next_notes: &[NoteCommitment],
) -> String {
    let mut concat: Vec<u8> = Vec::new();
    concat.extend_from_slice(prev_proof.as_bytes());
    concat.extend_from_slice(&to_vec(&delta).unwrap());
    concat.extend_from_slice(wallet_commitment(next_notes).as_bytes());

    hash_bytes(&concat)
}

pub fn apply_block(prev: &WalletState, delta: &BlockDelta) -> Result<WalletState> {
    if delta.height != prev.anchor_height + 1 {
        return Err(anyhow!(
            "non-sequential height: got {}, expected {}",
            delta.height,
            prev.anchor_height + 1
        ));
    }

    let mut next_notes: Vec<NoteCommitment> = prev.notes.clone();
    let mut next_secrets = prev.secrets.clone();

    for nf in &delta.nullifiers {
        if let Some(cm_to_remove) = prev
            .secrets
            .iter()
            .find_map(|(cm, rho)| (nf_from_rho(rho) == *nf).then(|| cm.clone()))
        {
            next_notes.retain(|n| n.commitment != cm_to_remove);
            next_secrets.remove(&cm_to_remove);
        }
    }

    next_notes.extend(delta.new_notes.clone());

    let next_proof = compute_next_proof(&prev.proof, delta, &next_notes);

    Ok(WalletState {
        anchor_height: delta.height,
        notes: next_notes,
        proof: next_proof,
        secrets: next_secrets,
    })
}

pub fn verify_transition(prev: &WalletState, next: &WalletState, delta: &BlockDelta) -> bool {
    if delta.height != prev.anchor_height + 1 || delta.height != next.anchor_height {
        return false;
    }

    let exp_proof = compute_next_proof(&prev.proof, delta, &next.notes);

    if exp_proof != next.proof {
        return false;
    }

    true
}

pub fn verify_chain(states: &[WalletState], deltas: &[BlockDelta]) -> bool {
    if states.len() != deltas.len() + 1 {
        return false;
    }

    for i in 0..deltas.len() {
        if !verify_transition(&states[i], &states[i + 1], &deltas[i]) {
            return false;
        }
    }

    true
}
