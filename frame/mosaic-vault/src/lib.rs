#![cfg_attr(not(feature = "std"), no_std)]
//! This pallet is used to work with remote tokens.
//! Allows to manage remote tokens, with relevant network, timing and limits configurations.
//! Allows to send amount to relayer deposit, and allows relayer to withdraw.

pub mod mocks;
pub mod traits;

pub use pallet::*;

#[cfg(test)]
mod tests;
#[frame_support::pallet]
pub mod pallet {

	use frame_support::{
		ensure,
		transactional,
		pallet_prelude::*,
		traits::{
			EnsureOrigin,
			UnixTime,
			fungibles::{Mutate, Transfer}
		},
		PalletId,
	};
	use frame_support::storage::{with_transaction, TransactionOutcome};
	use frame_support::traits::tokens::fungibles::Inspect;
	use sp_arithmetic::per_things::Perquintill;
	use sp_core::hashing::keccak_256;
	use frame_system::pallet_prelude::*;
 	use scale_info::TypeInfo;
	use sp_std::{fmt::Debug, vec::Vec};
	use codec::{Codec, FullCodec};
	use sp_runtime::{
         traits::{
			AtLeast32BitUnsigned, Convert, AccountIdConversion,
			Saturating, CheckedSub, CheckedAdd, CheckedMul, CheckedDiv, Zero,

		 },
	};
	use composable_traits::{loans::Timestamp, vault::{Deposit, FundsAvailability, StrategicVault, Vault, VaultConfig }};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
    pub trait Config: frame_system::Config {

        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
		     + Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			 + Inspect<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>;

		type Convert: Convert<Self::Balance, u128> + Convert<u128, Self::Balance>;

		type Balance: Parameter
		    + Member
			+ AtLeast32BitUnsigned
			+ Codec
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ MaxEncodedLen
			+ TypeInfo
			+ CheckedSub
			+ CheckedAdd
			+ Zero
			+ PartialOrd;

		type Nonce:  Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy + MaybeSerializeDeserialize + Debug + MaxEncodedLen + TypeInfo + CheckedSub + CheckedAdd;//+ From<u8>;

		type TransferDelay:  Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy + MaybeSerializeDeserialize + Debug + MaxEncodedLen + TypeInfo;

		type VaultId: Clone
		    + Codec
			+ Debug
			+ PartialEq
			+ Default
			+ Parameter;

		type Vault: StrategicVault<
			VaultId = Self::VaultId,
			AssetId = <Self as Config>::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,>;

		type AssetId: FullCodec
		     + Eq
			 + PartialEq
			 + Copy
			 + MaybeSerializeDeserialize
			 + Debug
			 + Default
			 + TypeInfo;

		type RemoteAssetId: FullCodec
			 + Eq
			 + PartialEq
			 + Copy
			 + MaybeSerializeDeserialize
			 + Debug
			 + Default
			 + TypeInfo;

		type RemoteNetworkId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo;

		type DepositId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo;

		type RelayerOrigin: EnsureOrigin<Self::Origin>;

		type AdminOrigin: EnsureOrigin<Self::Origin>;

		#[pallet::constant]
		type FeeFactor: Get<Self::Balance>;

		#[pallet::constant]
		type ThresholdFactor: Get<Self::Balance>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;

		type FeeAddress: Get<Self::AccountId>;

		type BlockTimestamp: UnixTime;

		type MaxFeeDefault: Get<Self::Balance>;

		type MinFeeDefault: Get<Self::Balance>;
	}
	#[derive(Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
	pub struct DepositInfo<AssetId, Balance > {
        pub asset_id: AssetId,
		pub amount: Balance,
	}

	#[pallet::storage]
	#[pallet::getter(fn remote_asset_id)]
    pub(super) type RemoteAssetId<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::RemoteNetworkId, Blake2_128Concat, T::AssetId, T::RemoteAssetId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn max_asset_transfer_size)]
	pub(super) type MaxAssetTransferSize<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, T::Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn min_asset_transfer_size)]
	pub(super) type MinAssetTransferSize<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, T::Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn max_transfer_delay)]
	pub(super) type MaxTransferDelay<T: Config> = StorageValue<_, T::TransferDelay, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn min_transfer_delay)]
	pub(super) type MinTransferDelay<T: Config> =  StorageValue<_, T::TransferDelay, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn last_transfer)]
	pub(super) type LastTransfer<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Timestamp, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn transfer_lockup_time)]
	pub(super) type TransferLockupTime<T: Config> = StorageValue<_, Timestamp, ValueQuery>;

	#[pallet::type_value]
	pub(super) fn MaxFeeDefault<T: Config>() -> T::Balance {
        T::MaxFeeDefault::get()
	}

	#[pallet::storage]
	#[pallet::getter(fn max_fee)]
	pub(super) type MaxFee<T: Config> = StorageValue<_, T::Balance, ValueQuery, MaxFeeDefault<T>>;

	#[pallet::type_value]
	pub(super) fn MinFeeDefault<T: Config>() -> T::Balance {
        T::MinFeeDefault::get()
	}

	#[pallet::storage]
	#[pallet::getter(fn min_fee)]
	pub(super) type MinFee<T: Config> = StorageValue<_, T::Balance, ValueQuery, MinFeeDefault<T>>;

	#[pallet::storage]
	#[pallet::getter(fn has_been_withdrawn)]
	pub(super) type HasBeenWithdrawn<T: Config> = StorageMap<_, Blake2_128Concat, T::DepositId, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn has_been_unlocked)]
	pub(super) type HasBeenUnlocked<T: Config> = StorageMap<_, Blake2_128Concat, T::DepositId, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn has_been_completed)]
	pub(super) type HasBeenCompleted<T: Config> = StorageMap<_, Blake2_128Concat, T::DepositId, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn in_transfer_funds)]
	pub(super) type InTransferFunds<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, T::Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn total_value_transferred)]
	pub(super) type TotalValueTransferred<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, T::Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn deposits)]
	pub(super) type Deposits<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, DepositInfo<T::AssetId, T::Balance>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn nonce)]
	pub(super) type Nonce<T: Config> = StorageValue<_, T::Nonce, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn fee_threshold)]
	pub(super) type FeeThreshold<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn last_withdraw_id)]
	pub(super) type LastWithdrawID<T: Config> = StorageValue<_, T::DepositId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn last_unlocked_id)]
	pub(super) type LastUnlockedID<T: Config> = StorageValue<_, T::DepositId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn pause_status)]
	pub(super) type PauseStatus<T :Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
 	pub enum Event<T: Config> {

		DepositCompleted {
			sender: T::AccountId,
			asset_id: T::AssetId,
		    remote_asset_id: T::RemoteAssetId,
		    remote_network_id: T::RemoteNetworkId,
		    destination_address: T::AccountId,
		    amount: T::Balance,
		    deposit_id: [u8; 32],
		    transfer_delay: T::TransferDelay,
		},

		WithdrawalCompleted{
		   destination_account: T::AccountId,
           amount: T::Balance,
		   withdraw_amount: T::Balance,
		   fee: T::Balance,
		   asset_id: T::AssetId,
		   deposit_id: T::DepositId,
		},

        TokenAdded {
		   asset_id: T::AssetId,
		   remote_asset_id: T::RemoteAssetId,
		   remote_network_id: T::RemoteNetworkId
		},

		TokenRemoved {
			asset_id: T::AssetId,
			remote_asset_id: T::RemoteAssetId,
			remote_network_id: T::RemoteNetworkId
		},

		MaxTransferDelayChanged {
			new_max_transfer_delay: T::TransferDelay,
		},

		MinTransferDelayChanged{
			new_min_transfer_delay: T::TransferDelay,
		},

		AssetMaxTransferSizeChanged{
			asset_id: T::AssetId,
			size: T::Balance,
		},

		AssetMinTransferSizeChanged {
			asset_id: T::AssetId,
			size: T::Balance,
		},

		LockupTimeChanged{
			sender: T::AccountId,
			old_lockup_time: Timestamp,
			lockup_time: Timestamp,
			action: Vec<u8>,
		},

		MinFeeChanged{
			min_fee: T::Balance,
		},

		MaxFeeChanged {
		   max_fee: T::Balance,
		},

		TransferFundsUnlocked {
			asset_id: T::AssetId,
			amount: T::Balance,
			deposit_id: T::DepositId
		},

		FeeTaken{
            sender: T::AccountId,
			destination_account: T::AccountId,
			asset_id: T::AssetId,
			amount: T::Balance,
			fee: T::Balance,
			deposit_id: T::DepositId,
		},

		FeeThresholdChanged{
			new_fee_threshold: T::Balance,
		},

		Pause{
			sender: T::AccountId,
		},

		UnPause{
			sender: T::AccountId,
		},

		FundsUnlocked{
			asset_id: T::AssetId,
			user_account_id: T::AccountId,
			amount: T::Balance,
			deposit_id: T::DepositId,
		},

		LiquidityMoved{
			sender: T::AccountId,
			to: T::AccountId,
			withdrawable_balance: T::Balance,
		},
	}

	#[allow(missing_docs)]
	#[pallet::error]
	pub enum Error<T> {

		DepositFailed,

		MaxAssetTransferSizeBelowMinimum,

		TransferDelayAboveMaximum,

		TransferDelayBelowMinimum,

		AmountAboveMaxAssetTransferSize,

		AmountBelowMinAssetTransferSize,

		MaxTransferDelayBelowMinimum,

		MinTransferDelayAboveMaximum,

	    MinFeeAboveFeeFactor,

		MaxFeeAboveFeeFactor,

		MinFeeAboveMaxFee,

		MaxFeeBelowMinFee,

		AlreadCompleted,

		InsufficientFunds,

		InsufficientAssetBalance,

		ThresholdFeeAboveThresholdFactor,

		AlreadyWithdrawn,

		TransferNotPossible,

		AssetUnlreadyUnlocked,

		TransferFromFailed,

		BurnFromFailed,

		MintToFailed,

		WithdrawFailed,

		ZeroAmount,

		DivisionError,

		ContractPaused,

		ContractNotPaused,

		NoTransferableBalance,

		UnsupportedToken,

		Underflow,

		Overflow,

		UnlockAmountGreaterThanTotalValueTransferred,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000)]
		 pub fn add_supported_token(origin: OriginFor<T>,
			asset_id: T::AssetId,
			remote_asset_id: T::RemoteAssetId,
			remote_network_id: T::RemoteNetworkId,
			max_asset_transfer_size: T::Balance,
			min_asset_transfer_size: T::Balance,) -> DispatchResultWithPostInfo {

		   T::AdminOrigin::ensure_origin(origin)?;

		   ensure!(max_asset_transfer_size > min_asset_transfer_size, Error::<T>::MaxAssetTransferSizeBelowMinimum);

		   <RemoteAssetId<T>>::insert(remote_network_id, asset_id, remote_asset_id);

		   <MaxAssetTransferSize<T>>::insert(asset_id, max_asset_transfer_size);

		   <MinAssetTransferSize<T>>::insert(asset_id, min_asset_transfer_size);

		   Self::deposit_event(Event::TokenAdded{asset_id, remote_asset_id, remote_network_id});

		   Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn remove_supported_token(origin: OriginFor<T>, asset_id: T::AssetId, remote_network_id: T::RemoteNetworkId) -> DispatchResultWithPostInfo {

			T::AdminOrigin::ensure_origin(origin)?;

 		    if let Some(remote_asset_id) = RemoteAssetId::<T>::get(remote_network_id, asset_id) {

				<RemoteAssetId<T>>::remove(remote_network_id, asset_id);

				<MaxAssetTransferSize<T>>::remove(asset_id);

				<MinAssetTransferSize<T>>::remove(asset_id);

				Self::deposit_event(Event::TokenRemoved{asset_id, remote_asset_id,  remote_network_id});
			 }

			 Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn set_asset_max_transfer_size(origin: OriginFor<T>, asset_id: T::AssetId, size: T::Balance) -> DispatchResultWithPostInfo {

			T::AdminOrigin::ensure_origin(origin)?;

			 <MaxAssetTransferSize<T>>::insert(asset_id, size);

			 Self::deposit_event(Event::AssetMaxTransferSizeChanged {asset_id, size});

			 Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn set_asset_min_transfer_size(origin: OriginFor<T>, asset_id: T::AssetId, size: T::Balance) -> DispatchResultWithPostInfo {

			T::AdminOrigin::ensure_origin(origin)?;

			 <MinAssetTransferSize<T>>::insert(asset_id, size);

			 Self::deposit_event(Event::AssetMinTransferSizeChanged {asset_id, size});

			 Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn set_transfer_lockup_time(origin: OriginFor<T>, lockup_time: Timestamp) -> DispatchResultWithPostInfo {

			T::AdminOrigin::ensure_origin(origin.clone())?;

			let sender = ensure_signed(origin)?;

			 let old_lockup_time = <TransferLockupTime<T>>::get();

			 <TransferLockupTime<T>>::put(lockup_time);

			 let action = b"Transfer".to_vec();

			 Self::deposit_event(Event::LockupTimeChanged{sender, old_lockup_time, lockup_time, action});

			 Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn set_max_transfer_delay(origin: OriginFor<T>, new_max_transfer_delay: T::TransferDelay) -> DispatchResultWithPostInfo {

			T::AdminOrigin::ensure_origin(origin)?;

			ensure!(new_max_transfer_delay >= Self::min_transfer_delay(), Error::<T>::MaxTransferDelayBelowMinimum);

			<MaxTransferDelay<T>>::put(new_max_transfer_delay);

			Self::deposit_event(Event::MaxTransferDelayChanged{new_max_transfer_delay});

			Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn set_min_transfer_delay(origin: OriginFor<T>, new_min_transfer_delay: T::TransferDelay) -> DispatchResultWithPostInfo {

			T::AdminOrigin::ensure_origin(origin)?;

			ensure!(new_min_transfer_delay <= Self::max_transfer_delay(), Error::<T>::MinTransferDelayAboveMaximum);

			<MinTransferDelay<T>>::put(new_min_transfer_delay);

			Self::deposit_event(Event::MinTransferDelayChanged{new_min_transfer_delay});

			Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn set_max_fee(origin: OriginFor<T>, max_fee: T::Balance) -> DispatchResultWithPostInfo {

			T::AdminOrigin::ensure_origin(origin)?;

			ensure!(max_fee < T::FeeFactor::get(), Error::<T>::MaxFeeAboveFeeFactor);

			ensure!(max_fee > Self::min_fee(), Error::<T>::MaxFeeBelowMinFee);

            <MaxFee<T>>::put(max_fee);

			Self::deposit_event(Event::MaxFeeChanged{max_fee});

			Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn set_min_fee(origin: OriginFor<T>, min_fee: T::Balance) -> DispatchResultWithPostInfo {

			T::AdminOrigin::ensure_origin(origin)?;

			ensure!(min_fee < Self::max_fee(), Error::<T>::MinFeeAboveMaxFee);

			ensure!(min_fee < T::FeeFactor::get(), Error::<T>::MinFeeAboveFeeFactor);

            <MinFee<T>>::put(min_fee);

			Self::deposit_event(Event::MinFeeChanged{min_fee});

			Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn set_fee_threshold(origin: OriginFor<T>, new_fee_threshold: T::Balance) -> DispatchResultWithPostInfo {

			T::AdminOrigin::ensure_origin(origin)?;

			ensure!(new_fee_threshold < T::ThresholdFactor::get(), Error::<T>::ThresholdFeeAboveThresholdFactor);

			<FeeThreshold<T>>::put(new_fee_threshold);

			Self::deposit_event(Event::FeeThresholdChanged{new_fee_threshold});

			Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 #[transactional]
		 pub fn deposit(
			 origin: OriginFor<T>,
			 amount: T::Balance,
			 asset_id: T::AssetId,
			 destination_address: T::AccountId,
			 remote_network_id: T::RemoteNetworkId,
			 transfer_delay: T::TransferDelay,
			) -> DispatchResultWithPostInfo {

			let sender = ensure_signed(origin)?;

			ensure!(Self::pause_status() == false, Error::<T>::ContractPaused);

			ensure!(amount != T::Balance::zero(), Error::<T>::ZeroAmount);

			let remote_asset_id = Self::only_supported_remote_asset(remote_network_id.clone(), asset_id.clone())?;

			ensure!(Self::last_transfer(&sender).checked_add(Self::transfer_lockup_time()).ok_or(Error::<T>::Overflow)? < T::BlockTimestamp::now().as_secs(), Error::<T>::TransferNotPossible);

			ensure!(transfer_delay >= <MinTransferDelay<T>>::get(), Error::<T>::TransferDelayBelowMinimum);

			ensure!(transfer_delay <= <MaxTransferDelay<T>>::get(), Error::<T>::TransferDelayAboveMaximum);

			ensure!(amount <= Self::max_asset_transfer_size(asset_id), Error::<T>::AmountAboveMaxAssetTransferSize);

			ensure!(amount >= Self::min_asset_transfer_size(asset_id), Error::<T>::AmountBelowMinAssetTransferSize);
			// update in_transfer_funds
			let in_transfer_funds = Self::in_transfer_funds(asset_id);
			let new_in_transfer_funds = in_transfer_funds.checked_add(&amount).ok_or(Error::<T>::Overflow)?;
			<InTransferFunds<T>>::insert(asset_id, new_in_transfer_funds);

			<LastTransfer<T>>::insert(&sender, T::BlockTimestamp::now().as_secs());

			let pallet_account_id = Self::account_id();

			let deposit_id = Self::generate_deposit_id(remote_network_id, &destination_address, pallet_account_id);
            <Deposits<T>>::insert(deposit_id, DepositInfo{asset_id, amount});

			Self::increase_total_value_transferred(asset_id, amount)?;
			// move funds to pallet amount
			T::Currency::burn_from(asset_id, &sender, amount).map_err(|_|Error::<T>::BurnFromFailed)?;

			Self::deposit_event(Event::DepositCompleted{
					sender,
					asset_id,
					remote_asset_id,
					remote_network_id,
					destination_address,
					amount,
					deposit_id,
					transfer_delay
			});

			Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 #[transactional]
		 pub fn withdraw(
			origin: OriginFor<T>,
			destination_account: T::AccountId,
			amount: T::Balance,
			asset_id: T::AssetId,
			remote_network_id: T::RemoteNetworkId,
	        deposit_id: T::DepositId,
			fee: T::Balance,
		 ) -> DispatchResultWithPostInfo {

			 let sender = ensure_signed(origin.clone())?;

			 T::RelayerOrigin::ensure_origin(origin)?;

			 ensure!(Self::pause_status() == false, Error::<T>::ContractPaused);

			Self::only_supported_remote_asset(remote_network_id.clone(), asset_id.clone())?;

			ensure!(Self::has_been_withdrawn(deposit_id) == false, Error::<T>::AlreadyWithdrawn);

			ensure!(Self::get_current_token_liquidity(asset_id)? >= amount, Error::<T>::InsufficientAssetBalance);

			  <HasBeenWithdrawn<T>>::insert(deposit_id, true);

			  <LastWithdrawID<T>>::put(deposit_id);

			  let pallet_account_id = Self::account_id();

			  let withdraw_amount = amount.saturating_sub(fee);

			 T::Currency::mint_into(asset_id, &destination_account, withdraw_amount).map_err(|_|Error::<T>::MintToFailed)?;

			 Self::decrease_total_value_transferred(asset_id, withdraw_amount)?;

			 if fee > T::Balance::zero() {

				T::Currency::mint_into(asset_id, &Self::get_fee_address(), fee).map_err(|_|Error::<T>::MintToFailed)?;

				Self::decrease_total_value_transferred(asset_id, fee)?;

				Self::deposit_event(Event::FeeTaken{
					sender,
					destination_account: destination_account.clone(),
					asset_id,
					amount,
					fee,
					deposit_id,
				});
			 }

			 Self::deposit_event(Event::WithdrawalCompleted{
				destination_account,
				amount,
				withdraw_amount,
				fee,
				asset_id,
				deposit_id
			 });

			 Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn unlock_intransfer_funds(
			origin: OriginFor<T>,
			asset_id: T:: AssetId,
			amount: T::Balance,
			deposit_id: T::DepositId,
		 ) ->DispatchResultWithPostInfo {

			T::RelayerOrigin::ensure_origin(origin)?;

			ensure!(Self::pause_status() == false, Error::<T>::ContractPaused);

			ensure!(Self::has_been_completed(deposit_id) == false, Error::<T>::AlreadCompleted);

			ensure!(Self::in_transfer_funds(asset_id) >= amount, Error::<T>::InsufficientFunds);

			let deposit = Self::deposits(deposit_id);

			ensure!(deposit.asset_id == asset_id && deposit.amount == amount, Error::<T>::InsufficientFunds);

			<HasBeenCompleted<T>>::insert(deposit_id, true);

	       let new_intransfer_funds = Self::in_transfer_funds(asset_id).checked_sub(&amount).ok_or(Error::<T>::Underflow)?;

		   <InTransferFunds<T>>::insert(asset_id, new_intransfer_funds);

		   Self::deposit_event(Event::TransferFundsUnlocked{asset_id, amount, deposit_id});

			Ok(().into())
		 }

		 /// Mints funds to `user_account_id` if there was deposit previously and not yet been unlocked.
		 /// Deposited is cleaned after.
		 #[pallet::weight(10_000)]
		 pub fn unlock_funds(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			user_account_id: T::AccountId,
			amount: T::Balance,
			deposit_id: T::DepositId,
		 ) ->DispatchResultWithPostInfo {

			 T::RelayerOrigin::ensure_origin(origin.clone())?;

			 ensure!(Self::has_been_unlocked(deposit_id) == false, Error::<T>::AssetUnlreadyUnlocked);

			 ensure!(Self::total_value_transferred(asset_id) >= amount, Error::<T>::UnlockAmountGreaterThanTotalValueTransferred);

			 <HasBeenUnlocked<T>>::insert(deposit_id, true);

			 <LastUnlockedID<T>>::put(deposit_id);

			 T::Currency::mint_into(asset_id, &user_account_id, amount).map_err(|_|Error::<T>::MintToFailed)?;

			 Self::decrease_total_value_transferred(asset_id, amount)?;

			Self::deposit_event(Event::FundsUnlocked{asset_id,user_account_id, amount, deposit_id});

			if Self::has_been_completed(deposit_id) == false {
				Self::unlock_intransfer_funds(origin, asset_id, amount, deposit_id)?;
		    }

			Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn save_funds(
			 origin: OriginFor<T>,
			 asset_id: T::AssetId,
			 to: T::AccountId,
		 ) -> DispatchResultWithPostInfo {

			T::AdminOrigin::ensure_origin(origin.clone())?;

			let sender = ensure_signed(origin)?;

			ensure!(Self::pause_status() == true, Error::<T>::ContractNotPaused);

			let withdrawable_balance = Self::total_value_transferred(asset_id);

			ensure!(withdrawable_balance > T::Balance::zero(), Error::<T>::NoTransferableBalance);

			T::Currency::mint_into(asset_id, &to, withdrawable_balance).map_err(|_|Error::<T>::MintToFailed)?;

			Self::decrease_total_value_transferred(asset_id, withdrawable_balance)?;

		    Self::deposit_event(Event::LiquidityMoved {sender, to, withdrawable_balance});

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn pause(origin: OriginFor<T>) -> DispatchResultWithPostInfo {

			 T::AdminOrigin::ensure_origin(origin.clone())?;

			let sender = ensure_signed(origin)?;

			ensure!(Self::pause_status() == false, Error::<T>::ContractPaused);
			 <PauseStatus<T>>::put(true);
			 Self::deposit_event(Event::Pause{sender});

			 Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn un_pause(origin: OriginFor<T>) -> DispatchResultWithPostInfo {

			T::AdminOrigin::ensure_origin(origin.clone())?;

			let sender = ensure_signed(origin)?;

			 <PauseStatus<T>>::put(false);
			 Self::deposit_event(Event::UnPause{sender});

			 Ok(().into())
		}
 	}

	impl<T: Config> Pallet<T> {
		fn account_id() -> T::AccountId {
			T::PalletId::get().into_account()
		}

		fn get_fee_address() -> T::AccountId {
			T::FeeAddress::get()
		}

		fn increment_nonce() -> T::Nonce {

			let mut nonce = Self::nonce();

			nonce += 1u8.into();

			<Nonce<T>>::put(nonce);

			nonce
		}

		fn get_current_token_liquidity(asset_id: T::AssetId) -> Result<T::Balance, DispatchError> {

			let available_funds = Self::total_value_transferred(asset_id);

			let liquidity = available_funds.checked_sub(&Self::in_transfer_funds(asset_id)).ok_or(Error::<T>::Underflow)?;

			 Ok(liquidity)
		}

		fn only_supported_remote_asset(remote_network_id: T::RemoteNetworkId, asset_id:T::AssetId) -> Result<T::RemoteAssetId, DispatchError> {

			let remote_asset_id = <RemoteAssetId<T>>::try_get(remote_network_id, asset_id).map_err(|_|Error::<T>::UnsupportedToken)?;

			Ok(remote_asset_id)
		}

		fn generate_deposit_id(
			remote_network_id: T::RemoteNetworkId,
			destination_address: &T::AccountId,
			pallet_account_id: T::AccountId,
		) -> [u8; 32] {
            
			let mut encoded_data = vec![
				&remote_network_id.encode(),
				&(Encode::encode(&<frame_system::Pallet<T>>::block_number())).encode(),
				&destination_address.encode(),
				&pallet_account_id.encode(),
				&(Encode::encode(&Self::increment_nonce())).encode()
			];

			let deposit_id = keccak_256(&encoded_data);

			deposit_id
		}

		fn increase_total_value_transferred(asset_id: T::AssetId, amount: T::Balance) -> Result<T::Balance, DispatchError>  {

			let total_value = (Self::total_value_transferred(asset_id)).checked_add(&amount).ok_or(Error::<T>::Overflow)?;

			<TotalValueTransferred<T>>::insert(asset_id, total_value);

			Ok(total_value)
		}

		fn decrease_total_value_transferred(asset_id: T::AssetId, amount: T::Balance) -> Result<T::Balance, DispatchError>  {

			let total_value = (Self::total_value_transferred(asset_id)).checked_sub(&amount).ok_or(Error::<T>::Overflow)?;

			<TotalValueTransferred<T>>::insert(asset_id, total_value);

			Ok(total_value)
		}
	}

 }