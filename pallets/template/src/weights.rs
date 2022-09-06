
//! Autogenerated weights for `pallet_template`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-06, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Hectors-MBP-14.fritz.box`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_template
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallets/all-weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_template.
pub trait WeightInfo {
	fn do_something(_s: u32) -> Weight;
}

/// Weight functions for `pallet_template`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: TemplateModule Something (r:0 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn do_something(_s: u32, ) -> Weight {
		Weight::from_ref_time(10_120_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
