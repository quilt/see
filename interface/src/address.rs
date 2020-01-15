use core::mem::{size_of, transmute_copy};
use core::ops::Add;

const LEN: usize = size_of::<u128>();

#[repr(packed)]
#[derive(Clone, Copy)]
pub struct Address(u128);

impl Address {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        debug_assert_eq!(bytes.len(), LEN);
        Self(unsafe { transmute_copy(&bytes) })
    }

    pub fn to_bytes(self) -> [u8; LEN] {
        self.0.to_le_bytes()
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn one() -> Self {
        Self(1)
    }
}

impl Add for Address {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Add<u128> for Address {
    type Output = Self;

    fn add(self, other: u128) -> Self {
        Self(self.0 + other)
    }
}

impl Add<Address> for u128 {
    type Output = Self;

    fn add(self, other: Address) -> Self {
        self + other.0
    }
}
