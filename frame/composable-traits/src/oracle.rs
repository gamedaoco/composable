use crate::{currency::LocalAssets, defi::CurrencyPair};
use frame_support::{dispatch::DispatchError, pallet_prelude::*};
use sp_runtime::FixedU128;
use sp_std::vec::Vec;

#[derive(Encode, Decode, Default, Debug, PartialEq)]
pub struct Price<PriceValue, BlockNumber> {
	pub price: PriceValue,
	pub block: BlockNumber,
}

/// An object that is able to provide an asset price.
/// Important: the current price-feed is providing prices in USDT only.
pub trait Oracle {
	type AssetId: Copy;
	type Balance: From<u64>;
	type Timestamp;
	type LocalAssets: LocalAssets<Self::AssetId>;
	/// Quote the `amount` of `asset_id` in normalized currency unit cent. Default is USDT Cent.
	/// Which is 0.01 of USDT. `Result::Err` is returned if `asset_id` not supported or price
	/// information not available.
	///
	/// # Normal assets
	///
	/// Assuming we have a price `price` for an unit (not smallest) of `asset_id` in USDT cents.
	/// Let `decimals` be the number of decimals for `asset_id` as given by
	/// `CurrencyFactory::decimals` The price of an amount `amount` of the smallest possible unit of
	/// `asset_id` is: `price * amount / 10^decimals`
	///
	///
	/// E.g. for BTC, the price is expressed for 1 BTC, but the amount is in sats:
	/// 1 BTC = 10^8 sats
	/// So that:
	/// `get_price(BTC, 1_00000000) = price(1BTC) * 1_00000000 / 10^8 = $50_000 = 5_000_000 USDT
	/// cents`
	///
	/// # Diluted assets
	///
	/// Implementation ensure that a LP token price can be resolved as long as the base asset price
	/// is resolvable.
	///```haskell
	/// data Currency = USDT | BTC
	/// data Asset = Base Currency | Vaulted Asset Int
	///
	/// price :: Asset -> Int
	/// price (Base USDT) = 100
	/// price (Base BTC) = 5000000
	/// price (Vaulted base stock_dilution_rate) = price base * stock_dilution_rate
	/// ```
	fn get_price(
		asset_id: Self::AssetId,
		amount: Self::Balance,
	) -> Result<Price<Self::Balance, Self::Timestamp>, DispatchError>;

	/// Check whether the provided `asset_id` is supported (a.k.a. a price can be computed) by the
	/// oracle.
	fn is_supported(asset: Self::AssetId) -> Result<bool, DispatchError> {
		let exponent = Self::LocalAssets::decimals(asset)?;
		let unit: Self::Balance = 10_u64.pow(exponent).into();
		Self::get_price(asset, unit).map(|_| true)
	}

	/// Time Weighted Average Price
	fn get_twap(
		of: Self::AssetId,
		weighting: Vec<Self::Balance>,
	) -> Result<Self::Balance, DispatchError>;

	/// Up to oracle how it decides ratio.
	/// If there is no direct trading pair, can estimate via common pair (to which all currencies
	/// are normalized). General formula
	/// ```rust
	/// let base_in_common = 1000.0;
	/// let quote_in_common = 100.0;
	/// let ratio = base_in_common / quote_in_common; // 10.0
	/// let base_amount = 3.0;
	/// let needed_base_for_quote = base_amount * ratio; // 300.0
	/// ```
	fn get_ratio(pair: CurrencyPair<Self::AssetId>) -> Result<FixedU128, DispatchError>;
}
