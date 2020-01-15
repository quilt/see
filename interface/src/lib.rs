#![no_std]

mod address;
mod blob;
pub mod number;
mod transaction;

pub use address::Address;
pub use blob::RawBlob;
pub use transaction::Transaction;

pub mod error {
    pub type Error = usize;
    pub const OK: usize = 0;
    pub const ERR: usize = 1;
}
