use toy_pcd_wallet::*;

#[test]
fn test_genesis_and_block1_compile() {
    let genesis = WalletState {
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
    let d1 = BlockDelta {
        height: 1,
        new_notes: vec![Note {
            commitment: "note_1".into(),
        }],
        nullifiers: vec![],
    };

    let _ = apply_block(&genesis, &d1);
}

#[test]
fn test_commitment_stable() {
    let c1 = wallet_commitment(&[
        Note {
            commitment: "b".into(),
        },
        Note {
            commitment: "a".into(),
        },
    ]);
    let c2 = wallet_commitment(&[
        Note {
            commitment: "a".into(),
        },
        Note {
            commitment: "b".into(),
        },
    ]);
    assert_eq!(c1, c2);
}
