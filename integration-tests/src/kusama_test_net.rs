//pub use picasso_runtime::AccountId;
///use xcm_emulator::*;
//pub use picasso_runtime::AccountId;

pub const ALICE: [u8; 32] = [4u8; 32];
pub const BOB: [u8; 32] = [5u8; 32];

pub const PICA: u128 = 1_000_000_000_000;

pub fn kusama_ext() -> sp_io::TestExternalities {
	use kusama_runtime::{Runtime, System};
	let mut storage = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
	// pallet_balances::GenesisConfig::<Runtime> {
	// 	balances: vec![
	// 		(ALICE, INITIAL_BALANCE)
	// 		]
	// }
	todo!()
}


// decl_test_relay_chain! {
// 	pub struct KusamaRelay {
// 		Runtime = kusama_runtime::Runtime,
// 		XcmConfig = kusama_runtime::XcmConfig,

// 	}
// }
