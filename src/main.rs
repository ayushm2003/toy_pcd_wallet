use std::collections::HashMap;
use toy_pcd_wallet::*;

fn main() -> anyhow::Result<()> {
    let mut state = WalletState {
        anchor_height: 0,
        notes: vec![
            NoteCommitment {
                commitment: "note_a".into(),
            },
            NoteCommitment {
                commitment: "note_b".into(),
            },
        ],
        proof: hash_bytes(b"genesis"),
        secrets: HashMap::new(),
    };

    let owned = new_owned_note(&mut state);
    state.notes.push(owned.clone());
    let rho = state.secrets.get(&owned.commitment).expect("rho present");
    let nf_owned = nf_from_rho(rho);

    let deltas = vec![
        BlockDelta {
            height: 1,
            new_notes: vec![NoteCommitment {
                commitment: "note_1".into(),
            }],
            nullifiers: vec![],
        },
        BlockDelta {
            height: 2,
            new_notes: vec![NoteCommitment {
                commitment: "note_2".into(),
            }],
            nullifiers: vec![nf_owned.clone()],
        },
        BlockDelta {
            height: 3,
            new_notes: vec![NoteCommitment {
                commitment: "note_3".into(),
            }],
            nullifiers: vec![],
        },
    ];

    let mut states = vec![state.clone()];

    for d in &deltas {
        let next = apply_block(&state, d)?;
        let proof_verified: bool = verify_transition(&state, &next, d);
        println!(
            "h={}, notes={:?}, proof={}, proof_verified={}",
            d.height,
            ids(&next),
            &next.proof[..8],
            proof_verified
        );
        state = next;
        states.push(state.clone());
    }

    println!("verify_chain: {}", verify_chain(&states, &deltas));
    Ok(())
}

fn ids(s: &WalletState) -> Vec<&str> {
    let mut v: Vec<&str> = s.notes.iter().map(|n| n.commitment.as_str()).collect();
    v.sort_unstable();
    v
}
