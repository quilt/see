use crate::Address;

#[derive(Clone)]
pub struct PublicKey(pub(crate) [u8; 65]);

impl PublicKey {
    pub fn new(k: [u8; 65]) -> Self {
        Self(k)
    }

    pub fn as_bytes(&self) -> &[u8; 65] {
        &self.0
    }

    pub fn to_address(self, size: u128) -> Address {
        let mut buf = [0; 16];
        buf[0..16].copy_from_slice(&self.0[49..65]);
        let addr = u128::from_le_bytes(buf);
        (addr / (core::u128::MAX / size)).into()
    }
}
