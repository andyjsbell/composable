use crate::{
	mock::{
		self as mock,
		accounts::ALICE,
		assets::DOT,
		runtime::{
			ExtBuilder, Origin, Runtime, System as SystemPallet, TestPallet,
			Timestamp as TimestampPallet,
		},
	},
	pallet::{Error, Event, Markets},
	tests::{as_inner, run_to_block, zero_to_one_open_interval},
};
use composable_traits::{
	clearing_house::ClearingHouse,
	time::{DurationSeconds, ONE_HOUR},
};
use frame_support::{assert_noop, assert_ok, traits::UnixTime};
use proptest::{
	num::f64::{NEGATIVE, POSITIVE, ZERO},
	prelude::*,
};
use sp_runtime::FixedI128;

type MarketConfig = <TestPallet as ClearingHouse>::MarketConfig;
type VammConfig = mock::vamm::VammConfig;

// ----------------------------------------------------------------------------------------------------
//                                          Valid Inputs
// ----------------------------------------------------------------------------------------------------

fn valid_vamm_config() -> VammConfig {
	VammConfig {}
}

fn valid_market_config() -> MarketConfig {
	MarketConfig {
		asset: DOT,
		vamm_config: valid_vamm_config(),
		// 10x max leverage to open a position
		margin_ratio_initial: FixedI128::from_float(0.1),
		// liquidate when above 50x leverage
		margin_ratio_maintenance: FixedI128::from_float(0.02),
		// 'One cent' of the quote asset
		minimum_trade_size: FixedI128::from_float(0.01),
		funding_frequency: ONE_HOUR,
		funding_period: ONE_HOUR * 24,
	}
}

// ----------------------------------------------------------------------------------------------------
//                                             Prop Compose
// ----------------------------------------------------------------------------------------------------

prop_compose! {
	fn float_le_zero()(float in ZERO | NEGATIVE) -> f64 {
		float
	}
}

prop_compose! {
	fn float_ge_one()(float in ZERO | POSITIVE) -> f64 {
		1.0 + float
	}
}

prop_compose! {
	fn initial_le_maintenance_margin_ratio()(
		(maintenance, decrement) in zero_to_one_open_interval()
			.prop_flat_map(|num| (Just(num), 0.0..num))
	) -> (FixedI128, FixedI128) {
		(FixedI128::from_float(maintenance - decrement), FixedI128::from_float(maintenance))
	}
}

prop_compose! {
	fn invalid_margin_ratio_req()(
		float in prop_oneof![float_le_zero(), float_ge_one()]
	) -> FixedI128 {
		FixedI128::from_float(float)
	}
}

prop_compose! {
	fn valid_margin_ratio_req()(float in zero_to_one_open_interval()) -> FixedI128 {
		FixedI128::from_float(float)
	}
}

// ----------------------------------------------------------------------------------------------------
//                                             Create Market
// ----------------------------------------------------------------------------------------------------

#[test]
fn create_first_market_succeeds() {
	ExtBuilder::default().build().execute_with(|| {
		run_to_block(10); // TimestampPallet unix time does not work properly at genesis
		let old_count = TestPallet::market_count();
		let block_time_now = <TimestampPallet as UnixTime>::now().as_secs();

		let config = valid_market_config();
		assert_ok!(TestPallet::create_market(Origin::signed(ALICE), config.clone()));

		// Ensure first market id is 0 (we know its type since it's defined in the mock runtime)
		SystemPallet::assert_last_event(
			Event::MarketCreated { market: 0_u64, asset: config.asset }.into(),
		);
		assert!(Markets::<Runtime>::contains_key(0_u64));

		// Ensure market count is increased by 1
		assert_eq!(TestPallet::market_count(), old_count + 1);

		// Ensure new market matches creation parameters
		let market = TestPallet::get_market(0_u64).unwrap();
		assert_eq!(market.asset_id, config.asset);
		assert_eq!(market.margin_ratio_initial, config.margin_ratio_initial);
		assert_eq!(market.margin_ratio_maintenance, config.margin_ratio_maintenance);
		assert_eq!(market.funding_frequency, config.funding_frequency);
		assert_eq!(market.funding_period, config.funding_period);

		// Ensure last funding rate timestamp is the same as this block's time
		assert_eq!(market.funding_rate_ts, block_time_now);
	})
}

#[test]
fn can_create_two_markets_with_same_config() {
	ExtBuilder::default().build().execute_with(|| {
		run_to_block(2);
		let mut count = TestPallet::market_count();
		let block_time_now = <TimestampPallet as UnixTime>::now().as_secs();

		for _ in 0..2 {
			assert_ok!(TestPallet::create_market(Origin::signed(ALICE), valid_market_config()));

			assert_eq!(TestPallet::get_market(count).unwrap().funding_rate_ts, block_time_now);
			count += 1;
		}
	})
}

#[test]
fn fails_to_create_market_for_unsupported_asset_by_oracle() {
	ExtBuilder { oracle_asset_support: Some(false), ..Default::default() }
		.build()
		.execute_with(|| {
			assert_noop!(
				TestPallet::create_market(Origin::signed(ALICE), valid_market_config()),
				Error::<Runtime>::NoPriceFeedForAsset
			);
		})
}

#[test]
fn fails_to_create_market_if_fails_to_create_vamm() {
	ExtBuilder { vamm_id: None, ..Default::default() }.build().execute_with(|| {
		assert_noop!(
			TestPallet::create_market(Origin::signed(ALICE), valid_market_config()),
			mock::vamm::Error::<Runtime>::FailedToCreateVamm
		);
	})
}

proptest! {
	#[test]
	fn fails_to_create_market_if_funding_period_is_not_multiple_of_frequency(rem in 1..ONE_HOUR) {
		ExtBuilder::default().build().execute_with(|| {
			let mut config = valid_market_config();
			config.funding_frequency = ONE_HOUR;
			config.funding_period = ONE_HOUR * 2 + rem;
			assert_noop!(
				TestPallet::create_market(Origin::signed(ALICE), config),
				Error::<Runtime>::FundingPeriodNotMultipleOfFrequency
			);
		})
	}
}

proptest! {
	#[test]
	fn fails_to_create_market_if_either_funding_period_or_frequency_are_zero(
		(funding_period, funding_frequency) in prop_oneof![
			(Just(0), any::<DurationSeconds>()),
			(any::<DurationSeconds>(), Just(0)),
			Just((0, 0))
		]
	) {
		ExtBuilder::default().build().execute_with(|| {
			let mut config = valid_market_config();
			config.funding_frequency = funding_frequency;
			config.funding_period = funding_period;
			assert_noop!(
				TestPallet::create_market(Origin::signed(ALICE), config),
				Error::<Runtime>::ZeroLengthFundingPeriodOrFrequency
			);
		})
	}
}

proptest! {
	#[test]
	fn fails_to_create_market_if_margin_ratios_not_between_zero_and_one(
		(margin_ratio_initial, margin_ratio_maintenance) in prop_oneof![
			(valid_margin_ratio_req(), invalid_margin_ratio_req()),
			(invalid_margin_ratio_req(), valid_margin_ratio_req()),
			(invalid_margin_ratio_req(), invalid_margin_ratio_req())
		]
	) {
		ExtBuilder::default().build().execute_with(|| {
			let mut config = valid_market_config();
			config.margin_ratio_initial = margin_ratio_initial;
			config.margin_ratio_maintenance = margin_ratio_maintenance;
			assert_noop!(
				TestPallet::create_market(Origin::signed(ALICE), config),
				Error::<Runtime>::InvalidMarginRatioRequirement
			);
		})
	}
}

proptest! {
	#[test]
	fn fails_to_create_market_if_initial_margin_ratio_le_maintenance(
		(margin_ratio_initial, margin_ratio_maintenance) in initial_le_maintenance_margin_ratio()
	) {
		ExtBuilder::default().build().execute_with(|| {
			let mut config = valid_market_config();
			config.margin_ratio_initial = margin_ratio_initial;
			config.margin_ratio_maintenance = margin_ratio_maintenance;
			assert_noop!(
				TestPallet::create_market(Origin::signed(ALICE), config),
				Error::<Runtime>::InitialMarginRatioLessThanMaintenance
			);
		})
	}
}

proptest! {
	#[test]
	fn fails_to_create_market_if_minimum_trade_size_is_negative(
		inner in as_inner(-1_000_000_000)..0
	) {
		ExtBuilder::default().build().execute_with(|| {
			let mut config = valid_market_config();
			config.minimum_trade_size = FixedI128::from_inner(inner);
			assert_noop!(
				TestPallet::create_market(Origin::signed(ALICE), config),
				Error::<Runtime>::NegativeMinimumTradeSize
			);
		})
	}
}

#[test]
fn can_create_market_with_zero_minimum_trade_size() {
	ExtBuilder::default().build().execute_with(|| {
		let mut config = valid_market_config();
		config.minimum_trade_size = 0.into();
		assert_ok!(TestPallet::create_market(Origin::signed(ALICE), config));
	})
}
