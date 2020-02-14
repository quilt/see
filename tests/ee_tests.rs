extern crate ease;
use arrayref::array_ref;
use ease::engine::Engine;
use interface::{Account, Address, PublicKey, Transaction, SIGNATURE_LEN};
use oof::Oof;
use proof::number::U2;
use proof::reflist::RefNode;
use wrapper::generate::{build_state, ee_code, transfer};

#[test]
fn test() {
    // TODO

    let mut engine = Engine::new();
    // build initial state
    let mut one_pk = vec![0; 49];
    let mut two_pk = vec![0; 49];
    one_pk.extend(&((std::u128::MAX >> 1) - 2).to_le_bytes());
    two_pk.extend(&((std::u128::MAX >> 1) + 1).to_le_bytes());

    let mut zero = Account {
        balance: 100,
        nonce: 1,
        pubkey: PublicKey::new(*array_ref![one_pk, 0, 65]),
    };

    let mut one = Account {
        balance: 42,
        nonce: 2,
        pubkey: PublicKey::new(*array_ref![two_pk, 0, 65]),
    };
    let initial_state = build_state::<U2>(vec![zero.clone(), one.clone()]);
    let mut initial_state_proof = build_state::<U2>(vec![zero.clone(), one.clone()]).to_proof();
    let initial_state_root = initial_state_proof.root().unwrap();
    //let initial_state = vec![0u8;32];
    // get generated ee wasm code
    let code = ee_code();
    // deploy code with state
    let index = engine.deploy(
        code,
        initial_state.to_proof().to_bytes(),
        *initial_state_root,
    );
    // create a tx
    let tx = Transaction {
        to: Address::zero(),
        from: Address::one(),
        nonce: 3,
        amount: 1,
        signature: [5; SIGNATURE_LEN],
    }
    .to_bytes()
    .to_vec();
    let txs = vec![tx; 1];
    // execute tx on node
    engine.run(index, txs);
    // get resulting state root
    let post_state_root = engine.get_root(index);
    // calculate expected root
    zero.balance += 1;
    one.balance -= 1;
    one.nonce += 1;
    let expected_state = build_state::<U2>(vec![zero.clone(), one.clone()]);
    let mut expected_stare_proof = expected_state.to_proof();
    let expected_state_root = *expected_stare_proof.root().unwrap();

    println!(
        "initial_state_root  => {:?}",
        hex::encode(initial_state_root)
    );
    println!("post_state_root => {:?}", hex::encode(post_state_root));
    println!(
        "expected_state_root => {:?}",
        hex::encode(expected_state_root)
    );
    assert_eq!(post_state_root, expected_state_root);
}
