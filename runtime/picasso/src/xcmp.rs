#![cfg_attr(not(feature = "std"), no_std)]

//! Setup of XCMP for parachain.
//! See links which help to understand how it works:
//! - How to setup XCMP:
//! - [Polkadot XCM Cross-Chain Asset Transfer Demo](https://medium.com/oak-blockchain/polkadot-xcm-cross-chain-asset-transfer-demo-53aa9a2e97a7)
//! - https://medium.com/oak-blockchain/tutorial-polkadot-cross-chain-message-passing-xcmp-demo-with-ping-pallet-
//! Format of messages:
//! - https://medium.com/polkadot-network/xcm-part-ii-versioning-and-compatibility-b313fc257b83

use super::{*}; // recursive dependency onto runtime

use codec::{Decode, Encode};
use composable_traits::assets::{RemoteAssetRegistry, XcmAssetLocation};
use cumulus_primitives_core::ParaId;
use support::{
	construct_runtime, match_type, parameter_types,
	traits::{Contains, Everything, KeyOwnerProofSystem, Nothing, Randomness, StorageInfo},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
		DispatchClass, IdentityFee, Weight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	},
	PalletId, StorageValue,
};

use orml_xcm_support::{IsNativeConcrete, MultiCurrencyAdapter, MultiNativeAsset};

use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, ConvertInto, Convert, Zero},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult,
};

use orml_traits::parameter_type_with_key;
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};

use sp_std::prelude::*;
use xcm::latest::prelude::*;
use xcm::latest::Error;
use xcm_executor::traits::{ShouldExecute, WeightTrader};
use xcm_executor::{Assets, Config, XcmExecutor};
use polkadot_parachain::primitives::Sibling;
use pallet_xcm::XcmPassthrough;
use xcm_builder::{AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom, AllowTopLevelPaidExecutionFrom, EnsureXcmOrigin, FixedWeightBounds, LocationInverter, ParentIsDefault, RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia, SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit};

/// here we should allow only from hydradx/acala
/// may be without credit
//pub type Barrier = (TakeWeightCredit, AllowTopLevelPaidExecutionFrom<Everything>);


impl ShouldExecute for Todo {
	fn should_execute<Call>(
		_origin: &MultiLocation,
		_message: &mut Xcm<Call>,
		max_weight: Weight,
		weight_credit: &mut Weight,
	) -> Result<(), ()> {
		dbg!("should execute {:?} {:?}", weight_credit, max_weight);
		Ok(())
	}
}

// pub type Barrier = (
// 	TakeWeightCredit,
// 	AllowTopLevelPaidExecutionFrom<Everything>,
// 	AllowUnpaidExecutionFrom<SpecParachain>,
// 	// Expected responses are OK.
// 	AllowKnownQueryResponses<PolkadotXcm>,
// 	// Subscriptions for version tracking are OK.
// 	AllowSubscriptionsFrom<Everything>,
// );


/// No local origins on this chain are allowed to dispatch XCM sends/executions.
/// https://medium.com/kusama-network/kusamas-governance-thwarts-would-be-attacker-9023180f6fb
//pub type LocalOriginToLocation = ();
pub type LocalOriginToLocation = SignedToAccountId32<Origin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, ()>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
);


//pub struct LocationToAccountId;

// impl xcm_executor::traits::Convert<xcm::v1::MultiLocation, sp_runtime::AccountId32> for LocationToAccountId {
//     fn convert(value: xcm::v1::MultiLocation) -> Result<sp_runtime::AccountId32, xcm::v1::MultiLocation> {
// 		todo!("0")
// 	}

//     fn convert_ref(value: impl std::borrow::Borrow<xcm::v1::MultiLocation>) -> Result<sp_runtime::AccountId32, ()> {
// 		todo!("1")
// 	}

//     fn reverse(value: sp_runtime::AccountId32) -> Result<xcm::v1::MultiLocation, sp_runtime::AccountId32> {
// 		todo!("2")
// 	}

//     fn reverse_ref(value: impl std::borrow::Borrow<sp_runtime::AccountId32>) -> Result<xcm::v1::MultiLocation, ()> {
// 		todo!("3")
// 	}
// }
/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
	// The parent (Relay-chain) origin converts to the default `AccountId`.
	ParentIsDefault<AccountId>,
	// Sibling parachain origins convert to AccountId via the `ParaId::into`.
	SiblingParachainConvertsVia<Sibling, AccountId>,
	// Straight up local `AccountId32` origins just alias directly to `AccountId`.
	AccountId32Aliases<RelayNetwork, AccountId>,
);

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	SovereignSignedViaLocation<LocationToAccountId, Origin>,
	// Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
	// recognised.
	RelayChainAsNative<RelayOrigin, Origin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognised.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, Origin>,
	// Superuser converter for the Relay-chain (Parent) location. This will allow it to issue a
	// transaction from the Root origin.
	ParentAsSuperuser<Origin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `Origin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, Origin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<Origin>,
);

pub struct Todo;


pub struct LocalAssetTransactor;
impl<

	> xcm_executor::traits::TransactAsset
	for LocalAssetTransactor
{
	// fn deposit_asset(asset: &MultiAsset, location: &MultiLocation) -> Result {
	// 	Ok(())
	// }

	fn withdraw_asset(asset: &MultiAsset, location: &MultiLocation) -> sp_std::result::Result<Assets, XcmError> {
		// UnknownAsset::withdraw(asset, location).or_else(|_| {
		// 	let who = AccountIdConvert::convert_ref(location)
		// 		.map_err(|_| XcmError::from(Error::AccountIdConversionFailed))?;
		// 	let currency_id = CurrencyIdConvert::convert(asset.clone())
		// 		.ok_or_else(|| XcmError::from(Error::CurrencyIdConversionFailed))?;
		// 	let amount: MultiCurrency::Balance = Match::matches_fungible(asset)
		// 		.ok_or_else(|| XcmError::from(Error::FailedToMatchFungible))?
		// 		.saturated_into();
		// 	MultiCurrency::withdraw(currency_id, &who, amount).map_err(|e| XcmError::FailedToTransactAsset(e.into()))
		// })?;

		dbg!("{:?}", location);
		Ok(asset.clone().into())



	}
}

// pub type LocalAssetTransactor = MultiCurrencyAdapter<
// 	Tokens,
// 	UnknownTokens,
// 	IsNativeConcrete<CurrencyId, CurrencyIdConvert>,
// 	AccountId,
// 	LocationToAccountId,
// 	CurrencyId,
// 	CurrencyIdConvert,
// >;


parameter_types! {
	pub const BaseXcmWeight: Weight = 0;
	pub const MaxInstructions: u32 = 10_000;
}

// parameter_types! {
// 	// One XCM operation is 1_000_000 weight - almost certainly a conservative estimate.
// 	pub const BaseXcmWeight: Weight = 100_000_000;
// 	pub const MaxInstructions: u32 = 100;
// }

pub struct TradePassthrough();

/// any payment to pass
impl WeightTrader for TradePassthrough {
	fn new() -> Self {
		Self()
	}

	fn buy_weight(&mut self, _weight: Weight, payment: Assets) -> Result<Assets, Error> {
		// Just let it through for now
		Ok(payment)
	}
}

pub struct XcmConfig;

impl xcm_executor::Config for XcmConfig {
	type Call = Call;
	type XcmSender = XcmRouter;
	// How to withdraw and deposit an asset.
	//type AssetTransactor = ();
	type AssetTransactor = LocalAssetTransactor;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = NativeAsset;
	type IsTeleporter = (); // <- should be enough to allow teleportation of PICA
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = Todo;//Barrier;
	type Weigher = FixedWeightBounds<BaseXcmWeight, Call, MaxInstructions>;
	//type Trader = ();
	type Trader = TradePassthrough;
	//type ResponseHandler = (); // Don't handle responses for now.
	type ResponseHandler = PolkadotXcm; // Don't handle responses for now.
	type SubscriptionService = PolkadotXcm;
	type AssetClaims = PolkadotXcm;
	type AssetTrap = PolkadotXcm;
}


parameter_types! {
	pub SelfLocation: MultiLocation = MultiLocation::new(1, X1(Parachain(ParachainInfo::parachain_id().into())));
}

impl orml_xtokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type CurrencyIdConvert = CurrencyIdConvert;
	type AccountIdToMultiLocation = AccountIdToMultiLocation;
	type SelfLocation = SelfLocation;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type Weigher = FixedWeightBounds<BaseXcmWeight, Call, MaxInstructions>;
	type BaseXcmWeight = BaseXcmWeight;
	type LocationInverter = LocationInverter<Ancestry>;
}

impl orml_unknown_tokens::Config for Runtime {
	type Event = Event;
}


pub struct AccountIdToMultiLocation;
impl Convert<AccountId, MultiLocation> for AccountIdToMultiLocation {
	fn convert(account: AccountId) -> MultiLocation {
		X1(AccountId32 {
			network: NetworkId::Any,
			id: account.into(),
		})
		.into()
	}
}


pub struct CurrencyIdConvert;

impl sp_runtime::traits::Convert<CurrencyId, Option<MultiLocation>> for CurrencyIdConvert {
	fn convert(id: CurrencyId) -> Option<MultiLocation> {
		dbg!("mapping {:?} on {:?}", id, ParachainInfo::parachain_id());
		<AssetsRegistry as RemoteAssetRegistry>::asset_to_location(id).map(Into::into)
	}
}

/// converts from Relay parent chain to child chain currency
/// expected that currency in location is in format well known for local chain
impl Convert<MultiLocation, Option<CurrencyId>> for CurrencyIdConvert {
	fn convert(location: MultiLocation) -> Option<CurrencyId> {

		dbg!("CurrencyIdConvert.convert {:?} on {:?}", &location, ParachainInfo::parachain_id());
		match location {
			MultiLocation {
				parents,
				interior: X2(Parachain(id), GeneralKey(key)),
			} if parents == 1 && ParaId::from(id) == ParachainInfo::parachain_id() => {
				// Handling native asset for this parachain
				if let Ok(currency_id) = CurrencyId::decode(&mut &key[..]) {
					Some(currency_id)
				} else {
					None
				}
			}
			_ => {

				let x= <AssetsRegistry as RemoteAssetRegistry>::location_to_asset(XcmAssetLocation(location)).map(Into::into);
				//todo!("11111111111111111");
				Some(x.unwrap())
			}
		}
	}
}


/// covert remote to local, usually when receiving transfer
impl Convert<MultiAsset, Option<CurrencyId>> for CurrencyIdConvert {
	fn convert(asset: MultiAsset) -> Option<CurrencyId> {
		dbg!("{:?}", &asset);
		if let MultiAsset {
			id: Concrete(location), ..
		} = asset
		{
			Self::convert(location)
		} else {
			dbg!("FAILED TO FIND REMOTE ASSET");
			None
		}
	}
}

// For test purposes.
match_type! {
	pub type SpecParachain: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: X1(Parachain(2000)) } |
			MultiLocation { parents: 1, interior: X1(Parachain(2001)) }
	};
}

impl pallet_xcm::Config for Runtime {
	type Event = Event;
	type SendXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	/// https://medium.com/kusama-network/kusamas-governance-thwarts-would-be-attacker-9023180f6fb
	type XcmExecuteFilter = Everything;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Everything;
	type XcmReserveTransferFilter = Everything;
	type LocationInverter = LocationInverter<Ancestry>;
	type Weigher = FixedWeightBounds<BaseXcmWeight, Call, MaxInstructions>;
	type Origin = Origin;
	type Call = Call;

	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}

impl cumulus_pallet_xcm::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type VersionWrapper = ();
	type ChannelInfo = ParachainSystem;
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = system::EnsureRoot<AccountId>;
}