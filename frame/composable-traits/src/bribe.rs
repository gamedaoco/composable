use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

pub trait Bribe {
	type Balance;
	type BribeIndex;
	type Conviction;
	type CurrencyId;
	type ReferendumIndex;
	type AccountId;

	/// Let a user create a bribe request and buy votes
	fn create_bribe(
		request: CreateBribeRequest<
			Self::AccountId,
			Self::ReferendumIndex,
			Self::Balance,
			Self::Conviction,
			Self::CurrencyId,
		>,
	) -> Result<Self::BribeIndex, DispatchError>;

	/// Sell your votes
	fn take_bribe(
		request: TakeBribeRequest<Self::BribeIndex, Self::Balance, Self::Conviction>,
	) -> Result<bool, DispatchError>;

	/// Delete the Bribe Request
	fn delete_bribe(request: DeleteBribeRequest<Self::BribeIndex>) -> Result<bool, DispatchError>;
}

/// A request to create a bribe for an (ongoing) referendum.
#[derive(Copy, Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
pub struct CreateBribeRequest<AccountId, ReferendumIndex, Balance, Conviction, CurrencyId> {
	/// Account id of the creator of the Bribe request
	pub account_id: AccountId,
	/// Index of the referendum.
	pub ref_index: ReferendumIndex,
	/// How much to pay bribe-takers in total.
	pub total_reward: Balance,
	/// What asset to pay the bribes with.
	pub asset_id: CurrencyId,
	/// Requested votes (e.g. number of tokens and the lock period).
	pub requested_votes: Votes<Balance, Conviction>,
	/// Whether the bribe is in favor of the proposal or against it.
	pub is_aye: bool,
}

/// A request to take a bribe and vote for the corresponding referendum.
#[derive(Copy, Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
pub struct TakeBribeRequest<BribeIndex, Balance, Conviction> {
	/// Index of the bribe.
	pub bribe_index: BribeIndex,
	/// A product of token amount and lock period.
	pub votes: Votes<Balance, Conviction>,
}

/// Delete a Bribe request
#[derive(Copy, Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
pub struct DeleteBribeRequest<BribeIndex> {
	/// Index of the bribe.
	pub bribe_index: BribeIndex,
}

#[derive(Copy, Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Votes<Balance, Conviction> {
	pub capital: Balance,
	pub conviction: Conviction,
}
