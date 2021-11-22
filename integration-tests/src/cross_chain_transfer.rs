use crate::kusama_test_net::*;
use common::AccountId;
use composable_traits::assets::RemoteAssetRegistry;
use kusama_runtime::*;
use primitives::currency::CurrencyId;
use support::assert_ok;
use xcm::latest::prelude::*;

use cumulus_primitives_core::ParaId;
use orml_traits::currency::MultiCurrency;
use sp_runtime::traits::AccountIdConversion;
use xcm_simulator::TestExt;
use picasso_runtime as dali_runtime;

#[test]
fn transfer_from_relay_chain() {
    // here we should add asset to registry
    KusamaRelay::execute_with(|| {
        let transfered = kusama_runtime::XcmPallet::reserve_transfer_assets(
            kusama_runtime::Origin::signed(ALICE.into()),
            Box::new(Parachain(PICASSO_PARA_ID).into().into()),
            Box::new(
                Junction::AccountId32 {
                    id: crate::kusama_test_net::BOB,
                    network : NetworkId::Any,
                }.into().into(),
            ),
            Box::new((Here, 3 * PICA).into()),
            0,
        );
        assert_ok!(transfered);
    });

    Picasso::execute_with(|| {

        let balance = picasso_runtime::Tokens::free_balance(CurrencyId::PICA, &AccountId::from(BOB));
        assert_eq!(balance, 3 * PICA);
    });
}

#[test]
fn transfer_to_relay_chain() {
    Picasso::execute_with(|| {
		// assert_ok!(<picasso_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
		// 	CurrencyId::PICA,
		// 	composable_traits::assets::XcmAssetLocation(MultiLocation::parent()),
		// ));
            let transferred = picasso_runtime::XTokens::transfer(
                picasso_runtime::Origin::signed(ALICE.into()),
                CurrencyId::PICA,
                3 * PICA,
                Box::new(
                    MultiLocation::new(
                        1,
                        X1(Junction::AccountId32 {
                            id : BOB,
                            network: NetworkId::Any,
                        })
                    ).into()
                ),
                4_600_000_000);

				assert_ok!(transferred);

            let remaining = picasso_runtime::Tokens::free_balance(
                CurrencyId::PICA, &AccountId::from(ALICE));

            assert_eq!(remaining, 200 * PICA - 3 * PICA);
    });

    KusamaRelay::execute_with(|| {
		assert_eq!(
			kusama_runtime::Balances::free_balance(&AccountId::from(BOB)),
			2999893333340 // 3 * PICA - fee
		);
	});
}


#[test]
fn transfer_from_picasso_to_dali() {
	Picasso::execute_with(|| {
		assert_ok!(<picasso_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
			CurrencyId::PICA,
			composable_traits::assets::XcmAssetLocation(MultiLocation::new(1, X2(Parachain(DALI_PARA_ID), CurrencyId::PICA.into())))
		));

		assert_ok!(picasso_runtime::XTokens::transfer(
			picasso_runtime::Origin::signed(ALICE.into()),
			CurrencyId::PICA,
			3 * PICA,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Junction::Parachain(DALI_PARA_ID),
						Junction::AccountId32 {
							id: BOB,
							network: NetworkId::Any,
						}
					)
				)
				.into()
			),
			399_600_000_000
		));
		assert_eq!(
			picasso_runtime::Balances::free_balance(&AccountId::from(ALICE)),
			200 * PICA - 3 * PICA
		);
	});

	// Picasso::execute_with(|| {
	// 	assert_eq!(
	// 		picasso_runtime::Tokens::free_balance(CurrencyId::PICA, &AccountId::from(BOB)),
	// 		3 * PICA
	// 	);
	// });
}

#[test]
fn transfer_from_dali() {
	Picasso::execute_with(|| {
		assert_ok!(<picasso_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
			CurrencyId::PICA,
			composable_traits::assets::XcmAssetLocation(MultiLocation::new(1, X2(Parachain(DALI_PARA_ID), CurrencyId::PICA.into())))
		));
		assert_ok!(<picasso_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
			CurrencyId::PICA,
			composable_traits::assets::XcmAssetLocation(MultiLocation::new(1, X2(Parachain(PICASSO_PARA_ID), CurrencyId::PICA.into())))
		));
	});

	Dali::execute_with(|| {
		assert_ok!(<dali_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
			CurrencyId::PICA,
			composable_traits::assets::XcmAssetLocation(MultiLocation::new(1, X2(Parachain(DALI_PARA_ID), CurrencyId::PICA.into())))
		));
		assert_ok!(<dali_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
			CurrencyId::PICA,
			composable_traits::assets::XcmAssetLocation(MultiLocation::new(1, X2(Parachain(PICASSO_PARA_ID), CurrencyId::PICA.into())))
		));
	});


	Dali::execute_with(|| {
		assert_ok!(dali_runtime::XTokens::transfer(
			dali_runtime::Origin::signed(ALICE.into()),
			CurrencyId::PICA,
			3 * PICA,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Junction::Parachain(PICASSO_PARA_ID),
						Junction::AccountId32 {
							id: BOB,
							network: NetworkId::Any,
						}
					)
				)
				.into()
			),
			399_600_000_000
		));
		assert_eq!(
			dali_runtime::Balances::free_balance(&AccountId::from(ALICE)),
			200 * PICA - 3 * PICA
		);
	});

	Picasso::execute_with(|| {
		assert_eq!(
			picasso_runtime::Tokens::free_balance(CurrencyId::PICA, &AccountId::from(BOB)),
			3 * PICA
		);
	});
}


#[test]
fn transfer_insufficient_amount_should_fail() {

	// Picasso::execute_with(|| {
	// 	assert_ok!(picasso_runtime::AssetRegistry::set_location(
	// 		picasso_runtime::Origin::root(),
	// 		1,
	// 		picasso_runtime::AssetLocation(MultiLocation::new(1, X2(Parachain(3000), GeneralKey(vec![0, 0, 0, 0]))))
	// 	));
	// });

	Dali::execute_with(|| {
		assert_ok!(dali_runtime::XTokens::transfer(
			dali_runtime::Origin::signed(ALICE.into()),
			CurrencyId::PICA,
			1_000_000 - 1,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Junction::Parachain(2000),
						Junction::AccountId32 {
							id: BOB,
							network: NetworkId::Any,
						}
					)
				)
				.into()
			),
			399_600_000_000
		));
		assert_eq!(
			dali_runtime::Balances::free_balance(&AccountId::from(ALICE)),
			199999999000001
		);
	});

	Picasso::execute_with(|| {
		// Xcm should fail therefore nothing should be deposit into beneficiary account
		assert_eq!(picasso_runtime::Tokens::free_balance(CurrencyId::PICA, &AccountId::from(BOB)), 0);
	});
}




