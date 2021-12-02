use crate::{env_logger_init, kusama_test_net::*};
use common::AccountId;
use composable_traits::assets::RemoteAssetRegistry;
use primitives::currency::*;
use support::assert_ok;
use xcm::latest::prelude::*;
use cumulus_primitives_core::ParaId;
use orml_traits::currency::MultiCurrency;
use sp_runtime::traits::AccountIdConversion;

use xcm_emulator::TestExt;
use picasso_runtime as dali_runtime;

/// assumes that our parachain has native relay token on relay account
/// and kusama can send xcm message to our network and transfer native token onto local network
#[test]
fn transfer_from_relay_chain() {
	crate::kusama_test_net::KusamaNetwork::reset();
	env_logger_init();
    Picasso::execute_with(|| {
		assert_ok!(picasso_runtime::AssetsRegistry::set_location(
			CurrencyId::KSM, // KSM id as it is locally
			// if we get tokens from parent chain, these can be only native token
			composable_traits::assets::XcmAssetLocation(MultiLocation::parent())
		));
	});
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
		assert_eq!(
			kusama_runtime::Balances::free_balance(&ParaId::from(PICASSO_PARA_ID).into_account()),
			13 * PICA
		);
    });

    Picasso::execute_with(|| {
        let native_token = picasso_runtime::Assets::free_balance(CurrencyId::KSM, &AccountId::from(BOB));
        assert_eq!(native_token, 3 * PICA);
    });
}

#[test]
fn transfer_insufficient_amount_should_fail() {
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




