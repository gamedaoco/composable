#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use codec::{Decode, Encode};

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
	pub trait CrowdloanRewardsRuntimeApi<AccountId, Balance>
	where
		AccountId: Encode + Decode,
		Balance: Encode + Decode,
	{
		fn is_claim_available_for(account: AccountId) -> bool;

		fn claim_amount_for(account: AccountId) -> Option<Balance>;
	}
}
