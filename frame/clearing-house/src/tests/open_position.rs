use crate::{
	math::FromBalance,
	mock::{
		accounts::ALICE,
		assets::USDC,
		runtime::{
			Balance, ExtBuilder, MarketId, Origin, Runtime, System as SystemPallet, TestPallet,
			Vamm as VammPallet,
		},
	},
	pallet::{Config, Direction, Error, Event},
	tests::{
		any_price, as_balance, run_to_block, valid_market_config, MarginInitializer,
		MarketInitializer,
	},
};
use composable_traits::clearing_house::ClearingHouse;
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;
use sp_runtime::{FixedI128, FixedPointNumber, FixedU128};

// ----------------------------------------------------------------------------------------------------
//                                          Valid Inputs
// ----------------------------------------------------------------------------------------------------

fn valid_quote_asset_amount() -> Balance {
	as_balance(100)
}

fn valid_base_asset_amount_limit() -> Balance {
	as_balance(10)
}

// ----------------------------------------------------------------------------------------------------
//                                             Prop Compose
// ----------------------------------------------------------------------------------------------------

fn any_direction() -> impl Strategy<Value = Direction> {
	prop_oneof![Just(Direction::Long), Just(Direction::Short)]
}

prop_compose! {
	fn min_trade_size_and_eps(min_size: u128)(
		eps in -(min_size as i128)..=(min_size as i128)
	) -> (FixedI128, i128) {
		// Couldn't find a better way to ensure that min_size is positive, so this will trigger a
		// test error otherwise
		assert!(min_size > 0);
		(FixedI128::from_inner(min_size as i128), eps)
	}
}

prop_compose! {
	fn percentage_fraction()(percent in 1..100_u128) -> FixedU128 {
		FixedU128::from((percent, 100))
	}
}

// ----------------------------------------------------------------------------------------------------
//                                            Open Position
// ----------------------------------------------------------------------------------------------------

#[test]
fn fails_to_open_position_if_market_id_invalid() {
	let mut market_id: MarketId = 0;
	let quote_amount = valid_quote_asset_amount();
	let base_amount_limit = valid_base_asset_amount_limit();

	ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
		.build()
		.init_market(&mut market_id, None)
		.add_margin(&ALICE, USDC, quote_amount)
		.execute_with(|| {
			// Current price = quote_amount / base_amount_limit
			VammPallet::set_price(Some((quote_amount, base_amount_limit).into()));

			assert_noop!(
				TestPallet::open_position(
					Origin::signed(ALICE),
					market_id + 1,
					Direction::Long,
					quote_amount,
					base_amount_limit
				),
				Error::<Runtime>::MarketIdNotFound,
			);
		})
}

#[test]
fn fails_to_create_new_position_if_violates_maximum_positions_num() {
	let max_positions = <Runtime as Config>::MaxPositions::get() as usize;
	let mut market_ids = Vec::<_>::new();
	let orders = max_positions + 1;
	let configs = vec![None; orders];

	let quote_amount_total = valid_quote_asset_amount();
	let quote_amount: Balance = quote_amount_total / (orders as u128);
	let base_amount_limit: Balance = valid_base_asset_amount_limit() / (orders as u128);

	ExtBuilder { balances: vec![(ALICE, USDC, quote_amount_total)], ..Default::default() }
		.build()
		.init_markets(&mut market_ids, configs.into_iter())
		.add_margin(&ALICE, USDC, quote_amount_total)
		.execute_with(|| {
			// Current price = quote_amount / base_amount_limit
			VammPallet::set_price(Some((quote_amount, base_amount_limit).into()));

			for market_id in market_ids.iter().take(max_positions) {
				assert_ok!(TestPallet::open_position(
					Origin::signed(ALICE),
					*market_id,
					Direction::Long,
					quote_amount,
					base_amount_limit,
				));
			}

			assert_noop!(
				TestPallet::open_position(
					Origin::signed(ALICE),
					market_ids[max_positions],
					Direction::Long,
					quote_amount,
					base_amount_limit,
				),
				Error::<Runtime>::MaxPositionsExceeded
			);
		})
}

proptest! {
	#[test]
	fn open_position_in_new_market_succeeds(
		direction in any_direction()
	) {
		let mut market_id: MarketId = 0;
		let quote_amount = valid_quote_asset_amount();
		let base_amount = valid_base_asset_amount_limit();

		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, None)
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				// For event emission
				run_to_block(1);

				let positions_before = TestPallet::get_positions(&ALICE).len();

				// Current price = quote_amount / base_amount
				VammPallet::set_price(Some((quote_amount, base_amount).into()));
				assert_ok!(TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					direction,
					quote_amount,
					base_amount,
				));

				let positions = TestPallet::get_positions(&ALICE);
				assert_eq!(positions.len(), positions_before + 1);
				let position = positions.iter().find(|p| p.market_id == market_id).unwrap();
				assert!(match direction {
					Direction::Long => position.base_asset_amount.is_positive(),
					Direction::Short => position.base_asset_amount.is_negative()
				});
				assert!(match direction {
					Direction::Long => position.quote_asset_notional_amount.is_positive(),
					Direction::Short => position.quote_asset_notional_amount.is_negative()
				});

				SystemPallet::assert_last_event(
					Event::TradeExecuted {
						market: market_id,
						direction,
						quote: quote_amount,
						base: base_amount,
					}.into()
				);
			})
	}

	#[test]
	fn fails_to_open_position_if_trade_size_too_small(
		(minimum_trade_size, eps) in min_trade_size_and_eps(as_balance((1, 100)))
	) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = minimum_trade_size;

		let quote_amount = eps.unsigned_abs();
		let direction = match eps.is_positive() {
			true => Direction::Long,
			false => Direction::Short,
		};
		let base_asset_amount_limit = eps; // Arbitrary (price = 1 in this case)

		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				VammPallet::set_price(Some((quote_amount, base_asset_amount_limit).into()));
				assert_noop!(
					TestPallet::open_position(
						Origin::signed(ALICE),
						market_id,
						direction,
						quote_amount,
						base_asset_amount_limit.unsigned_abs()
					),
					Error::<Runtime>::TradeSizeTooSmall
				);
			})
	}

	#[test]
	fn short_trade_can_close_long_position_within_tolerance(
		(minimum_trade_size, eps) in min_trade_size_and_eps(as_balance((1, 100)))
	) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = minimum_trade_size;

		let quote_amount = valid_quote_asset_amount();
		let base_amount_limit = valid_base_asset_amount_limit();

		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				// For event emission
				run_to_block(1);

				let positions_before = TestPallet::get_positions(&ALICE).len();

				// price * base_amount_limit = quote_amount
				VammPallet::set_price(Some((quote_amount, base_amount_limit).into()));
				assert_ok!(TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					Direction::Long,
					quote_amount,
					base_amount_limit,
				));

				// price' * base_amount_limit = (quote_amount + eps)
				VammPallet::set_price(Some(
					((quote_amount as i128 + eps).unsigned_abs(), base_amount_limit).into()
				));
				assert_ok!(TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					Direction::Short,
					quote_amount,
					base_amount_limit,
				));

				assert_eq!(TestPallet::get_positions(&ALICE).len(), positions_before);

				SystemPallet::assert_last_event(
					Event::TradeExecuted {
						market: market_id,
						direction: Direction::Short,
						quote: quote_amount,
						base: base_amount_limit,
					}.into()
				);
		})
	}

	#[test]
	fn long_trade_can_close_short_position_within_tolerance(
		(minimum_trade_size, eps) in min_trade_size_and_eps(as_balance((1, 100)))
	) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = minimum_trade_size;

		let quote_amount = valid_quote_asset_amount();
		let base_amount_limit = valid_base_asset_amount_limit();

		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				// For event emission
				run_to_block(1);

				let positions_before = TestPallet::get_positions(&ALICE).len();

				// price * base_amount_limit = quote_amount
				VammPallet::set_price(Some((quote_amount, base_amount_limit).into()));
				assert_ok!(TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					Direction::Short,
					quote_amount,
					base_amount_limit,
				));

				// price' * base_amount_limit = (quote_amount + eps)
				VammPallet::set_price(Some(
					((quote_amount as i128 + eps).unsigned_abs(), base_amount_limit).into()
				));
				assert_ok!(TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					Direction::Long,
					quote_amount,
					base_amount_limit,
				));

				assert_eq!(TestPallet::get_positions(&ALICE).len(), positions_before);

				SystemPallet::assert_last_event(
					Event::TradeExecuted {
						market: market_id,
						direction: Direction::Long,
						quote: quote_amount,
						base: base_amount_limit,
					}.into()
				);
			})
	}

	#[test]
	fn closing_long_position_with_trade_realizes_pnl(new_price in any_price()) {
		let mut market_id: MarketId = 0;

		let quote_amount = as_balance(100);
		let margin = quote_amount as i128;

		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(valid_market_config()))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				// For event emission
				run_to_block(1);

				let positions_before = TestPallet::get_positions(&ALICE).len();

				VammPallet::set_price(Some(10.into()));
				let base_amount_limit = quote_amount / 10;
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Long,
						quote_amount,
						base_amount_limit,
					),
					base_amount_limit,
				);

				VammPallet::set_price(Some(new_price));
				let new_base_value = new_price.saturating_mul_int(base_amount_limit);
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Short,
						new_base_value,
						base_amount_limit,
					),
					base_amount_limit
				);

				assert_eq!(TestPallet::get_positions(&ALICE).len(), positions_before);
				let pnl = new_base_value as i128 - margin;
				assert_eq!(
					TestPallet::get_margin(&ALICE).unwrap(),
					(margin + pnl) as u128
				);

				SystemPallet::assert_last_event(
					Event::TradeExecuted {
						market: market_id,
						direction: Direction::Short,
						quote: new_base_value,
						base: base_amount_limit,
					}.into()
				);
		})
	}

	#[test]
	fn closing_short_position_with_trade_realizes_pnl(new_price in any_price()) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = 0.into();

		let quote_amount = as_balance(100);
		let margin = quote_amount as i128;

		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				// For event emission
				run_to_block(1);

				let positions_before = TestPallet::get_positions(&ALICE).len();

				VammPallet::set_price(Some(10.into()));
				let base_amount = quote_amount / 10;
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Short,
						quote_amount,
						base_amount,
					),
					base_amount
				);

				VammPallet::set_price(Some(new_price));
				let new_base_value = new_price.saturating_mul_int(base_amount);
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Long,
						new_base_value,
						base_amount,
					),
					base_amount
				);

				assert_eq!(TestPallet::get_positions(&ALICE).len(), positions_before);
				let pnl = margin - new_base_value as i128;
				assert_eq!(
					TestPallet::get_margin(&ALICE).unwrap(),
					(margin + pnl).max(0) as u128
				);

				SystemPallet::assert_last_event(
					Event::TradeExecuted {
						market: market_id,
						direction: Direction::Long,
						quote: new_base_value,
						base: base_amount,
					}.into()
				);
		})
	}

	#[test]
	fn reducing_long_position_partially_realizes_pnl(
		new_price in any_price(),
		percentf in percentage_fraction()
	) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = 0.into();

		let quote_amount = as_balance(100);
		let margin = quote_amount as i128;
		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				// For event emission
				run_to_block(1);

				let positions_before = TestPallet::get_positions(&ALICE).len();

				VammPallet::set_price(Some(10.into()));
				let base_amount = quote_amount / 10;
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Long,
						quote_amount,
						base_amount,
					),
					base_amount
				);


				VammPallet::set_price(Some(new_price));
				// Reduce (close) position by desired percentage
				let base_amount_to_close = percentf.saturating_mul_int(base_amount);
				let base_value_to_close = new_price.saturating_mul_int(base_amount_to_close);
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Short,
						base_value_to_close,
						base_amount_to_close,
					),
					base_amount_to_close,
				);

				let positions = TestPallet::get_positions(&ALICE);
				// Positions remains open
				assert_eq!(positions.len(), positions_before + 1);

				// Fraction of the PnL is realized
				let entry_value = percentf.saturating_mul_int(quote_amount);
				let pnl = base_value_to_close as i128 - entry_value as i128;
				assert_eq!(
					TestPallet::get_margin(&ALICE).unwrap(),
					(margin + pnl) as u128
				);

				let position = positions.iter().find(|p| p.market_id == market_id).unwrap();
				// Position base asset and quote asset notional are cut by percentage
				assert_eq!(
					position.base_asset_amount.into_inner(),
					(base_amount - base_amount_to_close) as i128
				);
				assert_eq!(
					position.quote_asset_notional_amount.into_inner(),
					(quote_amount - entry_value) as i128
				);

				SystemPallet::assert_last_event(
					Event::TradeExecuted {
						market: market_id,
						direction: Direction::Short,
						quote: base_value_to_close,
						base: base_amount_to_close,
					}.into()
				);
			})
	}

	#[test]
	fn reducing_short_position_partially_realizes_pnl(
		new_price in any_price(),
		percentf in percentage_fraction()
	) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = 0.into();

		let quote_amount = as_balance(100);
		let margin = quote_amount as i128;
		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				// For event emission
				run_to_block(1);

				let positions_before = TestPallet::get_positions(&ALICE).len();

				VammPallet::set_price(Some(10.into()));
				// Initial price = 10
				let base_amount = quote_amount / 10;
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Short,
						quote_amount,
						base_amount,
					),
					base_amount
				);

				VammPallet::set_price(Some(new_price));
				// Reduce (close) position by desired percentage
				let base_amount_to_close = percentf.saturating_mul_int(base_amount);
				let base_value_to_close = new_price.saturating_mul_int(base_amount_to_close);
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Long,
						base_value_to_close,
						base_amount_to_close,
					),
					base_amount_to_close
				);

				// Positions remains open
				let positions = TestPallet::get_positions(&ALICE);
				assert_eq!(positions.len(), positions_before + 1);

				// Percentage of the PnL is realized
				let entry_value = percentf.saturating_mul_int(quote_amount);
				let pnl = entry_value as i128 - base_value_to_close as i128;
				assert_eq!(
					TestPallet::get_margin(&ALICE).unwrap(),
					(margin + pnl).max(0) as u128
				);

				let position = positions.iter().find(|p| p.market_id == market_id).unwrap();
				// Position base asset and quote asset notional are cut by percentage
				assert_eq!(
					position.base_asset_amount.into_inner(),
					-((base_amount - base_amount_to_close) as i128)
				);
				assert_eq!(
					position.quote_asset_notional_amount.into_inner(),
					-((quote_amount - entry_value) as i128)
				);

				SystemPallet::assert_last_event(
					Event::TradeExecuted {
						market: market_id,
						direction: Direction::Long,
						quote: base_value_to_close,
						base: base_amount_to_close,
					}.into()
				);
			})
	}

	#[test]
	fn reversing_long_position_realizes_pnl(new_price in any_price()) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = 0.into();

		let quote_amount = as_balance(100);
		let margin = quote_amount as i128;
		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				// For event emission
				run_to_block(1);

				let positions_before = TestPallet::get_positions(&ALICE).len();

				VammPallet::set_price(Some(10.into()));
				let base_amount = quote_amount / 10;
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Long,
						quote_amount,
						base_amount,
					),
					base_amount
				);

				VammPallet::set_price(Some(new_price));
				let new_base_value = new_price.saturating_mul_int(base_amount);
				// We want to end up with the reverse of the position (in base tokens)
				// Now:
				// base = new_base_value
				// Goal:
				// -base = -new_base_value
				// Delta:
				// base * 2 = new_base_value * 2
				let base_delta = base_amount * 2;
				let quote_delta = new_base_value * 2;
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Short,
						quote_delta,
						base_delta,
					),
					base_delta
				);

				// Position remains open
				let positions = TestPallet::get_positions(&ALICE);
				assert_eq!(positions.len(), positions_before + 1);

				let position = positions.iter().find(|p| p.market_id == market_id).unwrap();
				assert_eq!(
					position.base_asset_amount,
					-FixedI128::from_balance(base_amount).unwrap());
				assert_eq!(
					position.quote_asset_notional_amount,
					-FixedI128::from_balance(new_base_value).unwrap()
				);

				// Full PnL is realized
				let pnl = new_base_value as i128 - margin;
				assert_eq!(
					TestPallet::get_margin(&ALICE).unwrap(),
					(margin + pnl) as u128
				);

				SystemPallet::assert_last_event(
					Event::TradeExecuted {
						market: market_id,
						direction: Direction::Short,
						quote: quote_delta,
						base: base_delta,
					}.into()
				);
			})
	}

	#[test]
	fn reversing_short_position_realizes_pnl(new_price in any_price()) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = 0.into();

		let quote_amount = as_balance(100);
		let margin = quote_amount as i128;
		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				// For event emission
				run_to_block(1);

				let positions_before = TestPallet::get_positions(&ALICE).len();

				VammPallet::set_price(Some(10.into()));
				let base_amount = quote_amount / 10;
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Short,
						quote_amount,
						base_amount,
					),
					base_amount
				);

				VammPallet::set_price(Some(new_price));
				let new_base_value = new_price.saturating_mul_int(base_amount);
				// We want to end up with the reverse of the position (in base tokens)
				// Now:
				// -base = -new_base_value
				// Goal:
				// base = new_base_value
				// Delta:
				// -base * 2 = -new_base_value * 2
				let base_delta = base_amount * 2;
				let quote_delta = new_base_value * 2;
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Long,
						quote_delta,
						base_delta,
					),
					base_delta
				);

				// Position remains open
				let positions = TestPallet::get_positions(&ALICE);
				assert_eq!(positions.len(), positions_before + 1);

				let position = positions.iter().find(|p| p.market_id == market_id).unwrap();
				assert_eq!(
					position.base_asset_amount,
					FixedI128::from_balance(base_amount).unwrap()
				);
				assert_eq!(
					position.quote_asset_notional_amount,
					FixedI128::from_balance(new_base_value).unwrap()
				);

				// Full PnL is realized
				let pnl = margin - new_base_value as i128;
				assert_eq!(
					TestPallet::get_margin(&ALICE).unwrap(),
					(margin + pnl).max(0) as u128
				);

				SystemPallet::assert_last_event(
					Event::TradeExecuted {
						market: market_id,
						direction: Direction::Long,
						quote: quote_delta,
						base: base_delta,
					}.into()
				);
			})
	}

	#[test]
	fn fails_to_create_new_position_without_enough_margin(
		direction in any_direction(),
		excess in 1..as_balance(1_000_000),
	) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.margin_ratio_initial = (1, 10).into();  // 1/10 IMR, or 10x leverage

		let margin = as_balance(10);
		let quote_amount = as_balance(100) + excess; // Over 10x margin

		ExtBuilder { balances: vec![(ALICE, USDC, margin)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, margin)
			.execute_with(|| {
				VammPallet::set_price(Some(10.into()));
				let base_amount_limit = quote_amount / 10;
				assert_noop!(
					TestPallet::open_position(
						Origin::signed(ALICE),
						market_id,
						direction,
						quote_amount,
						base_amount_limit,
					),
					Error::<Runtime>::InsufficientCollateral,
				);
			})
	}

	#[test]
	fn succeeds_in_creating_new_position_with_enough_margin(
		direction in any_direction(),
		max_leverage_percent in 100..2_000_u128,  // Anywhere from 1x to 20x margin
		percentf in percentage_fraction()
	) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.margin_ratio_initial = (100, max_leverage_percent).into();

		let margin = as_balance(10);
		let quote_amount_max = market_config
			.margin_ratio_initial
			.reciprocal()
			.unwrap()
			.saturating_mul_int(margin);
		let quote_amount = percentf.saturating_mul_int(quote_amount_max);

		ExtBuilder { balances: vec![(ALICE, USDC, margin)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, margin)
			.execute_with(|| {
				VammPallet::set_price(Some(10.into()));
				let base_amount_limit = quote_amount / 10;
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						direction,
						quote_amount,
						base_amount_limit
					),
					base_amount_limit,
				);
			})
	}

	#[test]
	fn can_decrease_position_even_if_below_imr(direction in any_direction()) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.margin_ratio_initial = (1, 10).into();  // 1/10 IMR, or 10x leverage

		let margin = as_balance(10);
		let quote_amount = as_balance(100); // 10x margin => max leverage

		ExtBuilder { balances: vec![(ALICE, USDC, margin)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, margin)
			.execute_with(|| {
				VammPallet::set_price(Some(10.into()));
				let base_amount_limit = quote_amount / 10;
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						direction,
						quote_amount,
						base_amount_limit
					),
					base_amount_limit,
				);

				let new_price: FixedU128 = match direction {
					Direction::Long => 8, // decrease price => negative PnL
					Direction::Short => 12, // increase price => negative PnL
				}.into();
				VammPallet::set_price(Some(new_price));
				let new_base_value = new_price.saturating_mul_int(base_amount_limit);
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						match direction {
							Direction::Long => Direction::Short,
							Direction::Short => Direction::Long,
						},
						new_base_value / 2,
						base_amount_limit / 2,
					),
					base_amount_limit / 2,
				);
			})
	}

	// TODO(0xangelo): reversing should check IMR if needed

	#[test]
	fn imr_is_combination_of_market_imrs_with_open_positions(direction in any_direction()) {
		let mut market_ids = Vec::<_>::new();
		let mut configs = Vec::<_>::new();
		let mut market_config = valid_market_config();
		market_config.margin_ratio_initial = (1, 10).into(); // 10x leverage
		configs.push(Some(market_config.clone()));
		market_config.margin_ratio_initial = (1, 20).into(); // 20x leverage
		configs.push(Some(market_config));

		let margin = as_balance(60);

		ExtBuilder { balances: vec![(ALICE, USDC, margin)], ..Default::default() }
			.build()
			.init_markets(&mut market_ids, configs.into_iter())
			.add_margin(&ALICE, USDC, margin)
			.execute_with(|| {
				let price = 10;
				VammPallet::set_price(Some(price.into()));

				// Since the two markets have 10x and 20x max leverage respectively, the first has
				// two times more margin requirement than the second. Thus, it has double the weight
				// in calculating the account's max leverage. By splitting one third of our total
				// exposure in the first market and the rest in the second, we can have 15x max
				// leverage for our account.
				let quote_amount = as_balance(300); // (15 x 60 = 900)
				let base_amount = quote_amount / price;
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_ids[0],
						direction,
						quote_amount,
						base_amount,
					),
					base_amount,
				);

				// For second market
				let quote_amount = as_balance(600);
				let base_amount = quote_amount / price;
				// This should exceed the max leverage and fail
				let quote_amount_fail = quote_amount + 100;
				let base_amount_fail = quote_amount_fail / price;
				assert_noop!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_ids[1],
						direction,
						quote_amount_fail,
						base_amount_fail,
					),
					Error::<Runtime>::InsufficientCollateral
				);

				// This should succeed (max leverage)
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_ids[1],
						direction,
						quote_amount,
						base_amount,
					),
					base_amount
				);
			})
	}
}
