
//! Autogenerated weights for `pallet_multisig`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 46.2.0
//! DATE: 2025-07-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `protocol-team-testing`, CPU: `AMD Ryzen 5 3600 6-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// ./target/production/subspace-node
// benchmark
// pallet
// --runtime=./target/production/wbuild/subspace-runtime/subspace_runtime.compact.compressed.wasm
// --extrinsic=*
// --wasm-execution=compiled
// --genesis-builder=none
// --steps=50
// --repeat=20
// --heap-pages=4096
// --pallet=pallet_multisig
// --output=./crates/subspace-runtime/src/weights/pallet_multisig.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_multisig`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_680_000 picoseconds.
		Weight::from_parts(11_955_035, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 0
			.saturating_add(Weight::from_parts(397, 0).saturating_mul(z.into()))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// Storage: `RuntimeConfigs::EnableDynamicCostOfStorage` (r:1 w:0)
	/// Proof: `RuntimeConfigs::EnableDynamicCostOfStorage` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `186 + s * (2 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 35_919_000 picoseconds.
		Weight::from_parts(27_684_091, 0)
			.saturating_add(Weight::from_parts(0, 6811))
			// Standard Error: 416
			.saturating_add(Weight::from_parts(87_339, 0).saturating_mul(s.into()))
			// Standard Error: 4
			.saturating_add(Weight::from_parts(1_676, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `185`
		//  Estimated: `6811`
		// Minimum execution time: 22_279_000 picoseconds.
		Weight::from_parts(14_441_584, 0)
			.saturating_add(Weight::from_parts(0, 6811))
			// Standard Error: 266
			.saturating_add(Weight::from_parts(82_227, 0).saturating_mul(s.into()))
			// Standard Error: 2
			.saturating_add(Weight::from_parts(1_682, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `288 + s * (33 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 39_099_000 picoseconds.
		Weight::from_parts(30_363_882, 0)
			.saturating_add(Weight::from_parts(0, 6811))
			// Standard Error: 518
			.saturating_add(Weight::from_parts(92_325, 0).saturating_mul(s.into()))
			// Standard Error: 5
			.saturating_add(Weight::from_parts(1_676, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// Storage: `RuntimeConfigs::EnableDynamicCostOfStorage` (r:1 w:0)
	/// Proof: `RuntimeConfigs::EnableDynamicCostOfStorage` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `187 + s * (2 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 24_629_000 picoseconds.
		Weight::from_parts(26_384_331, 0)
			.saturating_add(Weight::from_parts(0, 6811))
			// Standard Error: 670
			.saturating_add(Weight::from_parts(90_871, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `185`
		//  Estimated: `6811`
		// Minimum execution time: 12_809_000 picoseconds.
		Weight::from_parts(13_514_532, 0)
			.saturating_add(Weight::from_parts(0, 6811))
			// Standard Error: 380
			.saturating_add(Weight::from_parts(83_163, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `357 + s * (1 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 25_249_000 picoseconds.
		Weight::from_parts(26_800_720, 0)
			.saturating_add(Weight::from_parts(0, 6811))
			// Standard Error: 354
			.saturating_add(Weight::from_parts(78_310, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
