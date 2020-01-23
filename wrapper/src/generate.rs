use interface::{Account, Address, Transaction};
use proof::{
    number::{Number, U4},
    List,
};

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

pub fn state_with_accounts<N: Number>(accounts: Vec<Account>) -> List<Account, N> {
    let list = List::new();

    for account in accounts {
        let addr = account.pubkey.to_address(N::val());
        list.insert(addr, account);
    }

    list
}
