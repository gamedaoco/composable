//! Setup of XCMP for parachain.
use super::{AssetId, *}; // recursive dependency onto runtime

use codec::{Decode, Encode};
use cumulus_primitives_core::ParaId;
use support::traits::{Everything, Nothing};
use orml_xcm_support::{IsNativeConcrete, MultiCurrencyAdapter, MultiNativeAsset};
use sp_runtime::traits::Convert;
use sp_std::prelude::*;
use xcm::latest::prelude::*;
use xcm::latest::Error;
use xcm_executor::traits::WeightTrader;
use xcm_executor::{Assets, Config, XcmExecutor};
use polkadot_parachain::primitives::Sibling;
use pallet_xcm::XcmPassthrough;
use xcm_builder::{
	AccountId32Aliases, AllowTopLevelPaidExecutionFrom, EnsureXcmOrigin, FixedWeightBounds, LocationInverter,
	ParentIsDefault, RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia,
	SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit,
};

/// here we should allow only from hydradx/acala
/// may be without credit
pub type Barrier = (TakeWeightCredit, AllowTopLevelPaidExecutionFrom<Everything>);

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
/// https://medium.com/kusama-network/kusamas-governance-thwarts-would-be-attacker-9023180f6fb
pub type LocalOriginToLocation = ();

//pub type LocalOriginToLocation = SignedToAccountId32<Origin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, ()>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
);

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

struct Todo;

// so if we have to use ORML stuff to simplify XCMP, need to impl on our trait, or need to implement(copy) custom stuff (akala/hydra uses ORML)
impl orml_traits::MultiCurrency<AccountId> for Todo {
    type CurrencyId = CurrencyId;

    type Balance = <Runtime as balances::Config>::Balance;

    fn minimum_balance(currency_id: Self::CurrencyId) -> Self::Balance {
        todo!()
    }

    fn total_issuance(currency_id: Self::CurrencyId) -> Self::Balance {
        todo!()
    }

    fn total_balance(currency_id: Self::CurrencyId, who: &AccountId) -> Self::Balance {
        todo!()
    }

    fn free_balance(currency_id: Self::CurrencyId, who: &AccountId) -> Self::Balance {
        todo!()
    }

    fn ensure_can_withdraw(currency_id: Self::CurrencyId, who: &AccountId, amount: Self::Balance) -> sp_runtime::DispatchResult {
        todo!()
    }

    fn transfer(
		currency_id: Self::CurrencyId,
		from: &AccountId,
		to: &AccountId,
		amount: Self::Balance,
	) -> sp_runtime::DispatchResult {
        todo!()
    }

    fn deposit(currency_id: Self::CurrencyId, who: &AccountId, amount: Self::Balance) -> sp_runtime::DispatchResult {
        todo!()
    }

    fn withdraw(currency_id: Self::CurrencyId, who: &AccountId, amount: Self::Balance) -> sp_runtime::DispatchResult {
        todo!()
    }

    fn can_slash(currency_id: Self::CurrencyId, who: &AccountId, value: Self::Balance) -> bool {
        todo!()
    }

    fn slash(currency_id: Self::CurrencyId, who: &AccountId, amount: Self::Balance) -> Self::Balance {
        todo!()
    }
}

pub type LocalAssetTransactor = MultiCurrencyAdapter<
	Todo, //	Currencies,
	UnknownTokens,
	IsNativeConcrete<AssetId, CurrencyIdConvert>,
	AccountId,
	LocationToAccountId,
	AssetId,
	CurrencyIdConvert,
>;

pub struct XcmConfig;

impl xcm_executor::Config for XcmConfig {
	type Call = Call;
	type XcmSender = XcmRouter;
	// How to withdraw and deposit an asset.
	//type AssetTransactor = ();
	type AssetTransactor = ();//LocalAssetTransactor;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = NativeAsset;
	type IsTeleporter = (); // <- should be enough to allow teleportation of PICA
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
	type Trader = ();
	type ResponseHandler = (); // Don't handle responses for now.
	type SubscriptionService = PolkadotXcm;
	type AssetClaims = PolkadotXcm;
	type AssetTrap = PolkadotXcm;
}

parameter_types! {
	pub SelfLocation: MultiLocation = MultiLocation::new(1, X1(Parachain(ParachainInfo::get().into())));
}


parameter_types! {
	/// The amount of weight an XCM operation takes. This is a safe overestimate.
	pub const BaseXcmWeight: Weight = 100_000_000;
	pub const MaxInstructions: u32 = 100;
}

impl orml_xtokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type CurrencyId = AssetId;
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

impl sp_runtime::traits::Convert<AssetId, Option<MultiLocation>> for CurrencyIdConvert {
	fn convert(id: AssetId) -> Option<MultiLocation> {
		todo!("here call our asset registry")
		// match id {
		// 	0 => Some(MultiLocation::new(
		// 		1,
		// 		X2(Parachain(ParachainInfo::get().into()), GeneralKey(id.encode())),
		// 	)),
		// 	_ => {
		// 		if let Some(loc) = AssetRegistry::asset_to_location(id) {
		// 			Some(loc.0)
		// 		} else {
		// 			None
		// 		}
		// 	}
		// }
	}
}