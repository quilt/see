extern crate ease;
use arrayref::array_ref;
use ease::engine::Engine;
use interface::{Account, Address, PublicKey};
use proof::number::U2;
use wrapper::generate::{build_state, ee_code, transfer};

#[test]
fn test() {
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

    let mut initial_state_proof = initial_state.clone().to_proof();
    let initial_state_root = initial_state_proof.root().unwrap();

    // deploy code with state
    let mut engine = Engine::new();
    let index = engine.deploy(
        ee_code(),
        initial_state.to_proof().to_bytes(),
        *initial_state_root,
    );

    // generate tx
    let tx = transfer(Address::zero(), Address::one(), 1, 3);

    // execute tx on node
    engine.run(index, tx, 1);

    // get resulting state root
    let post_state_root = engine.get_root(index);

    // calculate expected root
    zero.balance += 1;
    one.balance -= 1;
    one.nonce += 1;

    let expected_state_root = *build_state::<U2>(vec![zero.clone(), one.clone()])
        .to_proof()
        .root()
        .unwrap();

    println!("pre root  => {:?}", hex::encode(initial_state_root));
    println!("post root => {:?}", hex::encode(post_state_root));
    println!("expected  => {:?}", hex::encode(expected_state_root));

    assert_eq!(post_state_root, expected_state_root);
}
