/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_module, decl_storage, decl_event, StorageMap, dispatch::Result};
use system::ensure_signed;
use rstd::prelude::*;
use runtime_primitives::traits::{As, Convert, StaticLookup};
//use core::convert::TryFrom;

/// The module's configuration trait.
pub trait Trait: system::Trait + balances::Trait + contract::Trait {
	// TODO: Add other types and constants required configure this module.
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	type BalanceToCurrency: Convert<<Self as balances::Trait>::Balance, contract::BalanceOf<Self>>;
}

/// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as ERC20Storage {
		// Just a dummy storage item. 
		// Here we are declaring a StorageValue, `Something` as a Option<u32>
		// `get(something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
		Something get(something): Option<u32>;
		ERCS get(erc_owner): map(Vec<u8>) => T::AccountId;
		WhitelistContract get(whitelist_contracts): map(T::AccountId) => bool;
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event<T>() = default;

		pub fn create_erc20(origin, ticker :Vec<u8>, owner: T::AccountId) -> Result {
			let sender = ensure_signed(origin)?;
			<ERCS<T>>::insert(ticker.clone(), owner.clone());
			Self::deposit_event(RawEvent::ERC20Created(sender, owner, ticker));
			Ok(())
		}

		pub fn do_whitelist_contracts(origin, contract: T::AccountId, active: bool) -> Result {
			<WhitelistContract<T>>::insert(contract, active);
			Ok(())
		}

		pub fn call_to_contract(origin, value: u32, contractAddress: T::AccountId, data: Vec<u8>) -> Result {
			//let sender = ensure_signed(origin)?;
			<contract::Module<T>>::call(
				origin,
				T::Lookup::unlookup(contractAddress),
				<T::BalanceToCurrency as Convert<T::Balance, contract::BalanceOf<T>>>::convert(<T::Balance as As<_>>::sa(0)),
				<T::Gas as As<_>>::sa(10000),
				data
			)?;
			Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		SomethingStored(u32, AccountId),
		ERC20Created(AccountId, AccountId, Vec<u8>),
	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok};
	use runtime_primitives::{
		BuildStorage,
		traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	impl system::Trait for Test {
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type Digest = Digest;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type Log = DigestItem;
	}
	impl Trait for Test {
		type Event = ();
	}
	type TemplateModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
	}

	#[test]
	fn it_works_for_default_value() {
		with_externalities(&mut new_test_ext(), || {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			assert_eq!(TemplateModule::something(), Some(42));
		});
	}
}
