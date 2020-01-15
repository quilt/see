#![no_std]

mod account;
mod address;
mod blob;
mod transaction;

pub use account::Account;
pub use address::Address;
pub use blob::RawBlob;
pub use transaction::Transaction;

pub mod error {
    pub type Error = usize;
    pub const OK: usize = 0;
    pub const ERR: usize = 1;
}
