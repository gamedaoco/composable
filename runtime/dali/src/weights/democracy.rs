//! Autogenerated weights for democracy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-08-24, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 128

// Executed Command:
// ./target/release/composable
// benchmark
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=democracy
// --extrinsic=*
// --steps=50
// --repeat=20
// --raw
// --output=./runtime/picasso/src/weights

#![allow(unused_parens)]
#![allow(unused_imports)]

use sp_std::marker::PhantomData;
use support::{traits::Get, weights::Weight};

/// Weight functions for democracy.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> democracy::WeightInfo for WeightInfo<T> {
	fn propose() -> Weight {
		(112_082_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn second(s: u32) -> Weight {
		(56_014_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((349_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn vote_new(r: u32) -> Weight {
		(62_157_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((512_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn vote_existing(r: u32) -> Weight {
		(62_708_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((495_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn emergency_cancel() -> Weight {
		(38_669_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn blacklist(p: u32) -> Weight {
		(113_805_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((998_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn external_propose(v: u32) -> Weight {
		(19_088_000 as Weight)
			// Standard Error: 0
			.saturating_add((113_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn external_propose_majority() -> Weight {
		(4_287_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn external_propose_default() -> Weight {
		(4_227_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn fast_track() -> Weight {
		(38_917_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn veto_external(v: u32) -> Weight {
		(40_067_000 as Weight)
			// Standard Error: 0
			.saturating_add((172_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn cancel_proposal(p: u32) -> Weight {
		(69_372_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((991_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn cancel_referendum() -> Weight {
		(23_073_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn cancel_queued(r: u32) -> Weight {
		(36_184_000 as Weight)
			// Standard Error: 32_000
			.saturating_add((6_334_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn on_initialize_base(r: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 44_000
			.saturating_add((8_834_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
	}

	fn on_initialize_base_with_launch_period(_r: u32) -> Weight {
		(0 as Weight)
	}

	fn delegate(r: u32) -> Weight {
		(67_447_000 as Weight)
			// Standard Error: 64_000
			.saturating_add((11_600_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
	}
	fn undelegate(r: u32) -> Weight {
		(37_144_000 as Weight)
			// Standard Error: 36_000
			.saturating_add((10_896_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
	}
	fn clear_public_proposals() -> Weight {
		(4_006_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn note_preimage(b: u32) -> Weight {
		(61_302_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn note_imminent_preimage(b: u32) -> Weight {
		(39_217_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn reap_preimage(b: u32) -> Weight {
		(53_786_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn unlock_remove(r: u32) -> Weight {
		(54_249_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((313_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn unlock_set(r: u32) -> Weight {
		(48_885_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((486_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn remove_vote(r: u32) -> Weight {
		(25_483_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((508_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn remove_other_vote(r: u32) -> Weight {
		(26_969_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((289_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
