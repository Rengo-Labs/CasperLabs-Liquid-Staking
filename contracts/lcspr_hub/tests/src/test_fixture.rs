use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};
use casper_erc20::constants as consts;
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, AsymmetricType, CLTyped, ContractHash, Key, PublicKey, RuntimeArgs, U256, U512,
};

const CONTRACT_CSWAP_HUB: &str = "liquid_staking_hub.wasm";
const CONTRACT_KEY_NAME: &str = "liquid_staking_hub";

const CONTRACT_PRE_DEPOSIT: &str = "pre_deposit.wasm";

fn blake2b256(item_key_string: &[u8]) -> Box<[u8]> {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(item_key_string);
    hasher.finalize_boxed()
}

#[derive(Clone, Copy)]
pub struct Sender(pub AccountHash);

pub struct TestFixture {
    context: TestContext,
    pub ali: AccountHash,
    pub bob: AccountHash,
    pub joe: AccountHash,
}

impl TestFixture {
    pub const TOKEN_NAME: &'static str = "Liquid Casper";
    pub const TOKEN_SYMBOL: &'static str = "lCSPR";
    pub const TOKEN_DECIMALS: u8 = 9;
    
    pub fn install_contract() -> TestFixture {
        println!("DEBUG MESSAGE: install contract");
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
        let joe = PublicKey::ed25519_from_bytes([9u8; 32]).unwrap();

        let mut context = TestContextBuilder::new()
            .with_public_key(ali.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(bob.clone(), U512::from(500_000_000_000_000_000u64))
            .build();

        let session_code = Code::from(CONTRACT_CSWAP_HUB);
        let session_args = runtime_args! {
            consts::NAME_RUNTIME_ARG_NAME => TestFixture::TOKEN_NAME,
            consts::SYMBOL_RUNTIME_ARG_NAME => TestFixture::TOKEN_SYMBOL,
            consts::DECIMALS_RUNTIME_ARG_NAME => TestFixture::TOKEN_DECIMALS,
            consts::TOTAL_SUPPLY_RUNTIME_ARG_NAME => U256::from(0)
        };

        let session = SessionBuilder::new(session_code, session_args)
            .with_address(ali.to_account_hash())
            .with_authorization_keys(&[ali.to_account_hash()])
            .build();

        context.run(session);
        TestFixture {
            context,
            ali: ali.to_account_hash(),
            bob: bob.to_account_hash(),
            joe: joe.to_account_hash(),
        }
    }

    fn contract_hash(&self) -> ContractHash {
        self.context
            .get_account(self.ali)
            .unwrap()
            .named_keys()
            .get(CONTRACT_KEY_NAME)
            .unwrap()
            .normalize()
            .into_hash()
            .unwrap()
            .into()
    }

    fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self
            .context
            .query(self.ali, &[CONTRACT_KEY_NAME.to_string(), name.to_string()])
        {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                Some(value)
            }
        }
    }

    fn call(&mut self, sender: Sender, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;
        let code = Code::Hash(self.contract_hash().value(), method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        self.context.run(session);
    }

    pub fn token_total_supply(&self) -> U256 {
        self.query_contract(consts::TOTAL_SUPPLY_RUNTIME_ARG_NAME)
            .unwrap()
    }

    pub fn cspr_balance(&self) -> U512 {
        self.query_contract("cspr_balance").unwrap()
    }

    pub fn token_name(&self) -> String {
        self.query_contract(consts::NAME_RUNTIME_ARG_NAME).unwrap()
    }

    pub fn token_symbol(&self) -> String {
        self.query_contract(consts::SYMBOL_RUNTIME_ARG_NAME)
            .unwrap()
    }

    pub fn token_decimals(&self) -> u8 {
        self.query_contract(consts::DECIMALS_RUNTIME_ARG_NAME)
            .unwrap()
    }

    pub fn balance_of(&self, account: Key) -> Option<U256> {
        let item_key = base64::encode(&account.to_bytes().unwrap());

        let key = Key::Hash(self.contract_hash().value());
        let value = self
            .context
            .query_dictionary_item(key, Some(consts::BALANCES_KEY_NAME.to_string()), item_key)
            .ok()?;

        Some(value.into_t::<U256>().unwrap())
    }

    pub fn allowance(&self, owner: Key, spender: Key) -> Option<U256> {
        let mut preimage = Vec::new();
        preimage.append(&mut owner.to_bytes().unwrap());
        preimage.append(&mut spender.to_bytes().unwrap());
        let key_bytes = blake2b256(&preimage);
        let allowance_item_key = hex::encode(&key_bytes);

        let key = Key::Hash(self.contract_hash().value());

        let value = self
            .context
            .query_dictionary_item(
                key,
                Some(consts::ALLOWANCES_KEY_NAME.to_string()),
                allowance_item_key,
            )
            .ok()?;

        Some(value.into_t::<U256>().unwrap())
    }

    pub fn transfer(&mut self, recipient: Key, amount: U256, sender: Sender) {
        self.call(
            sender,
            consts::TRANSFER_ENTRY_POINT_NAME,
            runtime_args! {
                consts::RECIPIENT_RUNTIME_ARG_NAME => recipient,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount
            },
        );
    }

    pub fn approve(&mut self, spender: Key, amount: U256, sender: Sender) {
        self.call(
            sender,
            consts::APPROVE_ENTRY_POINT_NAME,
            runtime_args! {
                consts::SPENDER_RUNTIME_ARG_NAME => spender,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount
            },
        );
    }

    pub fn transfer_from(&mut self, owner: Key, recipient: Key, amount: U256, sender: Sender) {
        self.call(
            sender,
            consts::TRANSFER_FROM_ENTRY_POINT_NAME,
            runtime_args! {
                consts::OWNER_RUNTIME_ARG_NAME => owner,
                consts::RECIPIENT_RUNTIME_ARG_NAME => recipient,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount
            },
        );
    }

    pub fn deposit(&mut self, sender: Sender, cspr_amount: U512) {
        let Sender(address) = sender;
        let code = Code::from(CONTRACT_PRE_DEPOSIT);
        let session = SessionBuilder::new(
            code,
            runtime_args! {
                "cspr_amount" => cspr_amount,
                "liquid_staking_hub_hash_key" => Key::from(self.contract_hash())
            },
        )
        .with_address(address)
        .with_authorization_keys(&[address])
        .build();
        self.context.run(session);
    }

    pub fn withdraw(&mut self, sender: Sender, cspr_amount: U512) {
        self.call(
            sender,
            "withdraw",
            runtime_args! {
                "cspr_amount" => cspr_amount,
            },
        )
    }
}