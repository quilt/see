use crate::error::{Error, ERR, OK};
use crate::number::Number;
use crate::Index;
use bonsai::first_leaf;
use core::marker::PhantomData;
use oof::Oof;

pub struct RefList<T: RefNode, N: Number> {
    backend: Oof,
    _t: PhantomData<T>,
    _n: PhantomData<N>,
}

impl<T: RefNode, N: Number> RefList<T, N> {
    pub fn from_raw(bytes: &mut [u8]) -> Self {
        Self {
            backend: unsafe { Oof::from_raw(bytes.as_mut_ptr()) },
            _t: PhantomData,
            _n: PhantomData,
        }
    }

    pub fn verify(&mut self, a: &[u8; 32]) -> Error {
        let b = self.backend.root();

        if Ok(a) == b {
            OK
        } else {
            ERR
        }
    }

    pub fn get_mut(&mut self, idx: Index) -> T {
        T::new(
            first_leaf(1, N::val() as u128) + idx,
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

pub trait RefNode {
    fn new(idx: u128, backend: *mut Oof) -> Self;
}
