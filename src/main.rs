use toy_pcd_wallet::*;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let mut state = WalletState {
        anchor_height: 0,
        notes: vec![
            Note {
                commitment: "note_a".into(),
            },
            Note {
                commitment: "note_b".into(),
            },
        ],
        proof: hash_bytes(b"genesis"),
    };

    let deltas = vec![
        BlockDelta {
            height: 1,
            new_notes: vec![Note {
                commitment: "note_1".into(),
            }],
            nullifiers: vec![],
        },
        BlockDelta {
            height: 2,
            new_notes: vec![Note {
                commitment: "note_2".into(),
            }],
            nullifiers: vec!["note_a".into()],
        },
        BlockDelta {
            height: 3,
            new_notes: vec![Note {
                commitment: "note_3".into(),
            }],
            nullifiers: vec![],
        },
    ];

	let mut states = vec![state.clone()];

	for d in &deltas {
		let next = apply_block(&state, d)?;
		let state_transition = verify_transition(&state, &next, &delta);
		println!("h={} ok, proof={}, verified={}", d.height, next.proof);
		state = next;
		states.push(state.clone());
	}

	println!("verify_chain: {}", verify_chain(&states, &deltas));	

	Ok(())
}
