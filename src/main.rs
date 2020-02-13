


use wrapper::generate::{build_state, ee_code, transfer};
use proof::number::U2;
use oof::Oof;

use interface::{
    error::{Error, OK},
    RawBlob, RefAccount, Transaction,
    Account, Address, PublicKey, 
    SIGNATURE_LEN
};
use proof::{
    number::{Number, U4},
    reflist::RefList,
    reflist::RefNode
};
use std::fs;
use arrayref::array_ref;
extern crate byteorder;
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};


/// This is the entry point when compiled as an executable binary.
pub fn main() {


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

    let keys = initial_state_proof.keys();
    println!("keys: {:?}",keys);

    // create a tx
    let tx = Transaction {
        to: Address::zero(),
        from: Address::one(),
        nonce: 3,
        amount: 1,
        signature: [5; SIGNATURE_LEN],
    }.to_bytes().to_vec();

    let mut num_tx: Vec<u8> = Vec::new();
    num_tx.write_u32::<LittleEndian>(1).unwrap();

    let pre_state_root = initial_state_proof.root().unwrap();
    //let post_state_root = hex::decode(args[1]).unwrap();
    let mut input = [num_tx.to_vec(),tx,initial_state.to_proof().to_bytes()].concat();
    // Process input data
    let post_state_root = entry(&mut input, array_ref![pre_state_root, 0, 32]);

    //assert_eq!(post_root, *array_ref![post_state_root, 0, 32]);

    println!("pre_state_root  => {:?}", hex::encode(pre_state_root));
    println!("post_state_root => {:?}", hex::encode(post_state_root));
}

pub fn entry(blob: &mut [u8], pre: &[u8; 32]) -> [u8; 32] {
    let mut blob = RawBlob::new(blob);
    let mut db = RefList::<RefAccount, U2>::from_raw(blob.raw_proof());
    db.verify(&pre);

    let txs = blob.transactions();

    for tx in txs {
        process_tx(&mut db, &tx);
       /* db.begin();

        match process_tx(&mut db, &tx) {
            OK => db.commit(),
            _ => db.rollback(),
        }*/
    }
    *db.root().unwrap()
}

fn process_tx<N: Number>(db: &mut RefList<RefAccount, N>, tx: &Transaction) -> Error {
    tx.verify();
    let mut to = db.get_mut(tx.to.into());
    let mut from = db.get_mut(tx.from.into());

    let to_balance = to.balance();
    let from_balance = from.balance();

    to.set_balance(to_balance + tx.amount);
    from.set_balance(from_balance - tx.amount);
    from.inc_nonce();

    OK
}
