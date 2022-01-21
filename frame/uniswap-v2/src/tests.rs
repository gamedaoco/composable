use composable_support::validation::Validate;
use frame_support::assert_ok;

use crate::{mock::*, Error};
use composable_traits::dex::CurveAmm as ConstantProductAmmTrait;
use frame_support::traits::fungibles::{Inspect, Mutate};
use sp_runtime::Permill;

#[test]
fn spot_price_should_work() {
	let cases = vec![
		(1000, 2000, 500, 500, 100, Ok(200), "Easy case"),
		(1, 0, 1, 1, 1, Ok(0), "Zero buy_reserve"),
		(1, 1, 0, 1, 1, Ok(0), "Zero amount"),
		(Balance::MAX, Balance::MAX - 1, 1, 1, 1, Ok(0), "Truncated result"),
		(
			1,
			Balance::MAX,
			Balance::MAX, // lbp
			Balance::MAX, // lbp
			Balance::MAX,
			Err(Error::<Test>::Overflow),
			"Overflow weights",
		),
	];

	for case in cases {
		assert_eq!(
			crate::lbp::spot_price(case.0.validated().unwrap(), case.1, case.2, case.3, case.4),
			case.5,
			"{}",
			case.6
		);
	}
}

#[test]
fn out_given_in_should_work() {
	let cases: Vec<(_, _, _, _, _, Result<Balance, Error<Test>>, _)> = vec![
		(1000, 2000, 500, 500, 100, Ok(178), "Easy case"),
		(
			Balance::MAX,
			Balance::MAX,
			Balance::MAX, // lbp
			Balance::MAX, // lbp
			Balance::MAX,
			Ok(170141183460469231731687303715884105726),
			"max",
		),
		(1, 1, 1, 1, 0, Ok(0), "Zero out reserve and amount"),
	];

	for case in cases {
		assert_eq!(
			crate::lbp::calculate_out_given_in(
				case.0.validated().unwrap(),
				case.1.validated().unwrap(),
				case.2.validated().unwrap(),
				case.3.validated().unwrap(),
				case.4.validated().unwrap()
			),
			case.5,
			"{}",
			case.6
		);
	}
}

#[test]
fn add_remove_liquidity() {
	new_test_ext().execute_with(|| {
		// ConstantProductAmm configurations
		let assets = vec![MockCurrencyId::USDC, MockCurrencyId::USDT];
		let fee = Permill::zero();
		let admin_fee = Permill::zero();

		// Mint USDT for ALICE
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &ALICE, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 200000);
		// Mint USDC for ALICE
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &ALICE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDC, &ALICE, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &ALICE), 200000);
		// Mint USDT for BOB
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &BOB, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 200000);
		// Mint USDC for BOB
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &BOB), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDC, &BOB, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &BOB), 200000);

		// Create ConstantProductAmm pool
		let p = ConstantProductAmm::create_pool(&ALICE, assets, fee, admin_fee);
		assert_ok!(&p);
		let pool_id = p.unwrap();
		let pool = ConstantProductAmm::get_pool_info(pool_id);
		assert!(pool.is_some());
		let pool = pool.unwrap();
		let pool_lp_asset = pool.lp_token;
		// 1 USDC = 1 USDT

		// Add liquidity from ALICE's account to pool
		let amounts = vec![130000u128, 130000u128];
		assert_ok!(ConstantProductAmm::add_liquidity(&ALICE, pool_id, amounts.clone(), 0u128));
		let alice_balance = Tokens::balance(pool_lp_asset, &ALICE);
		assert_ne!(alice_balance, 0);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 200000 - 130000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &ALICE), 200000 - 130000);
		let pool = ConstantProductAmm::get_pool_info(pool_id);
		assert!(pool.is_some());

		// Add liquidity from BOB's account to pool
		assert_ok!(ConstantProductAmm::add_liquidity(&BOB, pool_id, amounts.clone(), 0u128));
		let bob_balance = Tokens::balance(pool_lp_asset, &BOB);
		assert_ne!(bob_balance, 0);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 200000 - 130000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &BOB), 200000 - 130000);
		let min_amt = vec![0u128, 0u128];

		// Check that pool has USDT and USDC transferred from ALICE and BOB
		assert_eq!(
			Tokens::balance(MockCurrencyId::USDC, &ConstantProductAmm::account_id(&pool_id)),
			260000
		);
		assert_eq!(
			Tokens::balance(MockCurrencyId::USDT, &ConstantProductAmm::account_id(&pool_id)),
			260000
		);

		// Withdraw ALICE"s fund from pool.
		assert_ok!(ConstantProductAmm::remove_liquidity(
			&ALICE,
			pool_id,
			alice_balance,
			min_amt.clone()
		));
		// Check balances which should be impacted.
		assert_eq!(Tokens::balance(pool_lp_asset, &ALICE), 0);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 200000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &ALICE), 200000);
		assert_eq!(
			Tokens::balance(MockCurrencyId::USDC, &ConstantProductAmm::account_id(&pool_id)),
			130000
		);
		assert_eq!(
			Tokens::balance(MockCurrencyId::USDT, &ConstantProductAmm::account_id(&pool_id)),
			130000
		);

		// Withdraw BOB"s fund from pool.
		assert_ok!(ConstantProductAmm::remove_liquidity(
			&BOB,
			pool_id,
			bob_balance,
			min_amt.clone()
		));
		// Check balances which should be impacted.
		assert_eq!(Tokens::balance(pool_lp_asset, &BOB), 0);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 200000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &BOB), 200000);
		assert_eq!(
			Tokens::balance(MockCurrencyId::USDC, &ConstantProductAmm::account_id(&pool_id)),
			0
		);
		assert_eq!(
			Tokens::balance(MockCurrencyId::USDT, &ConstantProductAmm::account_id(&pool_id)),
			0
		);
	});
}

#[test]
fn exchange_test() {
	new_test_ext().execute_with(|| {
		// ConstantProductAmm configurations
		let assets = vec![MockCurrencyId::USDC, MockCurrencyId::USDT];
		let fee = Permill::zero();
		let admin_fee = Permill::zero();

		// Mint USDT for ALICE
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &ALICE, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 200000);
		// Mint USDC for ALICE
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &ALICE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDC, &ALICE, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &ALICE), 200000);
		// Mint USDT for BOB
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &BOB, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 200000);
		// Mint USDC for BOB
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &BOB), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDC, &BOB, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &BOB), 200000);
		// Mint USDT for CHARLIE
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &CHARLIE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &CHARLIE, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &CHARLIE), 200000);

		// Create ConstantProductAmm pool
		let p = ConstantProductAmm::create_pool(&ALICE, assets, fee, admin_fee);
		assert_ok!(&p);
		let pool_id = p.unwrap();
		let pool = ConstantProductAmm::get_pool_info(pool_id);
		assert!(pool.is_some());
		let pool = pool.unwrap();
		let pool_lp_asset = pool.lp_token;
		// 1 USDC = 1 USDT
		// Add liquidity from ALICE's account to pool
		let amounts = vec![130000u128, 130000u128];
		assert_ok!(ConstantProductAmm::add_liquidity(&ALICE, pool_id, amounts.clone(), 0u128));
		let alice_balance = Tokens::balance(pool_lp_asset, &ALICE);
		assert_ne!(alice_balance, 0);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 200000 - 130000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &ALICE), 200000 - 130000);
		let pool = ConstantProductAmm::get_pool_info(pool_id);
		assert!(pool.is_some());
		// Add liquidity from BOB's account to pool
		assert_ok!(ConstantProductAmm::add_liquidity(&BOB, pool_id, amounts.clone(), 0u128));
		let bob_balance = Tokens::balance(pool_lp_asset, &BOB);
		assert_ne!(bob_balance, 0);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 200000 - 130000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &BOB), 200000 - 130000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &CHARLIE), 0);
		// CHARLIE exchanges USDT for USDC
		assert_ok!(ConstantProductAmm::exchange(&CHARLIE, pool_id, 1, 0, 65000, 0));
		sp_std::if_std! {
			println!("CHARLIE's USDC balance {:?}" , Tokens::balance(MockCurrencyId::USDC, &CHARLIE));
		}
		assert!(65000 >= Tokens::balance(MockCurrencyId::USDC, &CHARLIE));
	});
}
