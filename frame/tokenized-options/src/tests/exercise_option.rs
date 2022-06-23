use crate::mock::runtime::{
	Assets, Balance, Balances, Event, ExtBuilder, MockRuntime, Moment, Origin, System,
	TokenizedOptions, Vault,
};

use crate::mock::{accounts::*, assets::*};

use crate::{
	pallet::{self, OptionHashToOptionId, Sellers},
	tests::{
		buy_option::buy_option_success_checks,
		delete_sell_option::delete_sell_option_success_checks,
		sell_option::sell_option_success_checks, settle_options::settle_options_success_checks, *,
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
//		Exercise Options Tests
// ----------------------------------------------------------------------------------------------------
pub fn exercise_option_success_checks(option_id: AssetId, option_amount: Balance, who: Public) {
	// Get info before extrinsic for checks
	let _option = OptionIdToOption::<MockRuntime>::get(option_id).unwrap();

	// Call extrinsic
	assert_ok!(TokenizedOptions::exercise_option(Origin::signed(who), option_amount, option_id));

	// Check correct event
	System::assert_last_event(Event::TokenizedOptions(pallet::Event::ExerciseOption {
		user: who,
		option_amount,
		option_id,
	}));
}

#[test]
fn test_exercise_option_with_initialization_success() {
	ExtBuilder::default()
		.initialize_balances(Vec::from([
			(ALICE, BTC, 1 * UNIT),
			(ALICE, USDC, 50000 * UNIT),
			(BOB, BTC, 1 * UNIT),
			(BOB, USDC, 50000 * UNIT),
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
			let option_id = OptionHashToOptionId::<MockRuntime>::get(option_hash).unwrap();

			// Sell option and make checks
			let option_amount = 1u128;
			sell_option_success_checks(option_id, option_amount, BOB);

			// Go to purchase window
			run_to_block(3);

			// Buy option
			buy_option_success_checks(option_id, option_amount, ALICE);

			// BTC price moves from 50k to 55k
			set_oracle_price(option_config.base_asset_id, 55000u128 * UNIT);

			// Go to exercise window
			run_to_block(6);

			// Exercise option
			exercise_option_success_checks(option_id, option_amount, ALICE);
		});
}