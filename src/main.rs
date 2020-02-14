use arrayref::array_ref;
use interface::{Account, Address, PublicKey};
use proof::number::U2;
use see::entry;
use wrapper::generate::{build_state, transfer};

pub fn main() {
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

    // generate tx
    let tx = transfer(Address::zero(), Address::one(), 1, 3);

    let mut input = [
        1u32.to_le_bytes().to_vec(),
        tx,
        initial_state.to_proof().to_bytes(),
    ]
    .concat();

    let post_state_root = entry(&mut input, array_ref![initial_state_root, 0, 32]);

    // calculate the expected state root
    zero.balance += 1;
    one.balance -= 1;
    one.nonce += 1;

    let expected_state = build_state::<U2>(vec![zero, one]);
    let mut expected_state_proof = expected_state.to_proof();
    let expected_state_root = expected_state_proof.root().unwrap();

    println!("pre root  => {:?}", hex::encode(initial_state_root));
    println!("post root => {:?}", hex::encode(post_state_root));
    println!("expected  => {:?}", hex::encode(expected_state_root));

    assert_eq!(expected_state_root, array_ref![post_state_root, 0, 32]);
}
