use std::collections::HashMap;
use toy_pcd_wallet::*;

#[test]
fn test_commitment_stable() {
    let c1 = wallet_commitment(&[
        NoteCommitment {
            commitment: "b".into(),
        },
        NoteCommitment {
            commitment: "a".into(),
        },
    ]);
    let c2 = wallet_commitment(&[
        NoteCommitment {
            commitment: "a".into(),
        },
        NoteCommitment {
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
    let n1 = vec![NoteCommitment {
        commitment: "x".into(),
    }];
    let n2 = vec![NoteCommitment {
        commitment: "y".into(),
    }];

    assert_ne!(
        compute_next_proof(prev, &d, &n1),
        compute_next_proof(prev, &d, &n2)
    );
}

#[test]
fn test_apply_block_spend_and_add() {
    let mut genesis = WalletState {
        anchor_height: 0,
        notes: vec![NoteCommitment {
            commitment: "note_b".into(),
        }],
        proof: hash_bytes(b"genesis"),
        secrets: HashMap::new(),
    };
    let owned = new_owned_note(&mut genesis);
    let rho = genesis.secrets.get(&owned.commitment).unwrap();
    let nf = nf_from_rho(rho);
    genesis.notes.push(owned.clone());

    let d1 = BlockDelta {
        height: 1,
        new_notes: vec![NoteCommitment {
            commitment: "note_1".into(),
        }],
        nullifiers: vec![nf],
    };

    let s1 = apply_block(&genesis, &d1).expect("seq");

    let ids: Vec<String> = s1.notes.iter().map(|n| n.commitment.clone()).collect();
    assert!(ids.contains(&"note_b".to_string()));
    assert!(ids.contains(&"note_1".to_string()));
    assert!(!ids.contains(&owned.commitment));
    assert_eq!(s1.anchor_height, 1);
}

#[test]
fn test_verify_chain_ok_and_tamper_fails() {
    let g = WalletState {
        anchor_height: 0,
        notes: vec![NoteCommitment {
            commitment: "a".into(),
        }],
        proof: hash_bytes(b"genesis"),
        secrets: HashMap::new(),
    };
    let d1 = BlockDelta {
        height: 1,
        new_notes: vec![NoteCommitment {
            commitment: "n1".into(),
        }],
        nullifiers: vec![],
    };

    let s1 = apply_block(&g, &d1).unwrap();

    assert!(verify_transition(&g, &s1, &d1));
    assert!(verify_chain(&[g.clone(), s1.clone()], &[d1.clone()]));

    //add a fake note
    let mut s1_bad = s1.clone();
    s1_bad.notes.push(NoteCommitment {
        commitment: "evil".into(),
    });

    assert!(!verify_transition(&g, &s1_bad, &d1));
}

#[test]
fn test_apply_block_height_mismatch_err() {
    let g = WalletState {
        anchor_height: 1,
        notes: vec![NoteCommitment {
            commitment: "a".into(),
        }],
        proof: hash_bytes(b"genesis"),
        secrets: HashMap::new(),
    };
    let d = BlockDelta {
        height: 3, // should be 2
        new_notes: vec![NoteCommitment {
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
        NoteCommitment {
            commitment: "x".into(),
        },
        NoteCommitment {
            commitment: "y".into(),
        },
    ];
    let n2 = vec![
        NoteCommitment {
            commitment: "y".into(),
        },
        NoteCommitment {
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
        notes: vec![NoteCommitment {
            commitment: "a".into(),
        }],
        proof: hash_bytes(b"genesis"),
        secrets: HashMap::new(),
    };
    let d = BlockDelta {
        height: 1,
        new_notes: vec![NoteCommitment {
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
        secrets: HashMap::new(),
    };
    assert!(verify_chain(&[g.clone()], &[]));

    let d = BlockDelta {
        height: 1,
        new_notes: vec![],
        nullifiers: vec![],
    };
    assert!(!verify_chain(&[g.clone()], &[d]));
}

#[test]
fn test_wrong_nullifier() {
    let mut g = WalletState {
        anchor_height: 0,
        notes: vec![],
        proof: hash_bytes(b"genesis"),
        secrets: HashMap::new(),
    };
    let owned = new_owned_note(&mut g);
    g.notes.push(owned.clone());

    let d = BlockDelta {
        height: 1,
        new_notes: vec![],
        nullifiers: vec!["not_the_right_nf".into()],
    };
    let s1 = apply_block(&g, &d).expect("ok");

    // owned note should still be present
    assert!(s1.notes.iter().any(|n| n.commitment == owned.commitment));
    assert!(verify_transition(&g, &s1, &d));
}

#[test]
fn test_secret_removed_on_spend() {
    let mut g = WalletState {
        anchor_height: 0,
        notes: vec![],
        proof: hash_bytes(b"genesis"),
        secrets: HashMap::new(),
    };
    let owned = new_owned_note(&mut g);
    g.notes.push(owned.clone());
    let nf = {
        let rho = g.secrets.get(&owned.commitment).unwrap();
        nf_from_rho(rho)
    };

    let d = BlockDelta {
        height: 1,
        new_notes: vec![],
        nullifiers: vec![nf],
    };
    let s1 = apply_block(&g, &d).expect("ok");

    assert!(!s1.secrets.contains_key(&owned.commitment));
    assert!(!s1.notes.iter().any(|n| n.commitment == owned.commitment));
}
