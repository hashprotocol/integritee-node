
//! Autogenerated weights for `pallet_multisig`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-14, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("integritee-solo-fresh"), DB CACHE: 128

// Executed Command:
// target/release/integritee-node
// benchmark
// --chain=integritee-solo-fresh
// --steps=50
// --repeat=20
// --pallet=pallet_multisig
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=runtime/src/weights/pallet_multisig.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_multisig.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		(5_896_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(z as Weight))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		(47_432_000 as Weight)
			// Standard Error: 7_000
			.saturating_add((285_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(z as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: Multisig Calls (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	fn as_multi_create_store(s: u32, z: u32, ) -> Weight {
		(51_683_000 as Weight)
			// Standard Error: 8_000
			.saturating_add((408_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(z as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		(28_039_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((236_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(z as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: Multisig Calls (r:1 w:1)
	fn as_multi_approve_store(s: u32, z: u32, ) -> Weight {
		(50_363_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((356_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(z as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: Multisig Calls (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		(65_416_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((459_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((4_000 as Weight).saturating_mul(z as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	fn approve_as_multi_create(s: u32, ) -> Weight {
		(45_215_000 as Weight)
			// Standard Error: 15_000
			.saturating_add((484_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: Multisig Calls (r:1 w:0)
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		(27_906_000 as Weight)
			// Standard Error: 28_000
			.saturating_add((138_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: Multisig Calls (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn approve_as_multi_complete(s: u32, ) -> Weight {
		(107_320_000 as Weight)
			// Standard Error: 13_000
			.saturating_add((483_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: Multisig Calls (r:1 w:1)
	fn cancel_as_multi(s: u32, ) -> Weight {
		(79_773_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((383_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
