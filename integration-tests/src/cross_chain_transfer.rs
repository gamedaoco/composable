use crate::kusama_test_net::*;
use common::AccountId;
use kusama_runtime::*;
use primitives::currency::CurrencyId;
use support::assert_ok;
use xcm::latest::prelude::*;

use cumulus_primitives_core::ParaId;
use orml_traits::currency::MultiCurrency;
use sp_runtime::traits::AccountIdConversion;
use xcm_simulator::TestExt;

#[test]
fn transfer_from_relay_chain() {
    // here we should add asset to registry
    KusamaRelay::execute_with(|| {
        // let version = kusama_runtime::XcmPallet::force_default_xcm_version
        // (
        //     kusama_runtime::Origin::root(),
        //     Some(1),
        // );
        // assert_ok!(version);

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
            let transfered = picasso_runtime::XTokens::transfer(
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
            assert_ok!(transfered);
    });
}





