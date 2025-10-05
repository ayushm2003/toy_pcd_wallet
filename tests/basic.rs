use toy_pcd_wallet::*;

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

#[test]
fn test_next_proof_changes_with_notes() {
    let prev = "00";
    let d = BlockDelta {
        height: 1,
        new_notes: vec![],
        nullifiers: vec![],
    };
    let n1 = vec![Note {
        commitment: "x".into(),
    }];
    let n2 = vec![Note {
        commitment: "y".into(),
    }];

    assert_ne!(
        compute_next_proof(prev, &d, &n1),
        compute_next_proof(prev, &d, &n2)
    );
}

#[test]
fn test_apply_block_spend_and_add() {
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
    let d2 = BlockDelta {
        height: 1,
        new_notes: vec![Note {
            commitment: "note_1".into(),
        }],
        nullifiers: vec!["note_a".into()],
    };
    let s1 = apply_block(&genesis, &d2).expect("seq");
    // spent note_a, kept note_b, added note_1
    let ids: Vec<String> = s1.notes.iter().map(|n| n.commitment.clone()).collect();

    assert!(ids.contains(&"note_b".to_string()));
    assert!(ids.contains(&"note_1".to_string()));
    assert!(!ids.contains(&"note_a".to_string()));
    assert_eq!(s1.anchor_height, 1);
}

#[test]
fn test_verify_chain_ok_and_tamper_fails() {
    let g = WalletState {
        anchor_height: 0,
        notes: vec![Note {
            commitment: "a".into(),
        }],
        proof: hash_bytes(b"genesis"),
    };
    let d1 = BlockDelta {
        height: 1,
        new_notes: vec![Note {
            commitment: "n1".into(),
        }],
        nullifiers: vec![],
    };

    let s1 = apply_block(&g, &d1).unwrap();

    assert!(verify_transition(&g, &s1, &d1));
    assert!(verify_chain(&[g.clone(), s1.clone()], &[d1.clone()]));

    //add a fake note
    let mut s1_bad = s1.clone();
    s1_bad.notes.push(Note {
        commitment: "evil".into(),
    });

    assert!(!verify_transition(&g, &s1_bad, &d1));
}

fn test_apply_block_height_mismatch_err() {
    let g = WalletState {
        anchor_height: 1,
        notes: vec![Note {
            commitment: "a".into(),
        }],
        proof: hash_bytes(b"genesis"),
    };
    let d = BlockDelta {
        height: 3, // should be 2
        new_notes: vec![Note {
            commitment: "b".into(),
        }],
        nullifiers: vec![],
    };

    assert!(apply_block(&g, &d).is_err());
}

#[test]
fn test_next_proof_ignores_order_of_next_notes() {
    let prev = "00";
    let d = BlockDelta {
        height: 1,
        new_notes: vec![],
        nullifiers: vec![],
    };

    let n1 = vec![
        Note {
            commitment: "x".into(),
        },
        Note {
            commitment: "y".into(),
        },
    ];
    let n2 = vec![
        Note {
            commitment: "y".into(),
        },
        Note {
            commitment: "x".into(),
        },
    ];

    let p1 = compute_next_proof(prev, &d, &n1);
    let p2 = compute_next_proof(prev, &d, &n2);

    assert_eq!(p1, p2);
}

#[test]
fn test_nullifier_not_owned_is_ignored() {
    let g = WalletState {
        anchor_height: 0,
        notes: vec![Note {
            commitment: "a".into(),
        }],
        proof: hash_bytes(b"genesis"),
    };
    let d = BlockDelta {
        height: 1,
        new_notes: vec![Note {
            commitment: "b".into(),
        }],
        nullifiers: vec!["zzz".into()], // wallet doesn't own this
    };

    let s1 = apply_block(&g, &d).expect("ok");
    let ids: Vec<_> = s1.notes.iter().map(|n| n.commitment.as_str()).collect();

    assert!(ids.contains(&"a"));
    assert!(ids.contains(&"b"));
    assert_eq!(s1.anchor_height, 1);
    assert!(verify_transition(&g, &s1, &d));
}

#[test]
fn test_verify_chain_empty_and_length_mismatch() {
    let g = WalletState {
        anchor_height: 0,
        notes: vec![],
        proof: hash_bytes(b"genesis"),
    };
    assert!(verify_chain(&[g.clone()], &[]));

    let d = BlockDelta {
        height: 1,
        new_notes: vec![],
        nullifiers: vec![],
    };
    assert!(!verify_chain(&[g.clone()], &[d]));
}
