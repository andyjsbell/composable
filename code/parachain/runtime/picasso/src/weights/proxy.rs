
//! Autogenerated weights for `proxy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-16, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `c93baf6406af`, CPU: `Intel(R) Xeon(R) CPU @ 2.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/y1z2mfgy9msqas77hhxszf78hqg6mx5y-composable/bin/composable
// benchmark
// pallet
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=10
// --output=code/parachain/runtime/picasso/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `proxy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> proxy::WeightInfo for WeightInfo<T> {
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// The range of component `p` is `[1, 3]`.
	fn proxy(p: u32, ) -> Weight {
		(56_751_000 as Weight)
			// Standard Error: 1_424_000
			.saturating_add((144_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		(100_739_000 as Weight)
			// Standard Error: 33_000
			.saturating_add((679_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 898_000
			.saturating_add((132_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn remove_announcement(a: u32, _p: u32, ) -> Weight {
		(78_628_000 as Weight)
			// Standard Error: 39_000
			.saturating_add((850_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn reject_announcement(a: u32, p: u32, ) -> Weight {
		(65_887_000 as Weight)
			// Standard Error: 28_000
			.saturating_add((574_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 750_000
			.saturating_add((1_493_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn announce(a: u32, p: u32, ) -> Weight {
		(85_432_000 as Weight)
			// Standard Error: 24_000
			.saturating_add((569_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 648_000
			.saturating_add((917_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 3]`.
	fn add_proxy(p: u32, ) -> Weight {
		(72_349_000 as Weight)
			// Standard Error: 1_118_000
			.saturating_add((2_417_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 3]`.
	fn remove_proxy(p: u32, ) -> Weight {
		(72_690_000 as Weight)
			// Standard Error: 669_000
			.saturating_add((1_046_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 3]`.
	fn remove_proxies(p: u32, ) -> Weight {
		(63_884_000 as Weight)
			// Standard Error: 292_000
			.saturating_add((11_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 3]`.
	fn anonymous(_p: u32, ) -> Weight {
		(84_617_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[0, 2]`.
	fn kill_anonymous(p: u32, ) -> Weight {
		(65_477_000 as Weight)
			// Standard Error: 986_000
			.saturating_add((1_022_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
