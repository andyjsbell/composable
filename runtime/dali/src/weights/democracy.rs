
//! Autogenerated weights for `democracy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-01-17, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 128

// Executed Command:
// ./target/release/composable
// benchmark
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=democracy
// --extrinsic=*
// --steps=50
// --repeat=20
// --raw
// --output=runtime/dali/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `democracy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> democracy::WeightInfo for WeightInfo<T> {
	// Storage: Democracy PublicPropCount (r:1 w:1)
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	// Storage: Democracy DepositOf (r:0 w:1)
	fn propose() -> Weight {
		(72_511_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Democracy DepositOf (r:1 w:1)
	fn second(s: u32, ) -> Weight {
		(38_333_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((243_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vote_new(r: u32, ) -> Weight {
		(42_720_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((209_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vote_existing(r: u32, ) -> Weight {
		(42_341_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((214_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Cancellations (r:1 w:1)
	fn emergency_cancel() -> Weight {
		(26_547_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Blacklist (r:0 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn blacklist(p: u32, ) -> Weight {
		(88_823_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((545_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	fn external_propose(v: u32, ) -> Weight {
		(14_591_000 as Weight)
			// Standard Error: 0
			.saturating_add((77_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_majority() -> Weight {
		(2_814_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_default() -> Weight {
		(2_786_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn fast_track() -> Weight {
		(27_149_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:1)
	fn veto_external(v: u32, ) -> Weight {
		(28_257_000 as Weight)
			// Standard Error: 0
			.saturating_add((112_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn cancel_proposal(p: u32, ) -> Weight {
		(64_501_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((524_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn cancel_referendum() -> Weight {
		(17_105_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn cancel_queued(r: u32, ) -> Weight {
		(29_326_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((1_645_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:1 w:0)
	fn on_initialize_base(r: u32, ) -> Weight {
		(6_889_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((5_212_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy LastTabledWasExternal (r:1 w:0)
	// Storage: Democracy NextExternal (r:1 w:0)
	// Storage: Democracy PublicProps (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:1 w:0)
	fn on_initialize_base_with_launch_period(r: u32, ) -> Weight {
		(14_236_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((5_230_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy VotingOf (r:3 w:3)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn delegate(r: u32, ) -> Weight {
		(55_901_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((6_870_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
	}
	// Storage: Democracy VotingOf (r:2 w:2)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	fn undelegate(r: u32, ) -> Weight {
		(25_847_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((6_922_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
	}
	// Storage: Democracy PublicProps (r:0 w:1)
	fn clear_public_proposals() -> Weight {
		(3_043_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy Preimages (r:1 w:1)
	fn note_preimage(b: u32, ) -> Weight {
		(41_587_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy Preimages (r:1 w:1)
	fn note_imminent_preimage(b: u32, ) -> Weight {
		(27_444_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy Preimages (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	fn reap_preimage(b: u32, ) -> Weight {
		(39_917_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unlock_remove(r: u32, ) -> Weight {
		(36_489_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((119_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unlock_set(r: u32, ) -> Weight {
		(34_517_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((201_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	fn remove_vote(r: u32, ) -> Weight {
		(19_404_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((205_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	fn remove_other_vote(r: u32, ) -> Weight {
		(19_842_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((196_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
