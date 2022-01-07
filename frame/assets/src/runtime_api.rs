#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use codec::{Decode, Encode};

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
	pub trait AssetsApi<AssetId, AccountId, Balance>
	where
		AssetId: Encode + Decode,
		AccountId: Encode + Decode,
		Balance: Encode + Decode,
	{
		fn balance_of(asset_id: AssetId, account_id: AccountId) -> Option<Balance>;
	}
}
