#![no_std]
#![no_main]

use interface::{
    error::{Error, ERR, OK},
    number::{Number, U4},
    Address, RawBlob, Transaction,
};

use core::marker::PhantomData;

use bonsai::{first_leaf, subtree_index_to_general};
use oof::Oof;

type Root = [u8; 32];

#[no_mangle]
pub extern "C" fn main() {}

pub fn entry(blob: &mut [u8], pre: Root) {
    let mut blob = RawBlob::new(blob);
    let mut db = List::<Account, U4>::from_raw(blob.raw_proof());
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

fn process_tx<N: Number>(db: &mut List<Account, N>, tx: &Transaction) -> Error {
    tx.verify();
    let mut to = db.get_mut(&tx.to);
    let mut from = db.get_mut(&tx.from);

    let to_balance = to.balance();
    let from_balance = from.balance();

    to.set_balance(to_balance + tx.amount);
    from.set_balance(from_balance - tx.amount);
    from.inc_nonce();

    OK
}

struct List<T: Provable, N: Number> {
    backend: Oof,
    _t: PhantomData<T>,
    _n: PhantomData<N>,
}

impl<T: Provable, N: Number> List<T, N> {
    pub fn from_raw(bytes: &mut [u8]) -> Self {
        Self {
            backend: unsafe { Oof::from_raw(bytes.as_mut_ptr()) },
            _t: PhantomData,
            _n: PhantomData,
        }
    }

    pub fn verify(&mut self, a: &Root) -> Error {
        let b = self.backend.root();

        if Ok(a) == b {
            OK
        } else {
            ERR
        }
    }

    pub fn get_mut(&mut self, address: &Address) -> T {
        T::new(
            first_leaf(1, N::val() as u128) + *address,
            (&mut self.backend) as *mut Oof,
        )
    }

    pub fn begin(&mut self) {
        todo!()
    }

    pub fn commit(&mut self) {
        todo!()
    }

    pub fn rollback(&mut self) {
        todo!()
    }
}

struct Account {
    idx: u128,
    backend: *mut Oof,
}

impl Account {
    pub fn balance(&self) -> u64 {
        let bytes = Oof::get(unsafe { &*self.backend }, &self.balance_key()).unwrap();
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&bytes[0..8]);
        u64::from_le_bytes(buf)
    }

    pub fn set_balance(&mut self, balance: u64) {
        let mut buf = [0u8; 32];
        buf.copy_from_slice(&balance.to_le_bytes());
        Oof::set(unsafe { &mut *self.backend }, self.balance_key(), buf);
    }

    pub fn nonce(&self) -> u64 {
        let bytes = Oof::get(unsafe { &*self.backend }, &self.nonce_key()).unwrap();
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&bytes[0..8]);
        u64::from_le_bytes(buf)
    }

    pub fn inc_nonce(&mut self) {
        let nonce = self.nonce() + 1;
        let mut buf = [0u8; 32];
        buf.copy_from_slice(&nonce.to_le_bytes());
        Oof::set(unsafe { &mut *self.backend }, self.nonce_key(), buf);
    }

    pub fn pubkey(&self) -> [u8; 48] {
        let bytes_0 = Oof::get(
            unsafe { &*self.backend },
            &subtree_index_to_general(self.idx, 8),
        )
        .unwrap();
        let bytes_1 = Oof::get(
            unsafe { &*self.backend },
            &subtree_index_to_general(self.idx, 9),
        )
        .unwrap();

        let mut buf = [0u8; 48];
        buf.copy_from_slice(&bytes_0[0..32]);
        buf.copy_from_slice(&bytes_1[0..16]);
        buf
    }

    const fn balance_key(&self) -> u128 {
        subtree_index_to_general(self.idx, 5)
    }

    const fn nonce_key(&self) -> u128 {
        subtree_index_to_general(self.idx, 6)
    }
}

trait Provable {
    fn new(idx: u128, backend: *mut Oof) -> Self;
}

impl Provable for Account {
    fn new(idx: u128, backend: *mut Oof) -> Self {
        Self { idx, backend }
    }
}
