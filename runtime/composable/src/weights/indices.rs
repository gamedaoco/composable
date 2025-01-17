
//! Autogenerated weights for `indices`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-01-18, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("composable-dev"), DB CACHE: 128

// Executed Command:
// ./target/release/composable
// benchmark
// --chain=composable-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=indices
// --extrinsic=*
// --steps=50
// --repeat=20
// --raw
// --output=runtime/composable/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `indices`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> indices::WeightInfo for WeightInfo<T> {
	// Storage: Indices Accounts (r:1 w:1)
	fn claim() -> Weight {
		(37_831_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Indices Accounts (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn transfer() -> Weight {
		(45_343_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Indices Accounts (r:1 w:1)
	fn free() -> Weight {
		(38_017_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Indices Accounts (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn force_transfer() -> Weight {
		(39_059_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Indices Accounts (r:1 w:1)
	fn freeze() -> Weight {
		(43_452_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
