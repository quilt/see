#![no_std]
#![no_main]

use interface::{
    error::{Error, OK},
    RawBlob, RefAccount, Transaction,
};
use proof::{
    number::{Number, U4},
    reflist::RefList,
};

#[no_mangle]
pub extern "C" fn main() {}

pub fn entry(blob: &mut [u8], pre: [u8; 32]) {
    let mut blob = RawBlob::new(blob);
    let mut db = RefList::<RefAccount, U4>::from_raw(blob.raw_proof());
    db.verify(&pre);

    let txs = blob.transactions();

    for tx in txs {
        db.begin();

        match process_tx(&mut db, &tx) {
            OK => db.commit(),
            _ => db.rollback(),
        }
    }
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