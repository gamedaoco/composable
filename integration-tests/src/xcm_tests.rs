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

        let executed = XcmExecutor::<XcmConfig>::execute_xcm_in_credit(here, xcm, 42, 42);

        assert!(matches!(Outcome::Error(xcm::latest::Error::Trap(42)), executed));
    });
}