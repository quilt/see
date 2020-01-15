use crate::Transaction;
use arrayref::array_ref;
use core::{
    mem::{size_of, take, transmute},
    slice,
};

pub struct RawBlob<'a>(&'a mut [u8]);

impl<'a> RawBlob<'a> {
    pub fn new(blob: &'a mut [u8]) -> Self {
        Self(blob)
    }

    pub fn transactions(&self) -> &[Transaction] {
        let ptr = self.0.as_ptr();
        unsafe { slice::from_raw_parts(transmute(ptr.offset(4)), self.tx_count() as usize) }
    }

    pub fn tx_count(&self) -> u32 {
        u32::from_le_bytes(*array_ref![self.0, 0, 4])
    }

    pub fn raw_proof(&mut self) -> &mut [u8] {
        let offset = self.tx_count() as usize * size_of::<Transaction>() + 4;
        let (txs, proof): (&mut [u8], &mut [u8]) = take(&mut self.0).split_at_mut(offset);
        self.0 = txs;
        proof
    }
}
