#![allow(dead_code)]
#![allow(clippy::many_single_char_names)]
use codec::FullCodec;
use frame_support::{
	codec::{Decode, Encode},
	sp_runtime::Perbill,
};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, Permill};
use sp_std::vec::Vec;

use crate::{currency::{BalanceLike, AssetIdLike}, defi::{Take, OrderIdLike, SellTrait, DeFiTrait, CurrencyPair}};

/// Immediate AMM exchange. Either resolves trade immediately or returns error (mostly because of lack of liquidity).
pub trait AmmExchange : DeFiTrait {	
	type Error;

	/// Obtains the current price for a given asset, possibly routing through multiple markets.
	fn price(asset_id: Self::AssetId) -> Option<Self::Balance>;

	/// Exchange `amount` of `from` asset for `to` asset. The maximum price paid for the `to` asset
	/// is `SimpleExchange::price * (1 + slippage)`
	fn exchange(
		from: Self::AssetId,
		from_account: Self::AccountId,
		to: Self::AssetId,
		to_account: Self::AccountId,
		to_amount: Self::Balance,
		slippage: Perbill,
	) -> Result<Self::Balance, DispatchError>;
}


#[derive(Encode, Decode)]
pub enum Price<GroupId, Balance> {
	Preferred(GroupId, Balance),
	Both { preferred_id: GroupId, preferred_price: Balance, any_price: Balance },
	Any(Balance),
}

impl<GroupId, Balance> Price<GroupId, Balance> {
	pub fn new_any(price: Balance) -> Self {
		Self::Any(price)
	}
}



/// nothing bad in selling nothing
impl<AssetId: Default, Balance : Default> Default for Buy<AssetId, Balance> {
    fn default() -> Self {
        Self { pair: Default::default(), limit: Default::default() }
    }
}


/// take `base` currency and give `quote` currency back
#[derive(Encode, Decode, TypeInfo)]
pub struct Buy<AssetId, Balance> {
	pub pair: CurrencyPair<AssetId>,
	/// maximal price of `base` in `quote` 
	pub limit: Balance,
}



/// This order book is not fully DEX as it has no matching engine.
/// How to sell in market price using this orderbook? 
/// Request existing orders summary and send with `ask`/`bid` with proper amount. 
/// Or create new trait which is market aware, market sell api.
/// How to I see success for my operations?
/// Observer events or on chain history or your account state for give currency.
pub trait LimitOrderbook<Configuration>: SellTrait<Configuration> {
	/// if there is AMM,  and [Self::AmmConfiguration] allows for that, than can use DEX to sell some amount if it is good enough
	type AmmDex : MultiAssetAmm;
	///  buy base asset for price given or lower
	fn bid(
		from_to: &Self::AccountId,		
		order: Buy<Self::AssetId, Self::Balance>,		
		base_amount: Self::Balance,
		amm: Configuration,
	) -> Result<Self::OrderId, DispatchError>;

	/// updates same existing order with new price
	/// to avoid overpay, use `take` with `up_to` price
	fn patch(
		order_id: Self::OrderId,
		price: Self::Balance,
	) -> Result<(), DispatchError>;	
}

pub trait MultiAssetAmm : DeFiTrait {
	/// Perform an exchange between two coins.
	/// `i` - index value of the coin to send,
	/// `j` - index value of the coin to receive,
	/// `dx` - amount of `i` being exchanged,
	/// `min_dy` - minimum amount of `j` to receive.
	fn exchange(
		who: &Self::AccountId,
		pool_id: PoolId,
		i: PoolTokenIndex,
		j: PoolTokenIndex,
		dx: Self::Balance,
		min_dy: Self::Balance,
	) -> Result<(), DispatchError>;
}



// /// AMM for pools with multiple assets (more than 2)
// impl MultiAssetAmm for () {
//     fn exchange(
// 		who: &Self::AccountId,
// 		pool_id: PoolId,
// 		i: PoolTokenIndex,
// 		j: PoolTokenIndex,
// 		dx: Self::Balance,
// 		min_dy: Self::Balance,
// 	) -> Result<(), DispatchError> {
//         DispatchError::CannotLookup // not sure if can do better error
//     }
// }

/// Implement AMM curve from [StableSwap - efficient mechanism for Stablecoin liquidity by Micheal Egorov](https://curve.fi/files/stableswap-paper.pdf) 
/// Also blog at [Understanding stableswap curve](https://miguelmota.com/blog/understanding-stableswap-curve/) as explanation.
pub trait CurveAmm : MultiAssetAmm {
	/// Current number of pools (also ID for the next created pool)
	fn pool_count() -> PoolId;

	/// Information about the pool with the specified `id`
	fn pool(id: PoolId) -> Option<PoolInfo<Self::AccountId, Self::AssetId, Self::Balance>>;

	/// Creates a pool, taking a creation fee from the caller
	fn create_pool(
		who: &Self::AccountId,
		assets: Vec<Self::AssetId>,
		amplification_coefficient: Self::Balance,
		fee: Permill,
		admin_fee: Permill,
	) -> Result<PoolId, DispatchError>;

	/// Deposit coins into the pool
	/// `amounts` - list of amounts of coins to deposit,
	/// `min_mint_amount` - minimum amout of LP tokens to mint from the deposit.
	fn add_liquidity(
		who: &Self::AccountId,
		pool_id: PoolId,
		amounts: Vec<Self::Balance>,
		min_mint_amount: Self::Balance,
	) -> Result<(), DispatchError>;

	/// Withdraw coins from the pool.
	/// Withdrawal amount are based on current deposit ratios.
	/// `amount` - quantity of LP tokens to burn in the withdrawal,
	/// `min_amounts` - minimum amounts of underlying coins to receive.
	fn remove_liquidity(
		who: &Self::AccountId,
		pool_id: PoolId,
		amount: Self::Balance,
		min_amounts: Vec<Self::Balance>,
	) -> Result<(), DispatchError>;	

	/// Withdraw admin fees
	fn withdraw_admin_fees(
		who: &Self::AccountId,
		pool_id: PoolId,
		admin_fee_account: &Self::AccountId,
	) -> Result<(), DispatchError>;
}

//issue: pool will never be as large as u32, event not u16, probably u8     
/// Type that represents index type of token in the pool passed from the outside as an extrinsic
/// argument.
pub type PoolTokenIndex = u32;

/// Type that represents pool id
pub type PoolId = u32;

/// Pool type
#[derive(Encode, Decode, TypeInfo, Clone, Default, PartialEq, Eq, Debug)]
pub struct PoolInfo<AccountId, AssetId, Balance> {
	/// Owner of pool
	pub owner: AccountId,
	/// LP multiasset
	pub pool_asset: AssetId,
	/// List of multiasset supported by the pool
	pub assets: Vec<AssetId>,
	/// Initial amplification coefficient
	pub amplification_coefficient: Balance,
	/// Amount of the fee pool charges for the exchange
	pub fee: Permill,
	/// Amount of the admin fee pool charges for the exchange
	pub admin_fee: Permill,
	/// Current balances excluding admin_fee
	pub balances: Vec<Balance>,
	/// Current balances including admin_fee
	pub total_balances: Vec<Balance>,
}
