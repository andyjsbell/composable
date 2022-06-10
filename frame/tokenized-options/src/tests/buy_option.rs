use crate::mock::runtime::{
	Assets, Balance, Balances, Event, ExtBuilder, MockRuntime, Moment, Origin, System,
	TokenizedOptions, Vault,
};

use crate::mock::{accounts::*, assets::*};

use crate::{
	pallet::{self, OptionHashToOptionId, Sellers},
	tests::{
		delete_sell_option::delete_sell_option_success_checks,
		sell_option::sell_option_success_checks, *,
	},
};

use composable_traits::vault::CapabilityVault;
use composable_traits::{
	tokenized_options::TokenizedOptions as TokenizedOptionsTrait, vault::Vault as VaultTrait,
};
use frame_support::{assert_err, assert_noop, assert_ok, traits::fungibles::Inspect};

use frame_system::ensure_signed;
use sp_core::{sr25519::Public, H256};
use sp_runtime::ArithmeticError;

// ----------------------------------------------------------------------------------------------------
//		Sell Options Tests
// ----------------------------------------------------------------------------------------------------

pub fn buy_option_success_checks(
	option_hash: H256,
	_option_config: OptionConfig<AssetId, Balance, Moment>,
	option_amount: Balance,
	who: Public,
) {
	// Get info before extrinsic for checks
	let option_id = OptionHashToOptionId::<MockRuntime>::get(option_hash).unwrap();
	let asset_id = USDC;

	let protocol_account = TokenizedOptions::account_id(asset_id);

	let initial_issuance_buyer =
		OptionIdToOption::<MockRuntime>::get(option_id).unwrap().total_issuance_buyer;
	let initial_user_balance_options = Assets::balance(option_id, &who);
	let initial_user_balance = Assets::balance(asset_id, &who);
	let initial_protocol_balance = Assets::balance(asset_id, &protocol_account);

	// Call extrinsic
	assert_ok!(TokenizedOptions::buy_option(Origin::signed(who), option_amount, option_id));

	// Check correct event
	System::assert_last_event(Event::TokenizedOptions(pallet::Event::BuyOption {
		buyer: who,
		option_amount,
		option_id,
	}));

	let option_premium = TokenizedOptions::fake_option_price().unwrap() * option_amount;

	// Check buyer balance after sale has premium subtracted
	assert_eq!(Assets::balance(asset_id, &who), initial_user_balance - option_premium);

	// Check protocol balance after purchase is correct
	assert_eq!(
		Assets::balance(asset_id, &protocol_account),
		initial_protocol_balance + option_premium
	);

	// Check user owns the correct issuance of option token
	assert_eq!(Assets::balance(option_id, &who), initial_user_balance_options + option_amount);

	// Check position is updated correctly
	let updated_issuance_buyer = OptionIdToOption::<MockRuntime>::try_get(option_id)
		.unwrap()
		.total_issuance_buyer;

	assert_eq!(updated_issuance_buyer, initial_issuance_buyer + option_amount)
}

#[test]
fn test_buy_option_with_initialization_success() {
	ExtBuilder::default()
		.initialize_balances(Vec::from([
			(ALICE, BTC, 1 * 10u128.pow(12)),
			(ALICE, USDC, 50000 * 10u128.pow(12)),
			(BOB, BTC, 1 * 10u128.pow(12)),
			(BOB, USDC, 50000 * 10u128.pow(12)),
		]))
		.build()
		.initialize_oracle_prices()
		.execute_with(|| {
			// Get BTC and USDC vault config
			let btc_vault_config = VaultConfigBuilder::default().build();
			let usdc_vault_config = VaultConfigBuilder::default().asset_id(USDC).build();

			// Create BTC and USDC vaults
			assert_ok!(TokenizedOptions::create_asset_vault(
				Origin::signed(ADMIN),
				btc_vault_config
			));

			assert_ok!(TokenizedOptions::create_asset_vault(
				Origin::signed(ADMIN),
				usdc_vault_config
			));

			// Create default BTC option
			let option_config = OptionsConfigBuilder::default().build();

			assert_ok!(TokenizedOptions::create_option(
				Origin::signed(ADMIN),
				option_config.clone()
			));

			let option_hash = TokenizedOptions::generate_id(
				option_config.base_asset_id,
				option_config.quote_asset_id,
				option_config.base_asset_strike_price,
				option_config.quote_asset_strike_price,
				option_config.option_type,
				option_config.expiring_date,
				option_config.exercise_type,
			);

			// Check creation ended correctly
			assert!(OptionHashToOptionId::<MockRuntime>::contains_key(option_hash));

			// Sell option and make checks
			let option_amount = 1u128;
			sell_option_success_checks(option_hash, option_config.clone(), option_amount, BOB);

			// Go to purchase window
			run_to_block(3);

			// Buy option
			buy_option_success_checks(option_hash, option_config, option_amount, ALICE);
		});
}

#[test]
fn test_buy_option_success() {
	ExtBuilder::default()
		.initialize_balances(Vec::from([
			(ALICE, BTC, 3 * 10u128.pow(12)),
			(ALICE, USDC, 150000 * 10u128.pow(12)),
			(BOB, BTC, 5 * 10u128.pow(12)),
			(BOB, USDC, 250000 * 10u128.pow(12)),
		]))
		.build()
		.initialize_oracle_prices()
		.initialize_all_vaults()
		.initialize_all_options()
		.execute_with(|| {
			let option_config = OptionsConfigBuilder::default().build();

			let option_hash = TokenizedOptions::generate_id(
				option_config.base_asset_id,
				option_config.quote_asset_id,
				option_config.base_asset_strike_price,
				option_config.quote_asset_strike_price,
				option_config.option_type,
				option_config.expiring_date,
				option_config.exercise_type,
			);

			assert!(OptionHashToOptionId::<MockRuntime>::contains_key(option_hash));

			let bob_option_amount = 5u128;
			let alice_option_amount = 3u128;

			sell_option_success_checks(option_hash, option_config.clone(), bob_option_amount, BOB);
			run_to_block(3);
			buy_option_success_checks(option_hash, option_config, alice_option_amount, ALICE);
		});
}

#[test]
fn test_buy_option_multiple_times() {
	ExtBuilder::default()
		.initialize_balances(Vec::from([
			(ALICE, BTC, 5 * 10u128.pow(12)),
			(ALICE, USDC, 250000 * 10u128.pow(12)),
			(BOB, BTC, 5 * 10u128.pow(12)),
			(BOB, USDC, 250000 * 10u128.pow(12)),
		]))
		.build()
		.initialize_oracle_prices()
		.initialize_all_vaults()
		.initialize_all_options()
		.execute_with(|| {
			let option_config = OptionsConfigBuilder::default().build();

			let option_hash = TokenizedOptions::generate_id(
				option_config.base_asset_id,
				option_config.quote_asset_id,
				option_config.base_asset_strike_price,
				option_config.quote_asset_strike_price,
				option_config.option_type,
				option_config.expiring_date,
				option_config.exercise_type,
			);

			assert!(OptionHashToOptionId::<MockRuntime>::contains_key(option_hash));

			let bob_option_amount = 5u128;
			let alice_option_amount = 3u128;

			sell_option_success_checks(option_hash, option_config.clone(), bob_option_amount, BOB);

			run_to_block(3);

			buy_option_success_checks(
				option_hash,
				option_config.clone(),
				alice_option_amount,
				ALICE,
			);

			let alice_option_amount = 2u128;
			buy_option_success_checks(option_hash, option_config, alice_option_amount, ALICE);
		});
}

#[test]
fn test_buy_option_multiple_users() {
	ExtBuilder::default()
		.initialize_balances(Vec::from([
			(ALICE, BTC, 5 * 10u128.pow(12)),
			(ALICE, USDC, 250000 * 10u128.pow(12)),
			(BOB, BTC, 5 * 10u128.pow(12)),
			(BOB, USDC, 250000 * 10u128.pow(12)),
			(CHARLIE, BTC, 5 * 10u128.pow(12)),
			(CHARLIE, USDC, 250000 * 10u128.pow(12)),
		]))
		.build()
		.initialize_oracle_prices()
		.initialize_all_vaults()
		.initialize_all_options()
		.execute_with(|| {
			let option_config = OptionsConfigBuilder::default().build();

			let option_hash = TokenizedOptions::generate_id(
				option_config.base_asset_id,
				option_config.quote_asset_id,
				option_config.base_asset_strike_price,
				option_config.quote_asset_strike_price,
				option_config.option_type,
				option_config.expiring_date,
				option_config.exercise_type,
			);

			assert!(OptionHashToOptionId::<MockRuntime>::contains_key(option_hash));

			let bob_option_amount = 5u128;
			let alice_option_amount = 3u128;
			let charlie_option_amount = 4u128;

			sell_option_success_checks(option_hash, option_config.clone(), bob_option_amount, BOB);
			sell_option_success_checks(
				option_hash,
				option_config.clone(),
				charlie_option_amount,
				CHARLIE,
			);

			run_to_block(3);

			buy_option_success_checks(
				option_hash,
				option_config.clone(),
				alice_option_amount,
				ALICE,
			);

			let charlie_option_amount = 2u128;

			buy_option_success_checks(
				option_hash,
				option_config.clone(),
				charlie_option_amount,
				CHARLIE,
			);
			let alice_option_amount = 2u128;
			buy_option_success_checks(option_hash, option_config, alice_option_amount, ALICE);
		});
}

#[test]
fn test_buy_option_error_option_not_exists() {
	ExtBuilder::default()
		.initialize_balances(Vec::from([
			(ALICE, BTC, 5 * 10u128.pow(12)),
			(ALICE, USDC, 250000 * 10u128.pow(12)),
			(BOB, BTC, 5 * 10u128.pow(12)),
			(BOB, USDC, 250000 * 10u128.pow(12)),
		]))
		.build()
		.execute_with(|| {
			assert_noop!(
				// 10000000000005u128 it's a meaningless number
				TokenizedOptions::buy_option(
					Origin::signed(BOB),
					1u128,
					AssetId(10000000000005u128)
				),
				Error::<MockRuntime>::OptionDoesNotExists
			);
		});
}

#[test]
fn test_buy_option_error_not_into_purchase_window() {
	ExtBuilder::default()
		.initialize_balances(Vec::from([
			(ALICE, BTC, 5 * 10u128.pow(12)),
			(ALICE, USDC, 250000 * 10u128.pow(12)),
			(BOB, BTC, 5 * 10u128.pow(12)),
			(BOB, USDC, 250000 * 10u128.pow(12)),
		]))
		.build()
		.initialize_oracle_prices()
		.initialize_all_vaults()
		.initialize_all_options()
		.execute_with(|| {
			let option_config = OptionsConfigBuilder::default().build();

			let option_hash = TokenizedOptions::generate_id(
				option_config.base_asset_id,
				option_config.quote_asset_id,
				option_config.base_asset_strike_price,
				option_config.quote_asset_strike_price,
				option_config.option_type,
				option_config.expiring_date,
				option_config.exercise_type,
			);

			assert!(OptionHashToOptionId::<MockRuntime>::contains_key(option_hash));

			let bob_option_amount = 5u128;
			let alice_option_amount = 2u128;

			sell_option_success_checks(option_hash, option_config.clone(), bob_option_amount, BOB);

			// Purchase window goes from block 3 <= x < 6. Now we are in block 3.
			let option_id = OptionHashToOptionId::<MockRuntime>::get(option_hash).unwrap();

			assert_noop!(
				TokenizedOptions::buy_option(Origin::signed(ALICE), alice_option_amount, option_id),
				Error::<MockRuntime>::NotIntoPurchaseWindow
			);

			// Now it should work
			run_to_block(3);
			buy_option_success_checks(option_hash, option_config, alice_option_amount, ALICE);

			// Now we are out of purchase window again and should fail
			run_to_block(6);
			assert_noop!(
				TokenizedOptions::buy_option(Origin::signed(ALICE), alice_option_amount, option_id),
				Error::<MockRuntime>::NotIntoPurchaseWindow
			);
		});
}

#[test]
fn test_buy_option_error_user_has_not_enough_funds() {
	ExtBuilder::default()
		.initialize_balances(Vec::from([
			(ALICE, BTC, 3 * 10u128.pow(12)),
			(ALICE, USDC, 3000 * 10u128.pow(12)),
			(BOB, BTC, 5 * 10u128.pow(12)),
			(BOB, USDC, 250000 * 10u128.pow(12)),
		]))
		.build()
		.initialize_oracle_prices()
		.initialize_all_vaults()
		.initialize_all_options()
		.execute_with(|| {
			let option_config = OptionsConfigBuilder::default().build();

			let option_hash = TokenizedOptions::generate_id(
				option_config.base_asset_id,
				option_config.quote_asset_id,
				option_config.base_asset_strike_price,
				option_config.quote_asset_strike_price,
				option_config.option_type,
				option_config.expiring_date,
				option_config.exercise_type,
			);

			assert!(OptionHashToOptionId::<MockRuntime>::contains_key(option_hash));

			let bob_option_amount = 5u128;
			sell_option_success_checks(option_hash, option_config.clone(), bob_option_amount, BOB);

			run_to_block(3);

			let option_id = OptionHashToOptionId::<MockRuntime>::get(option_hash).unwrap();

			let alice_option_amount = 4u128; // Each option costs 1000 USDC, Alice has 3000

			assert_noop!(
				TokenizedOptions::buy_option(Origin::signed(ALICE), alice_option_amount, option_id),
				Error::<MockRuntime>::UserHasNotEnoughFundsToDeposit
			);

			let alice_option_amount = 3u128; // Each option costs 1000 USDC, Alice has 3000

			// Counter test
			buy_option_success_checks(option_hash, option_config, alice_option_amount, ALICE);
		});
}

#[test]
fn test_buy_option_error_cannot_buy_zero_options() {
	ExtBuilder::default()
		.initialize_balances(Vec::from([
			(ALICE, BTC, 5 * 10u128.pow(12)),
			(ALICE, USDC, 250000 * 10u128.pow(12)),
			(BOB, BTC, 5 * 10u128.pow(12)),
			(BOB, USDC, 250000 * 10u128.pow(12)),
		]))
		.build()
		.initialize_oracle_prices()
		.initialize_all_vaults()
		.initialize_all_options()
		.execute_with(|| {
			let option_config = OptionsConfigBuilder::default().build();

			let option_hash = TokenizedOptions::generate_id(
				option_config.base_asset_id,
				option_config.quote_asset_id,
				option_config.base_asset_strike_price,
				option_config.quote_asset_strike_price,
				option_config.option_type,
				option_config.expiring_date,
				option_config.exercise_type,
			);

			assert!(OptionHashToOptionId::<MockRuntime>::contains_key(option_hash));

			let bob_option_amount = 5u128;
			sell_option_success_checks(option_hash, option_config, bob_option_amount, BOB);

			run_to_block(3);

			let option_id = OptionHashToOptionId::<MockRuntime>::get(option_hash).unwrap();

			assert_noop!(
				TokenizedOptions::buy_option(Origin::signed(ALICE), 0u128, option_id),
				Error::<MockRuntime>::CannotPassZeroOptionAmount
			);
		});
}

#[test]
fn test_buy_option_error_overflow_asset_amount() {
	ExtBuilder::default()
		.initialize_balances(Vec::from([
			(ALICE, BTC, 5 * 10u128.pow(12)),
			(ALICE, USDC, 250000 * 10u128.pow(12)),
			(BOB, BTC, 5 * 10u128.pow(12)),
			(BOB, USDC, 250000 * 10u128.pow(12)),
		]))
		.build()
		.initialize_oracle_prices()
		.initialize_all_vaults()
		.initialize_all_options()
		.execute_with(|| {
			let option_config = OptionsConfigBuilder::default().build();

			let option_hash = TokenizedOptions::generate_id(
				option_config.base_asset_id,
				option_config.quote_asset_id,
				option_config.base_asset_strike_price,
				option_config.quote_asset_strike_price,
				option_config.option_type,
				option_config.expiring_date,
				option_config.exercise_type,
			);

			assert!(OptionHashToOptionId::<MockRuntime>::contains_key(option_hash));

			let bob_option_amount = 5u128;
			sell_option_success_checks(option_hash, option_config, bob_option_amount, BOB);

			run_to_block(3);

			let option_id = OptionHashToOptionId::<MockRuntime>::get(option_hash).unwrap();

			// Balance: u128 contains until ~4 * 10^38. Considering 12 decimals,
			// the asset_amount to transfer should overflow with option amount > 3 * 10^26.
			// The fake option cost right now is fixed at 1000 USDC, so
			// option amount should be > 3 * 10^23 to cause overflow.
			// It works until 3 * 10^23.
			let alice_option_amount = 4 * 10u128.pow(23);

			assert_noop!(
				TokenizedOptions::buy_option(Origin::signed(ALICE), alice_option_amount, option_id),
				ArithmeticError::Overflow
			);
		});
}

#[test]
fn test_buy_option_error_not_enough_options_for_sale() {
	ExtBuilder::default()
		.initialize_balances(Vec::from([
			(ALICE, BTC, 5 * 10u128.pow(12)),
			(ALICE, USDC, 250000 * 10u128.pow(12)),
			(BOB, BTC, 5 * 10u128.pow(12)),
			(BOB, USDC, 250000 * 10u128.pow(12)),
		]))
		.build()
		.initialize_oracle_prices()
		.initialize_all_vaults()
		.initialize_all_options()
		.execute_with(|| {
			let option_config = OptionsConfigBuilder::default().build();

			let option_hash = TokenizedOptions::generate_id(
				option_config.base_asset_id,
				option_config.quote_asset_id,
				option_config.base_asset_strike_price,
				option_config.quote_asset_strike_price,
				option_config.option_type,
				option_config.expiring_date,
				option_config.exercise_type,
			);

			assert!(OptionHashToOptionId::<MockRuntime>::contains_key(option_hash));

			let bob_option_amount = 5u128;
			let alice_option_amount = 3u128;

			sell_option_success_checks(option_hash, option_config.clone(), bob_option_amount, BOB);

			run_to_block(3);

			buy_option_success_checks(option_hash, option_config, alice_option_amount, ALICE);

			let option_id = OptionHashToOptionId::<MockRuntime>::get(option_hash).unwrap();

			assert_noop!(
				TokenizedOptions::buy_option(Origin::signed(ALICE), alice_option_amount, option_id),
				Error::<MockRuntime>::NotEnoughOptionsForSale
			);
		});
}
