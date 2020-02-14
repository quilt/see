#![no_std]
#![cfg_attr(target_arch = "wasm32", no_main)]

use interface::{
    error::{Error, OK},
    RawBlob, RefAccount, Transaction,
};

use proof::{
    number::{Number, U2},
    reflist::RefList,
};

#[cfg(target_arch = "wasm32")]
use oof::Oof;

#[cfg(target_arch = "wasm32")]
mod native {
    extern "C" {
        pub fn eth2_loadPreStateRoot(offset: *const u32);
        pub fn eth2_blockDataSize() -> u32;
        pub fn eth2_blockDataCopy(outputOfset: *const u32, offset: u32, length: u32);
        pub fn eth2_savePostStateRoot(offset: *const u32);
    }
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn main() {
    let input_size = unsafe { native::eth2_blockDataSize() as usize };
    let mut input = [0u8; 64000];
    unsafe {
        native::eth2_blockDataCopy(input.as_mut_ptr() as *const u32, 0, input_size as u32);
    }
    let mut pre_state_root = [0u8; 32];
    unsafe { native::eth2_loadPreStateRoot(pre_state_root.as_mut_ptr() as *const u32) }
    let post_root = entry(&mut input, &pre_state_root);
    unsafe { native::eth2_savePostStateRoot(post_root.as_ptr() as *const u32) }
}

pub fn entry(blob: &mut [u8], pre: &[u8; 32]) -> [u8; 32] {
    let mut blob = RawBlob::new(blob);
    let mut db = RefList::<RefAccount, U2>::from_raw(blob.raw_proof());
    db.verify(&pre);

    let txs = blob.transactions();

    for tx in txs {
        db.begin();
        match process_tx(&mut db, &tx) {
            OK => (),
            _ => db.rollback(),
        }
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
