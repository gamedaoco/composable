//! basic XCM `code katas`, example


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
use xcm_executor::XcmExecutor;
//use crate::xcm_simulator::TestExt;
use xcm_simulator::TestExt;
use picasso_runtime as dali_runtime;


/// as per documentation is way to throw exception with specific error code is Trap(1)
#[test]
fn throw_exception() {
    Picasso::execute_with(|| {
        let here = MultiLocation::new(0, Here);
        let xcm = Xcm(vec![Trap(42)]);

        let executed = XcmExecutor::<XcmConfig>::execute_xcm_in_credit(here, xcm, 1000000000, 1000000000);

        match executed {
            Outcome::Incomplete(_, error) => assert_eq!(XcmError::Trap(42), error),
            _ => unreachable!(),
        }
    });
}

#[test]
fn teleport_all() {
    env_logger::init();
    Picasso::execute_with(|| {
        let here = MultiLocation::new(0, Here);

        let xcm = Xcm(vec![InitiateTeleport {
            assets: All.into(),
            dest: Parent.into(),
            xcm: Xcm(vec![
                // BuyExecution {
                //     fees: (Parent, 10000000).into(),
                //     weight_limit: WeightLimit::Limited(100000000),
                // },
                // Trap(13),
            ]),
        }]);
        let executed = XcmExecutor::<XcmConfig>::execute_xcm_in_credit(here, xcm,
            1000000000, 1000000000);


        //assert!(matches!(Outcome::Error(xcm::latest::Error::Trap(42)), executed));
        dbg!("{:?}", executed);
        assert!(matches!(Outcome::Complete(42), executed));

    });
}