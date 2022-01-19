//! Functions testing if we have correctly forwarding calls to the right pallet implementation.

use crate::*;
use composable_tests_helpers::prop_assert_ok;
use mocks::{
	new_test_ext, AccountId, Balance, Test, ACCOUNT_FREE_START, BALANCES, MINIMUM_BALANCE,
};
use proptest::prelude::*;

prop_compose! {
	fn valid_amounts_without_overflow_k(max_accounts: usize, limit: Balance)
		(balances in prop::collection::vec(MINIMUM_BALANCE..limit, 3..max_accounts))
		 -> Vec<(AccountId, Balance)> {
			(ACCOUNT_FREE_START..balances.len() as AccountId)
				.zip(balances)
				.collect()
		}
}

prop_compose! {
	fn valid_amounts_without_overflow_1()
		(x in MINIMUM_BALANCE..Balance::MAX) -> Balance {
		x
	}
}

prop_compose! {
	fn valid_amounts_without_overflow_2()
		(x in MINIMUM_BALANCE..Balance::MAX / 2,
		 y in MINIMUM_BALANCE..Balance::MAX / 2) -> (Balance, Balance) {
			(x, y)
	}
}

prop_compose! {
	fn valid_amounts_without_overflow_3()
		(x in MINIMUM_BALANCE..Balance::MAX / 3,
		 y in MINIMUM_BALANCE..Balance::MAX / 3,
		 z in MINIMUM_BALANCE..Balance::MAX / 3) -> (Balance, Balance, Balance) {
			(x, y, z)
		}
}

prop_compose! {
	fn accounts()
		(x in ACCOUNT_FREE_START..AccountId::MAX) -> AccountId {
			x
		}
}

prop_compose! {
	fn accounts_2()
		(x in ACCOUNT_FREE_START..AccountId::MAX / 2, y in (AccountId::MAX/2)+1..AccountId::MAX) -> (AccountId, AccountId) {
			(x, y)
		}
}

mod currency {
	use super::*;
	use frame_support::traits::{
		tokens::{currency::Currency, Imbalance},
		ExistenceRequirement, WithdrawReasons,
	};
	proptest! {
			#![proptest_config(ProptestConfig::with_cases(10000))]

			/// Covers all the methods from the currency trait.
			#[test]
			fn test_trait_implementation(
				account in accounts(),
				(first, second, third) in valid_amounts_without_overflow_3()
			) {
		   new_test_ext().execute_with(|| {
			macro_rules! assert_issuance {
			($val:expr) => {
				let issuance = BALANCES.iter().fold(0, | sum, (_, val)| val + sum);
				prop_assert_eq!(
				<Pallet::<Test> as Currency<AccountId>>::total_issuance(),
					issuance + $val
			  );
			}
		}

	prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::minimum_balance(), MINIMUM_BALANCE);
	prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::total_balance(&account), 0);
	prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account), 0);
	assert_issuance!(0);
	prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_creating(&account,first).peek(), first);
	prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::total_balance(&account), first);
	prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account), first);
	assert_issuance!(first);
	prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_into_existing(&account,
	second).unwrap().peek(), second);
	prop_assert_eq!(<Pallet::<Test> as
	Currency<AccountId>>::total_balance(&account), first + second);
	prop_assert_eq!(<Pallet::<Test>
	as Currency<AccountId>>::free_balance(&account), first + second);
	assert_issuance!(first + second);

	prop_assert!(<Pallet::<Test> as Currency<AccountId>>::can_slash(&account, first + second));
	let (_, difference) = <Pallet::<Test> as Currency<AccountId>>::slash(&account, third);
	let balance = if first + second > third {
		prop_assert_eq!(difference, 0);
		let balance = (first + second) - third;
		prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::total_balance(&account), balance);
		assert_issuance!(balance);
		balance
	} else {
		prop_assert_eq!(difference, third - (first + second));
		prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::total_balance(&account), 0);
		assert_issuance!(0);
		0
	};

	let issue = <Pallet::<Test> as Currency<AccountId>>::issue(second);
	let added = issue.peek();
	prop_assert_eq!(added, second);
	if balance == 0 {
		<Pallet::<Test> as Currency<AccountId>>::resolve_creating(&account, issue);
	} else {
		<Pallet::<Test> as Currency<AccountId>>::resolve_into_existing(&account, issue).unwrap();
	}
	prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::total_balance(&account), balance + added);
	prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account), balance + added );

	assert_issuance!(balance + added);

	prop_assert!(!<Pallet::<Test> as Currency<AccountId>>::ensure_can_withdraw(&account, balance + added, WithdrawReasons::TRANSFER, 0).is_err());
	prop_assert!( <Pallet::<Test> as Currency<AccountId>>::withdraw(&account, balance + added, WithdrawReasons::TRANSFER, ExistenceRequirement::KeepAlive).is_err());
	prop_assert!(<Pallet::<Test> as Currency<AccountId>>::withdraw(&account, balance + added,WithdrawReasons::TRANSFER, ExistenceRequirement::AllowDeath).is_ok() 				);
	<Pallet::<Test> as Currency<AccountId>>::make_free_balance_be(&account, first);
	prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::total_balance(&account), first);
	prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account), first);

	assert_issuance!(first);

	let burned = <Pallet::<Test> as Currency<AccountId>>::burn(second);
	let diff = <Pallet::<Test> as Currency<AccountId>>::settle(&account, burned,WithdrawReasons::all(), ExistenceRequirement::AllowDeath);

	if second > first {
		prop_assert!(diff.is_err());
	} else {
		prop_assert_ok!(diff);
	}

	<Pallet::<Test> as Currency<AccountId>>::make_free_balance_be(&account, third);
	prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::total_balance(&account), third);
	prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account), third);

	let receiver = &(account + 1);
	prop_assert_ok!(<Pallet::<Test> as Currency<AccountId>>::transfer(&account, receiver, third,
	ExistenceRequirement::AllowDeath));
	prop_assert_eq!(<Pallet::<Test> as
	Currency<AccountId>>::total_balance(receiver), third);
	prop_assert_eq!(<Pallet::<Test> as
	Currency<AccountId>>::free_balance(receiver), third);
		Ok(())
		}).unwrap();
		}
	}
}

mod reservable_currency {
	use super::*;
	use frame_support::traits::tokens::{
		currency::{Currency, ReservableCurrency},
		BalanceStatus, Imbalance,
	};

	macro_rules! assert_issuance {
		($val:expr) => {
			let issuance = BALANCES.iter().fold(0, |sum, (_, val)| val + sum);
			prop_assert_eq!(
				<Pallet::<Test> as Currency<AccountId>>::total_issuance(),
				issuance + $val
			);
		};
	}

	proptest! {
		#![proptest_config(ProptestConfig::with_cases(10000))]

		#[test]
		fn test_can_reserve_implementation(
			account_1 in accounts(),
			(first, _, _) in valid_amounts_without_overflow_3()
		) {
			new_test_ext().execute_with(|| {

				prop_assert_eq!(<Pallet::<Test> as ReservableCurrency<AccountId>>::can_reserve(&account_1, first), false);
				assert_issuance!(0);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_creating(&account_1, first).peek(), first);
				assert_issuance!(first);
				prop_assert_eq!(<Pallet::<Test> as ReservableCurrency<AccountId>>::can_reserve(&account_1, first), true);

				Ok(())
			}).unwrap();
		}

		#[test]
		fn test_reserve_implementation(
			account_1 in accounts(),
			(first, second, third) in valid_amounts_without_overflow_3()
		) {
			new_test_ext().execute_with(|| {

				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_creating(&account_1, first).peek(), first);
				assert_issuance!(first);
				//increase user balance
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_into_existing(&account_1, second).unwrap().peek(), second);
				assert_issuance!(first+second);
				//increase user balance
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_into_existing(&account_1, third).unwrap().peek(), third);
				assert_issuance!(first+second+third);
				//reserve
				prop_assert_ok!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserve(&account_1, first+second));
				prop_assert_eq!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserved_balance(&account_1), first+second);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account_1), third);
				Ok(())
			}).unwrap();
		}

		#[test]
		fn test_slash_reserve_implementation(
			account_1 in accounts(),
			(first, second, third) in valid_amounts_without_overflow_3()
		) {
			new_test_ext().execute_with(|| {

				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_creating(&account_1, first+second+third).peek(), first+second+third);
				//reserve
				prop_assert_ok!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserve(&account_1, first+second));
				let total_issuance = <Pallet::<Test> as Currency<AccountId>>::total_issuance();
				//slash
				let (_, difference) = <Pallet::<Test> as ReservableCurrency<AccountId>>::slash_reserved(&account_1, third);
				let _balance = if  first + second > third {
					prop_assert_eq!(difference, 0);
					let balance = (first + second) - third;
					// check reserve balance after slash
					prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::total_issuance(),total_issuance-(third-difference));
					prop_assert_eq!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserved_balance(&account_1), balance);

					balance
				} else {
					prop_assert_eq!(difference, third - (first + second ));
					prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::total_issuance(),total_issuance-(third-difference));
					prop_assert_eq!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserved_balance(&account_1), 0);
					0
				};

				Ok(())
			}).unwrap();
		}

		#[test]
		fn test_repariate_reserve_implementation(
			(account_1, account_2) in accounts_2(),
			(first, second, third) in valid_amounts_without_overflow_3()
		) {
			new_test_ext().execute_with(|| {

				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_creating(&account_1, first + second + third).peek(), first + second + third);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_creating(&account_2, first).peek(), first);
				prop_assert_ok!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserve(&account_1, first+second+third));
				//repatriate to free balance
				let repatriate_free = <Pallet::<Test> as ReservableCurrency<AccountId>>::repatriate_reserved(&account_1, &account_2, second, BalanceStatus::Free).unwrap();
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account_2),first + (second - repatriate_free));
				//repatriate to reserved balance
				let repatriate_reserved = <Pallet::<Test> as ReservableCurrency<AccountId>>::repatriate_reserved(&account_1, &account_2, third, BalanceStatus::Reserved).unwrap();
				prop_assert_eq!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserved_balance(&account_2), third - repatriate_reserved);
				prop_assert_eq!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserved_balance(&account_1), first + (repatriate_free + repatriate_reserved));

				Ok(())
			}).unwrap();
		}


		#[test]
		fn test_unreserve_implementation(
			account_1 in accounts(),
			(first, second, third) in valid_amounts_without_overflow_3()
		) {
			new_test_ext().execute_with(|| {

				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_creating(&account_1, first + second + third).peek(), first + second + third);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account_1), first + second + third);
				prop_assert_ok!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserve(&account_1, first+second+third));
				prop_assert_eq!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserved_balance(&account_1), first + second + third);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account_1), 0);
				//repatriate to free balance
				let mut remaining = <Pallet::<Test> as ReservableCurrency<AccountId>>::unreserve(&account_1, third);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account_1), third - remaining);
				let mut free_balance = <Pallet::<Test> as Currency<AccountId>>::free_balance(&account_1);
				remaining = <Pallet::<Test> as ReservableCurrency<AccountId>>::unreserve(&account_1, second);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account_1), free_balance + (second - remaining));

				free_balance = <Pallet::<Test> as Currency<AccountId>>::free_balance(&account_1);
				remaining = <Pallet::<Test> as ReservableCurrency<AccountId>>::unreserve(&account_1, first);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account_1), free_balance + (first - remaining));
				prop_assert_eq!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserved_balance(&account_1), 0);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account_1), first + second + third);

				Ok(())
			}).unwrap();
		}
	}
}

/*
type PositiveImbalance: Imbalance<Self::Balance, Opposite = Self::NegativeImbalance>
The opaque token type for an imbalance. This is returned by unbalanced operations and must be dealt with. It may be dropped but cannot be cloned.

[src]
type NegativeImbalance: Imbalance<Self::Balance, Opposite = Self::PositiveImbalance>
The opaque token type for an imbalance. This is returned by unbalanced operations and must be dealt with. It may be dropped but cannot be cloned.
*/
