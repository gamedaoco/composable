//! Autogenerated weights for frame_system
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-08-22, STEPS: `[5, ]`, REPEAT: 2, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 128

// Executed Command:
// ./target/release/composable
// benchmark
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=frame_system
// --extrinsic=*
// --steps=5
// --repeat=2
// --raw
// --output=./runtime/picasso/src/weights

#![allow(unused_parens)]
#![allow(unused_imports)]

use sp_std::marker::PhantomData;
use support::{traits::Get, weights::Weight};

/// Weight functions for frame_system.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_system::WeightInfo for WeightInfo<T> {
	fn remark(_b: u32) -> Weight {
		(1_381_000 as Weight)
	}
	fn remark_with_event(b: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(b as Weight))
	}
	fn set_heap_pages() -> Weight {
		(3_000_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_changes_trie_config() -> Weight {
		(15_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn set_storage(i: u32) -> Weight {
		(25_491_000 as Weight)
			// Standard Error: 28_000
			.saturating_add((962_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	fn kill_storage(i: u32) -> Weight {
		(23_657_000 as Weight)
			// Standard Error: 55_000
			.saturating_add((676_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	fn kill_prefix(p: u32) -> Weight {
		(13_414_000 as Weight)
			// Standard Error: 77_000
			.saturating_add((1_634_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
}
