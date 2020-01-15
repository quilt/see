use bonsai::subtree_index_to_general;
use oof::Oof;
use proof::Provable;

pub struct Account {
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

impl Provable for Account {
    fn new(idx: u128, backend: *mut Oof) -> Self {
        Self { idx, backend }
    }
}
