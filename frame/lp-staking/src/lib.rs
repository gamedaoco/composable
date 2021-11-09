#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

pub use pallet::*;
use sp_runtime::DispatchResult;
use sp_std::{prelude::*, vec::Vec};
use support::{
	dispatch::{CallMetadata, GetCallMetadata},
	pallet_prelude::*,
codec::{Codec, FullCodec}ns, PalletInfoAccess},
	transactional,
};
use system::pallet_prelude::*;
use weights::WeightInfo;

mod mock;
mod tests;
mod weights;

#[support::pallet]
pub mod pallet {
	use codec::FullCodec;
use sp_runtime::Perbill;
use support::{PalletId, traits::PalletInfo};

use super::*;

	#[pallet::config]
	pub trait Config: system::Config {
		/// Overarching event type
		type Event: From<Event<Self>> + IsType<<Self as system::Config>::Event>;

		type Balance: Codec + Default + FullCodec;
		type AssetId: Codec + Default + FullCodec;

		/// The origin which may set LP token configurations.
		type SetOrigin: EnsureOrigin<Self::Origin>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;

		// Weight information for the extrinsics in this module.
		type WeightInfo: WeightInfo;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The passed configuration is incorrect. Most likely due to ratios not adding up to 1.
		InvalidConfiguration,
	}

	#[pallet::event]
	#[pallet::generate_deposit(fn deposit_event)]
	pub enum Event<T: Config> {
		StakeAdded,
		StakeRemoved,
		Claimed,
		ConfigurationUpdated,
	}

	
	#[pallet::storage]
	#[pallet::getter(fn stakers)]
	pub type Stakers<T: Config> =
		StorageDoubleMap<_, Twox64Concat, T::AccountId, Twox64Concat, T::AssetId, (
			// number of locked tokens.
			T::Balance,
			// number of shares
			T::Balance
		), OptionQuery>;


	#[pallet::storage]
	#[pallet::getter(fn lp_tokens)]
	pub type LpTokens<T: Config> =
		StorageMap<_, Twox64Concat, T::AssetId, (
			// weight of the LpToken, might be an inefficient way to model this, as removing or adding an asset requires iterating through all assets.
			Perbill,
			// Duration that they must be staked for, differs per asset as for some LP tokens holding them is riskier, thus the moment should be shorter
			T::Moment
		), OptionQuery>;



	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_initialize(block: T::BlockNumber) -> Weight {
			// should be configured to only run 
			Self::update_income()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn stake(
			origin: OriginFor<T>,
			asset: T::AssetId,
			amount: T::Balance
		) -> DispatchResult {
			let account = ensure_signed(origin)?;
			// 1. Check if already staking, if so perform claim and update stake info.
			// 2. Compute pool shares by normalizing through oracle 	
			// 3. hold(asset, amount)?;
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn unstake(
			origin: OriginFor<T>,
			asset: T::AssetId,
			amount: T::Balance
		) -> DispatchResult {
			let account = ensure_signed(origin)?;
			Self::try_claim(acount, asset, amount)?;
			Self::try_unstake(account, asset, amount)?;
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn claim(
			origin: OriginFor<T>,
			asset: T::AssetId,
			amount: T::Balance
		) -> DispatchResult {
			let account = ensure_signed(origin)?;		
			Self::try_claim(acount, asset, amount)
		}

		#[pallet::weight(10_000)]
		pub fn add_lp_token(
			origin: OriginFor<T>,
			asset: T::AssetId,
			weight: Perbill,
		) -> DispatchResult {
			let account = T::AddOrigin::ensure_origin(origin)?;		
			// 1. compute share (see comment on share storage about making this more efficient).
			// 2. set asset in LpTokens.
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn remove_lp_token(
			origin: OriginFor<T>,
			asset: T::AssetId,
		) -> DispatchResult {
			let account = T::AddOrigin::ensure_origin(origin)?;		
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn try_claim(account: T::AccountId, asset: T::AssetId, amount: T::Balance) -> DispatchResultWithPostInfo {
			// 1. get LpTokenInfo (if removed, error)
			// 2. compute rewards
			todo!();
		}

		fn try_unstake(account: T::AccountId, asset: T::AssetId, amount: T::Balance) -> DispatchResultWithPostInfo {

			todo!();
		}

		fn update_income() {

		}
	}
}