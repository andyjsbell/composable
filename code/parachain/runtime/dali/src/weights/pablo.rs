
//! Autogenerated weights for `pablo`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-16, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `fde3d2d43403`, CPU: `Intel(R) Xeon(R) CPU @ 2.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/y1z2mfgy9msqas77hhxszf78hqg6mx5y-composable/bin/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=10
// --output=code/parachain/runtime/dali/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pablo`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pablo::WeightInfo for WeightInfo<T> {
	// Storage: CurrencyFactory AssetIdRanges (r:1 w:1)
	// Storage: Pablo PoolCount (r:1 w:1)
	// Storage: Pablo Pools (r:0 w:1)
	fn create() -> Weight {
		(72_652_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Pablo Pools (r:1 w:0)
	// Storage: Tokens Accounts (r:5 w:5)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Pablo PriceCumulativeState (r:1 w:1)
	fn add_liquidity() -> Weight {
		(403_307_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: Pablo Pools (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Tokens Accounts (r:5 w:5)
	// Storage: System Account (r:1 w:0)
	// Storage: Pablo PriceCumulativeState (r:1 w:1)
	fn remove_liquidity() -> Weight {
		(232_273_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: Pablo Pools (r:1 w:0)
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: System Account (r:2 w:1)
	// Storage: Pablo PriceCumulativeState (r:1 w:1)
	fn buy() -> Weight {
		(198_738_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: Pablo Pools (r:1 w:0)
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: System Account (r:2 w:1)
	// Storage: Pablo PriceCumulativeState (r:1 w:1)
	fn swap() -> Weight {
		(199_547_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: CurrencyFactory AssetIdRanges (r:1 w:1)
	// Storage: Pablo PoolCount (r:1 w:1)
	// Storage: Pablo Pools (r:0 w:1)
	fn do_create_pool() -> Weight {
		(58_314_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
