//! # DEX Router Pallet
//!
//! Is used to add route to DEX for given asset_id's pair.

#![cfg_attr(not(test), warn(clippy::disallowed_method, clippy::indexing_slicing))] // allow in tests
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_type)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use codec::{Codec, FullCodec};
	use composable_traits::{
		defi::LiftedFixedBalance,
		dex::{CurveAmm, DexRoute, DexRouteNode, DexRouter},
	};
	use core::fmt::Debug;
	use frame_support::pallet_prelude::*;
	use sp_runtime::{
		traits::{
			AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, IntegerSquareRoot, One, Zero,
		},
		DispatchResult, FixedPointOperand,
	};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type AssetId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo
			+ Ord;
		type Balance: Default
			+ Parameter
			+ Codec
			+ Copy
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ AtLeast32BitUnsigned
			+ From<u64> // at least 64 bit
			+ Zero
			+ One
			+ IntegerSquareRoot
			+ FixedPointOperand
			+ Into<LiftedFixedBalance>
			+ Into<u128>; // cannot do From<u128>, until LiftedFixedBalance integer part is larger than 128
			  // bit
		/// The maximum hops in the route.
		type MaxHopsInRoute: Get<u32> + TypeInfo;
		type PoolId: FullCodec
			+ Default
			+ TypeInfo
			+ Eq
			+ PartialEq
			+ Ord
			+ Copy
			+ Debug
			+ CheckedAdd
			+ Zero
			+ One;
		type PoolTokenIndex: Copy + Debug + Eq + Into<u32>;
		type StableSwapDex: CurveAmm<
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			PoolTokenIndex = Self::PoolTokenIndex,
			PoolId = Self::PoolId,
		>;
		type ConstantProductDex: CurveAmm<
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			PoolTokenIndex = Self::PoolTokenIndex,
			PoolId = Self::PoolId,
		>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type DexRoutes<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		Blake2_128Concat,
		T::AssetId,
		DexRoute<T::PoolId, T::MaxHopsInRoute>,
		OptionQuery,
	>;

	#[pallet::error]
	pub enum Error<T> {
		/// Number of hops in route exceeded maximum limit.
		MaxHopsExceeded,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RouteAdded {
			x_asset_id: T::AssetId,
			y_asset_id: T::AssetId,
			route: Vec<DexRouteNode<T::PoolId>>,
		},
		RouteDeleted {
			x_asset_id: T::AssetId,
			y_asset_id: T::AssetId,
			route: Vec<DexRouteNode<T::PoolId>>,
		},
		RouteUpdated {
			x_asset_id: T::AssetId,
			y_asset_id: T::AssetId,
			route: Vec<DexRouteNode<T::PoolId>>,
		},
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	impl<T: Config> Pallet<T> {}

	impl<T: Config> DexRouter<T::AssetId, T::PoolId> for Pallet<T> {
		fn update_route(
			asset_pair: (T::AssetId, T::AssetId),
			route: Option<Vec<DexRouteNode<T::PoolId>>>,
		) -> DispatchResult {
			let k1 = asset_pair.0;
			let k2 = asset_pair.1;
			match route {
				Some(route_vec) => {
					let bounded_route =
						route_vec.clone().try_into().map_err(|_| Error::<T>::MaxHopsExceeded)?;
					if DexRoutes::<T>::contains_key(k1, k2) {
						DexRoutes::<T>::insert(k1, k2, DexRoute::Direct(bounded_route));
						Self::deposit_event(Event::RouteUpdated {
							x_asset_id: k1,
							y_asset_id: k2,
							route: route_vec,
						});
					} else {
						DexRoutes::<T>::insert(k1, k2, DexRoute::Direct(bounded_route));
						Self::deposit_event(Event::RouteAdded {
							x_asset_id: k1,
							y_asset_id: k2,
							route: route_vec,
						});
					}
				},
				None => {
					if let Some(deleted_route) = DexRoutes::<T>::take(k1, k2) {
						let deleted_route = match deleted_route {
							DexRoute::Direct(bounded_vec) => bounded_vec.into_inner(),
						};
						Self::deposit_event(Event::RouteDeleted {
							x_asset_id: k1,
							y_asset_id: k2,
							route: deleted_route,
						});
					}
				},
			}
			Ok(())
		}
		fn get_route(asset_pair: (T::AssetId, T::AssetId)) -> Option<Vec<DexRouteNode<T::PoolId>>> {
			let route = DexRoutes::<T>::get(asset_pair.0, asset_pair.1);
			if let Some(route) = route {
				match route {
					DexRoute::Direct(bounded_vec) => return Some(bounded_vec.into_inner()),
				}
			}
			None
		}
	}
}
