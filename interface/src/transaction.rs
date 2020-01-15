use crate::{
    error::{Error, ERR, OK},
    Address,
};
use arrayref::array_ref;
use core::mem::{size_of, transmute, transmute_copy};

const TX_LEN: usize = size_of::<Transaction>();
const SIGNATURE_LEN: usize = 96;

#[repr(packed)]
pub struct Transaction {
    pub to: Address,
    pub from: Address,
    pub nonce: u64,
    pub amount: u64,
    pub signature: [u8; SIGNATURE_LEN],
}

impl Transaction {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        debug_assert_eq!(bytes.len(), TX_LEN);
        unsafe { transmute_copy(array_ref![bytes, 0, TX_LEN]) }
    }

    pub fn to_bytes(self) -> [u8; TX_LEN] {
        unsafe { transmute(self) }
    }

    pub const fn len(&self) -> usize {
        size_of::<Transaction>()
    }

    pub fn verify(&self) -> Error {
        OK
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize() {
        let tx = Transaction {
            to: Address::zero(),
            from: Address::one(),
            nonce: 3,
            amount: 4,
            signature: [5; SIGNATURE_LEN],
        };

        let mut bytes = Address::zero().to_bytes().to_vec();
        bytes.extend(&Address::one().to_bytes());
        bytes.extend(&3u64.to_le_bytes());
        bytes.extend(&4u64.to_le_bytes());
        bytes.extend([5u8; SIGNATURE_LEN].to_vec());

        assert_eq!(tx.to_bytes().to_vec(), bytes);
    }
}
