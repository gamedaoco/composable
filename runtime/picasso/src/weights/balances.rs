
//! Autogenerated weights for `balances`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
<<<<<<< HEAD
//! DATE: 2022-01-18, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
=======
//! DATE: 2022-01-19, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
>>>>>>> a26f8d09c2eebfa5abe6e802a45f40018fc17beb
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 128

// Executed Command:
// ./target/release/composable
// benchmark
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=balances
// --extrinsic=*
// --steps=50
// --repeat=20
// --raw
// --output=runtime/picasso/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `balances`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> balances::WeightInfo for WeightInfo<T> {
	// Storage: System Account (r:2 w:2)
	fn transfer() -> Weight {
<<<<<<< HEAD
		(85_649_000 as Weight)
=======
		(91_710_000 as Weight)
>>>>>>> a26f8d09c2eebfa5abe6e802a45f40018fc17beb
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	fn transfer_keep_alive() -> Weight {
<<<<<<< HEAD
		(52_142_000 as Weight)
=======
		(55_210_000 as Weight)
>>>>>>> a26f8d09c2eebfa5abe6e802a45f40018fc17beb
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	fn set_balance_creating() -> Weight {
<<<<<<< HEAD
		(30_267_000 as Weight)
=======
		(31_990_000 as Weight)
>>>>>>> a26f8d09c2eebfa5abe6e802a45f40018fc17beb
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	fn set_balance_killing() -> Weight {
<<<<<<< HEAD
		(35_831_000 as Weight)
=======
		(38_394_000 as Weight)
>>>>>>> a26f8d09c2eebfa5abe6e802a45f40018fc17beb
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: System Account (r:3 w:3)
	fn force_transfer() -> Weight {
<<<<<<< HEAD
		(85_803_000 as Weight)
=======
		(91_815_000 as Weight)
>>>>>>> a26f8d09c2eebfa5abe6e802a45f40018fc17beb
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	fn transfer_all() -> Weight {
<<<<<<< HEAD
		(63_021_000 as Weight)
=======
		(67_669_000 as Weight)
>>>>>>> a26f8d09c2eebfa5abe6e802a45f40018fc17beb
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	fn force_unreserve() -> Weight {
<<<<<<< HEAD
		(27_374_000 as Weight)
=======
		(29_533_000 as Weight)
>>>>>>> a26f8d09c2eebfa5abe6e802a45f40018fc17beb
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
