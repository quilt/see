use crate::Address;

#[derive(Clone)]
pub struct PublicKey(pub(crate) [u8; 65]);

impl PublicKey {
    pub fn to_address(self, size: usize) -> Address {
        let mut buf = [0; 16];
        buf[0..size].copy_from_slice(&self.0[65 - size..65]);
        u128::from_le_bytes(buf).into()
    }
}
