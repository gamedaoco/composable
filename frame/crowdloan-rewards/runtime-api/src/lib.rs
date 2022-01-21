#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use codec::{Decode, Encode};
use pallet_crowdloan_rewards::models::RemoteAccount;

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
	pub trait CrowdloanRewardsRuntimeApi<RelayChainAccountId, Balance>
	where
		RelayChainAccountId: Encode + Decode,
		Balance: Encode + Decode,
	{
		fn amount_available_to_claim_for(account: RemoteAccount<RelayChainAccountId>) -> Balance;
	}
}
