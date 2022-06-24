use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{
    account::AccountHash, bytesrepr::{FromBytes, ToBytes}, system::auction,
    runtime_args, CLTyped, ContractHash, PublicKey, RuntimeArgs, U512,
};

const CONTRACT_PUBLIC_KEY_DELEGATIONS: &str = "contract_public_key_delegation.wasm";
const CONTRACT_KEY_NAME: &str = "public_key_delegation_contract_hash";

pub const ENTRY_POINT_INIT: &str = "initialize_contract";
pub const ENTRY_POINT_DELEGATE: &str = "delegate_to";
pub const ENTRY_POINT_UNDELEGATE: &str = "initialized";

pub const PUBLIC_KEY: &str = "contract_public_key";
pub const ACCOUNT_HASH: &str = "contract_account_hash";
pub const PUBLIC_KEY_HEX: &str = "contract_public_key_hex";
pub const PUBLIC_KEY: &str = "contract_public_key";
pub const CONTRACT_PURSE: &str = "contract_purse";
pub const INIT: &str = "contract_purse";

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
    
    pub fn install_contract() -> TestFixture {
        println!("DEBUG MESSAGE: install contract");
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
        let joe = PublicKey::ed25519_from_bytes([9u8; 32]).unwrap();

        let mut context = TestContextBuilder::new()
            .with_public_key(ali.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(bob.clone(), U512::from(500_000_000_000_000_000u64))
            .build();

        let session_code = Code::from(CONTRACT_PUBLIC_KEY_DELEGATIONS);
        // let session_args = runtime_args! {};
        let session_args: RuntimeArgs = RuntimeArgs::new();

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

    pub fn delegate_to(&mut self, delegator: PublicKey, validator: PublicKey, amount: U512, sender: Sender) {
        self.call(
            sender,
           ENTRY_POINT_DELEGATE,
            runtime_args! {
                auction::ARG_DELEGATOR => delegator,
                auction::ARG_VALIDATOR => validator,
                auction::ARG_AMOUNT => amount,
            },
        );
    }

    pub fn undelegate_from(&mut self, delegator: PublicKey, validator: PublicKey, amount: U512, sender: Sender) {
        self.call(
            sender,
            ENTRY_POINT_UNDELEGATE,
            runtime_args! {
                auction::ARG_DELEGATOR => delegator,
                auction::ARG_VALIDATOR => validator,
                auction::ARG_AMOUNT => amount,
            },
        );
    }

    pub fn initialize_contract(&mut self, sender: Sender) {
        self.call(
            sender,
            ENTRY_POINT_INIT,
            runtime_args! {},
        );
    }

    pub fn public_key(&self) -> PublicKey {
        self.query_contract(PUBLIC_KEY)
            .unwrap()
    }
}