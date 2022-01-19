
//! Autogenerated weights for `timestamp`
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
// --pallet=timestamp
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

/// Weight functions for `timestamp`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> timestamp::WeightInfo for WeightInfo<T> {
	// Storage: Timestamp Now (r:1 w:1)
	// Storage: Aura CurrentSlot (r:1 w:0)
	fn set() -> Weight {
<<<<<<< HEAD
		(10_297_000 as Weight)
=======
		(10_081_000 as Weight)
>>>>>>> a26f8d09c2eebfa5abe6e802a45f40018fc17beb
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn on_finalize() -> Weight {
<<<<<<< HEAD
		(4_714_000 as Weight)
=======
		(4_540_000 as Weight)
>>>>>>> a26f8d09c2eebfa5abe6e802a45f40018fc17beb
	}
}
