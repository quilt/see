use interface::{Account, Address, Transaction};
use proof::{number::Number, List};

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
    let mut list = List::new();

    for account in accounts {
        let addr = account
            .pubkey
            .clone()
            .to_address(2u128.pow(N::val() as u32));

        list.insert(addr.into(), account);
    }

    list
}

#[cfg(test)]
mod test {
    use super::*;
    use arrayref::array_ref;
    use interface::crypto::PublicKey;
    use oof::hash::hash;
    use proof::number::U2;

    fn make_value(val: &[u8]) -> [u8; 32] {
        let mut buf = [0; 32];
        buf[0..val.len()].copy_from_slice(val);
        buf
    }

    #[test]
    fn test() {
        let mut one_pk = vec![0; 49];
        let mut two_pk = vec![0; 49];
        one_pk.extend(&((std::u128::MAX >> 1) - 2).to_le_bytes());
        two_pk.extend(&((std::u128::MAX >> 1) + 1).to_le_bytes());

        let one = Account {
            balance: 100,
            nonce: 1,
            pubkey: PublicKey::new(*array_ref![one_pk, 0, 65]),
        };

        let two = Account {
            balance: 42,
            nonce: 2,
            pubkey: PublicKey::new(*array_ref![two_pk, 0, 65]),
        };

        let state = state_with_accounts::<U2>(vec![one.clone(), two.clone()]);

        let forty = hash(
            array_ref![one.pubkey.as_bytes(), 0, 32],
            array_ref![one.pubkey.as_bytes(), 32, 32],
        );
        let forty_one = hash(&make_value(&one.pubkey.as_bytes()[64..65]), &[0; 32]);
        let twenty = hash(&forty, &forty_one);
        let ten = hash(&twenty, &make_value(&one.balance.to_le_bytes()));
        let eleven = hash(&make_value(&one.nonce.to_le_bytes()), &[0; 32]);
        let five = hash(&ten, &eleven);

        let forty_eight = hash(
            array_ref![two.pubkey.as_bytes(), 0, 32],
            array_ref![two.pubkey.as_bytes(), 32, 32],
        );
        let forty_nine = hash(&make_value(&two.pubkey.as_bytes()[64..65]), &[0; 32]);
        let twenty_four = hash(&forty_eight, &forty_nine);
        let twelve = hash(&twenty_four, &make_value(&two.balance.to_le_bytes()));
        let thirteen = hash(&make_value(&two.nonce.to_le_bytes()), &[0; 32]);
        let six = hash(&twelve, &thirteen);

        let two = hash(&[0; 32], &five);
        let three = hash(&six, &[0; 32]);
        let one = hash(&two, &three);

        assert_eq!(state.to_proof().root(), Ok(&one));
    }
}
