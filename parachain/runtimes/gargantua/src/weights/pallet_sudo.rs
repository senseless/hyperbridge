
//! Autogenerated weights for `pallet_sudo`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-06-04, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `polytope-labs`, CPU: `AMD Ryzen Threadripper PRO 5995WX 64-Cores`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("gargantua-2000")`, DB CACHE: 1024

// Executed Command:
// target/release/hyperbridge
// benchmark
// pallet
// --chain=gargantua-2000
// --wasm-execution=compiled
// --pallet
// pallet_sudo
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// parachain/runtimes/gargantua/src/weights/pallet_sudo.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_sudo`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_sudo::WeightInfo for WeightInfo<T> {
	/// Storage: `Sudo::Key` (r:1 w:1)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn set_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `132`
		//  Estimated: `1517`
		// Minimum execution time: 10_630_000 picoseconds.
		Weight::from_parts(11_031_000, 0)
			.saturating_add(Weight::from_parts(0, 1517))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Sudo::Key` (r:1 w:0)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn sudo() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `132`
		//  Estimated: `1517`
		// Minimum execution time: 11_893_000 picoseconds.
		Weight::from_parts(12_293_000, 0)
			.saturating_add(Weight::from_parts(0, 1517))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Sudo::Key` (r:1 w:0)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn sudo_as() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `132`
		//  Estimated: `1517`
		// Minimum execution time: 12_354_000 picoseconds.
		Weight::from_parts(12_583_000, 0)
			.saturating_add(Weight::from_parts(0, 1517))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Sudo::Key` (r:1 w:1)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn remove_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `132`
		//  Estimated: `1517`
		// Minimum execution time: 9_849_000 picoseconds.
		Weight::from_parts(10_009_000, 0)
			.saturating_add(Weight::from_parts(0, 1517))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}