//! Test that the runtime is config is good and secured, no sending XCM

use crate::{env_logger_init, kusama_test_net::*};
use common::AccountId;
use composable_traits::assets::RemoteAssetRegistry;
use kusama_runtime::*;
use primitives::currency::CurrencyId;
use support::assert_ok;
use xcm::latest::prelude::*;
use codec::Encode;
use cumulus_primitives_core::{ChannelStatus, GetChannelInfo, ParaId};
use orml_traits::currency::MultiCurrency;
use sp_runtime::traits::AccountIdConversion;
use xcm_executor::XcmExecutor;
//use crate::xcm_simulator::TestExt;
use xcm_emulator::TestExt;
use picasso_runtime as dali_runtime;
use kusama_runtime::KusamaNetwork as KusamaNetworkId;
use crate::kusama_test_net::KusamaNetwork;

//use serial_test::serial;

//#[serial]
#[test]
fn channel_to_relay() {
    env_logger_init();
    KusamaNetwork::reset();
    Picasso::execute_with(|| {
        let status = <picasso_runtime::ParachainSystem as GetChannelInfo>::get_channel_status(ParaId::new(0));
    });
    Picasso::execute_with(|| {
        let status = <picasso_runtime::ParachainSystem as GetChannelInfo>::get_channel_status(ParaId::new(0));
        assert!(matches!(status, ChannelStatus::Closed));
    });
}

//#[serial]
#[test]
fn channel_to_parachain() {
    env_logger_init();
    KusamaNetwork::reset();
    Picasso::execute_with(|| {
        let status = <picasso_runtime::ParachainSystem as GetChannelInfo>::get_channel_status(ParaId::new(DALI_PARA_ID));
    });
    Picasso::execute_with(|| {
        let status = <picasso_runtime::ParachainSystem as GetChannelInfo>::get_channel_status(ParaId::new(DALI_PARA_ID));

        assert!(matches!(status, ChannelStatus
            ::Closed));
    });
}