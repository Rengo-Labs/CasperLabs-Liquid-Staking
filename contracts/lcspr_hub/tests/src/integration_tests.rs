#[cfg(test)]
mod test_fixture;

#[cfg(test)]
mod tests {
    // use casper_engine_test_support::TestContext;
    use casper_types::{Key, U256, U512};

    use crate::test_fixture::{Sender, TestFixture};

    #[test]
    fn should_install() {
        let fixture = TestFixture::install_contract();
        assert_eq!(fixture.token_name(), TestFixture::TOKEN_NAME);
        assert_eq!(fixture.token_symbol(), TestFixture::TOKEN_SYMBOL);
        assert_eq!(fixture.token_decimals(), TestFixture::TOKEN_DECIMALS);
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(fixture.token_total_supply())
        );
    }

    #[test]
    fn should_deposit() {
        let mut fixture = TestFixture::install_contract();

        let cspr_deposit_amount = U512::from(100);
        let obtained_cswap = U256::from(100);
        let sender = Sender(fixture.ali);

        let initial_balance = fixture.balance_of(Key::from(fixture.ali)).unwrap();
        let expected_balance = initial_balance + obtained_cswap;

        fixture.deposit(sender, cspr_deposit_amount);

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(expected_balance)
        );
        assert_eq!(fixture.cspr_balance(), cspr_deposit_amount);
    }

    #[test]
    fn should_withdraw() {
        let mut fixture = TestFixture::install_contract();

        let cspr_deposit_amount = U512::from(42);
        let obtained_cswap = U256::from(42);
        let sender = Sender(fixture.ali);

        let initial_balance = fixture.balance_of(Key::from(fixture.ali)).unwrap();
        let expected_deposit_balance = initial_balance + obtained_cswap;

        fixture.deposit(sender, cspr_deposit_amount);
        assert_eq!(fixture.cspr_balance(), cspr_deposit_amount);
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(expected_deposit_balance)
        );

        let cspr_withdraw_amount1 = U512::from(12);
        let cspr_withdraw_amount1_u256 = U256::from(12);
        fixture.withdraw(sender, cspr_withdraw_amount1);

        assert_eq!(
            fixture.cspr_balance(),
            cspr_deposit_amount - cspr_withdraw_amount1
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(expected_deposit_balance - cspr_withdraw_amount1_u256)
        );
    }

    #[test]
    fn should_transfer() {
        let mut fixture = TestFixture::install_contract();

        // Deposit CSWAP to ali balance
        let cspr_deposit_amount = U512::from(1000);
        let obtained_cswap = U256::from(1000);
        let sender = Sender(fixture.ali);

        let initial_balance = fixture.balance_of(Key::from(fixture.ali)).unwrap();
        let expected_balance = initial_balance + obtained_cswap;

        fixture.deposit(sender, cspr_deposit_amount);

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(expected_balance)
        );
        assert_eq!(fixture.cspr_balance(), cspr_deposit_amount);

        // Make transfer from ali to bob
        assert_eq!(fixture.balance_of(Key::from(fixture.bob)), None);
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(fixture.token_total_supply())
        );
        let transfer_amount_1 = U256::from(42);
        fixture.transfer(
            Key::from(fixture.bob),
            transfer_amount_1,
            Sender(fixture.ali),
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob)),
            Some(transfer_amount_1)
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(fixture.token_total_supply() - transfer_amount_1)
        );

        // Make transfer from bob to ali
        let transfer_amount_2 = U256::from(20);
        fixture.transfer(
            Key::from(fixture.ali),
            transfer_amount_2,
            Sender(fixture.bob),
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(fixture.token_total_supply() - transfer_amount_1 + transfer_amount_2),
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob)),
            Some(transfer_amount_1 - transfer_amount_2)
        );
    }

    #[test]
    fn should_transfer_from() {
        let approve_amount = U256::from(100);
        let transfer_amount = U256::from(42);
        assert!(approve_amount > transfer_amount);

        let mut fixture = TestFixture::install_contract();

        let owner = fixture.ali;
        let spender = fixture.bob;
        let recipient = fixture.joe;

        // Deposit CSWAP to ali balance
        let cspr_deposit_amount = U512::from(1000);
        let obtained_cswap = U256::from(1000);
        let sender = Sender(fixture.ali);

        let initial_balance = fixture.balance_of(Key::from(fixture.ali)).unwrap();
        let expected_balance = initial_balance + obtained_cswap;

        fixture.deposit(sender, cspr_deposit_amount);

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(expected_balance)
        );
        assert_eq!(fixture.cspr_balance(), cspr_deposit_amount);

        // Approve ali CSWAP to be spent by bob
        let owner_balance_before = fixture
            .balance_of(Key::from(owner))
            .expect("owner should have balance");
        fixture.approve(Key::from(spender), approve_amount, Sender(owner));
        assert_eq!(
            fixture.allowance(Key::from(owner), Key::from(spender)),
            Some(approve_amount)
        );

        // Bob transfer CSWAP of ali to joe
        fixture.transfer_from(
            Key::from(owner),
            Key::from(recipient),
            transfer_amount,
            Sender(spender),
        );

        assert_eq!(
            fixture.balance_of(Key::from(owner)),
            Some(owner_balance_before - transfer_amount),
            "should decrease balance of the owner"
        );
        assert_eq!(
            fixture.allowance(Key::from(owner), Key::from(spender)),
            Some(approve_amount - transfer_amount),
            "should decrease allowance of the spender"
        );
        assert_eq!(
            fixture.balance_of(Key::from(recipient)),
            Some(transfer_amount),
            "recipient should receive tokens"
        );
    }

    #[test]
    fn should_transfer_full_amount() {
        let mut fixture = TestFixture::install_contract();

        // Deposit CSWAP to ali balance
        let cspr_deposit_amount = U512::from(1000);
        let obtained_cswap = U256::from(1000);
        let sender = Sender(fixture.ali);

        let initial_balance = fixture.balance_of(Key::from(fixture.ali)).unwrap();
        let expected_balance = initial_balance + obtained_cswap;

        fixture.deposit(sender, cspr_deposit_amount);

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(expected_balance)
        );
        assert_eq!(fixture.cspr_balance(), cspr_deposit_amount);

        // Transfer all CSWAP from ali to bob
        let initial_ali_balance = fixture.balance_of(Key::from(fixture.ali)).unwrap();
        assert_eq!(fixture.balance_of(Key::from(fixture.bob)), None);

        fixture.transfer(
            Key::from(fixture.bob),
            initial_ali_balance,
            Sender(fixture.ali),
        );

        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob)),
            Some(initial_ali_balance)
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(U256::zero())
        );

        // Transfer all CSWAP from bob to ali
        fixture.transfer(
            Key::from(fixture.ali),
            initial_ali_balance,
            Sender(fixture.bob),
        );

        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob)),
            Some(U256::zero())
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(initial_ali_balance)
        );
    }

    #[should_panic(expected = "ApiError::User(65534) [131070]")]
    #[test]
    fn should_not_transfer_with_insufficient_balance() {
        let mut fixture = TestFixture::install_contract();

        let initial_ali_balance = fixture.balance_of(Key::from(fixture.ali)).unwrap();
        assert_eq!(fixture.balance_of(Key::from(fixture.bob)), None);

        fixture.transfer(
            Key::from(fixture.bob),
            initial_ali_balance + U256::one(),
            Sender(fixture.ali),
        );
    }

    #[should_panic(expected = "ApiError::User(65533) [131069]")]
    #[test]
    fn should_not_transfer_from_more_than_approved() {
        let approve_amount = U256::from(100);
        let transfer_amount = U256::from(42);
        assert!(approve_amount > transfer_amount);

        let mut fixture = TestFixture::install_contract();

        let owner = fixture.ali;
        let spender = fixture.bob;
        let recipient = fixture.joe;

        fixture.approve(Key::from(spender), approve_amount, Sender(owner));
        assert_eq!(
            fixture.allowance(Key::from(owner), Key::from(spender)),
            Some(approve_amount)
        );

        fixture.transfer_from(
            Key::from(owner),
            Key::from(recipient),
            approve_amount + U256::one(),
            Sender(spender),
        );
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}