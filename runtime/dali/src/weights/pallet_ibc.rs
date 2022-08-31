
//! Autogenerated weights for `pallet_ibc`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-25, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=runtime/dali/src/weights
// --log
// error

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_ibc`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_ibc::WeightInfo for WeightInfo<T> {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Ibc ClientUpdateTime (r:0 w:1)
	// Storage: Ibc ClientUpdateHeight (r:0 w:1)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:2 w:1)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:3 w:1)
	fn update_tendermint_client() -> Weight {
		(840_567_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: System BlockHash (r:2 w:0)
	// Storage: Ibc HostConsensusStates (r:1 w:0)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:1)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	fn conn_try_open_tendermint() -> Weight {
		(616_397_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: System BlockHash (r:2 w:0)
	// Storage: Ibc HostConsensusStates (r:1 w:0)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:1)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	fn conn_open_ack_tendermint() -> Weight {
		(587_236_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:1)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	fn conn_open_confirm_tendermint() -> Weight {
		(282_745_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Ibc ChannelCounter (r:1 w:1)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Ibc ChannelsConnection (r:1 w:1)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:0)
	// Storage: unknown [0x6962632f6368616e6e656c456e64732f706f7274732f70696e672f6368616e6e] (r:0 w:1)
	// Storage: unknown [0x6962632f6e65787453657175656e636541636b2f706f7274732f70696e672f63] (r:0 w:1)
	// Storage: unknown [0x6962632f6e65787453657175656e6365526563762f706f7274732f70696e672f] (r:0 w:1)
	// Storage: unknown [0x6962632f6e65787453657175656e636553656e642f706f7274732f70696e672f] (r:0 w:1)
	fn channel_open_init() -> Weight {
		(142_761_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: unknown [0x6962632f6368616e6e656c456e64732f706f7274732f70696e672f6368616e6e] (r:1 w:1)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	fn channel_open_try_tendermint() -> Weight {
		(301_013_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: unknown [0x6962632f6368616e6e656c456e64732f706f7274732f70696e672f6368616e6e] (r:1 w:1)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	fn channel_open_ack_tendermint() -> Weight {
		(311_525_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: unknown [0x6962632f6368616e6e656c456e64732f706f7274732f70696e672f6368616e6e] (r:1 w:1)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	fn channel_open_confirm_tendermint() -> Weight {
		(291_402_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: unknown [0x6962632f6368616e6e656c456e64732f706f7274732f70696e672f6368616e6e] (r:1 w:1)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:0)
	fn channel_close_init() -> Weight {
		(134_145_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: unknown [0x6962632f6368616e6e656c456e64732f706f7274732f70696e672f6368616e6e] (r:1 w:1)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	fn channel_close_confirm_tendermint() -> Weight {
		(287_634_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Ibc ClientUpdateTime (r:1 w:0)
	// Storage: Ibc ClientUpdateHeight (r:1 w:0)
	// Storage: Ibc PacketReceiptCounter (r:1 w:1)
	// Storage: unknown [0x6962632f6368616e6e656c456e64732f706f7274732f70696e672f6368616e6e] (r:1 w:0)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	// Storage: unknown [0x6962632f72656365697074732f706f7274732f70696e672f6368616e6e656c73] (r:1 w:1)
	fn recv_packet_tendermint(i: u32, ) -> Weight {
		(365_359_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((95_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Ibc ClientUpdateTime (r:1 w:0)
	// Storage: Ibc ClientUpdateHeight (r:1 w:0)
	// Storage: Ibc PacketCounter (r:1 w:1)
	// Storage: unknown [0x6962632f6368616e6e656c456e64732f706f7274732f70696e672f6368616e6e] (r:1 w:0)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:0)
	// Storage: unknown [0x6962632f636f6d6d69746d656e74732f706f7274732f70696e672f6368616e6e] (r:1 w:1)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	fn ack_packet_tendermint(i: u32, j: u32, ) -> Weight {
		(355_814_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((92_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 1_000
			.saturating_add((105_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Ibc ClientUpdateTime (r:1 w:0)
	// Storage: Ibc ClientUpdateHeight (r:1 w:0)
	// Storage: Ibc PacketCounter (r:1 w:1)
	// Storage: unknown [0x6962632f6368616e6e656c456e64732f706f7274732f70696e672f6368616e6e] (r:1 w:1)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	// Storage: unknown [0x6962632f636f6d6d69746d656e74732f706f7274732f70696e672f6368616e6e] (r:1 w:1)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	fn timeout_packet_tendermint(i: u32, ) -> Weight {
		(379_261_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((96_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Ibc ConnectionCounter (r:1 w:1)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Ibc ConnectionClient (r:1 w:1)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:0 w:1)
	fn conn_open_init() -> Weight {
		(142_670_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Ibc ClientCounter (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Ibc ClientUpdateTime (r:0 w:1)
	// Storage: Ibc ClientUpdateHeight (r:0 w:1)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:0 w:2)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:0 w:1)
	fn create_client() -> Weight {
		(125_111_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}

	fn set_params() -> Weight {
        0
    }

	fn transfer() -> Weight {
        0
    }

	fn on_chan_open_init() -> Weight {
        0
    }

	fn on_chan_open_try() -> Weight {
        0
    }

	fn on_recv_packet() -> Weight {
        0
    }

	fn on_chan_open_ack() -> Weight {
        0
    }

	fn on_chan_open_confirm() -> Weight {
        0
    }

	fn on_chan_close_init() -> Weight {
        0
    }

	fn on_chan_close_confirm() -> Weight {
        0
    }

	fn on_acknowledgement_packet() -> Weight {
        0
    }

	fn on_timeout_packet() -> Weight {
        0
    }
}