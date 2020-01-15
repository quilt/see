pub mod generate {
    use interface::{Address, Transaction};

    pub fn transfer(to: Address, from: Address, amount: u64, nonce: u64) -> Vec<u8> {
        Transaction {
            to,
            from,
            amount,
            nonce,
            signature: [0; 96],
        }
        .to_bytes()
        .to_vec()
    }
}
