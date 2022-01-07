use crate::runtime_api::AssetsApi as AssetsRuntimeApi;
use codec::Codec;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result as RpcResult};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;

#[rpc]
pub trait AssetsApi<BlockHash, AssetId, AccountId, Balance> {
	#[rpc(name = "assets_balanceOf")] // , returns = "Balance"
	fn balance_of(
		&self,
		at: Option<BlockHash>,
		currency: AssetId,
		account: AccountId,
	) -> RpcResult<Option<Balance>>;
}

/// A struct that implements the `SumStorageApi`.
pub struct Assets<C, Block> {
	// If you have more generics, no need to SumStorage<C, M, N, P, ...>
	// just use a tuple like SumStorage<C, (M, N, P, ...)>
	client: Arc<C>,
	_marker: std::marker::PhantomData<Block>,
}

impl<C, M> Assets<C, M> {
	/// Create new `SumStorage` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// Error type of this RPC api.
// pub enum Error {
// 	/// The transaction was not decodable.
// 	DecodeError,
// 	/// The call to runtime failed.
// 	RuntimeError,
// }
//
// impl From<Error> for i64 {
// 	fn from(e: Error) -> i64 {
// 		match e {
// 			Error::RuntimeError => 1,
// 			Error::DecodeError => 2,
// 		}
// 	}
// }

impl<C, Block, AssetId, AccountId, Balance>
	AssetsApi<<Block as BlockT>::Hash, AssetId, AccountId, Balance>
	for Assets<C, (Block, AssetId, AccountId, Balance)>
where
	Block: BlockT,
	AssetId: Codec + Send + Sync + 'static,
	AccountId: Codec + Send + Sync + 'static,
	Balance: Codec + Send + Sync + 'static,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: AssetsRuntimeApi<Block, AssetId, AccountId, Balance>,
{
	fn balance_of(
		&self,
		at: Option<<Block as BlockT>::Hash>,
		asset_id: AssetId,
		account_id: AccountId,
	) -> RpcResult<Option<Balance>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| {
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash
		}));

		let runtime_api_result = api.balance_of(&at, asset_id, account_id);
		// TODO(benluelo): Review what error message & code to use
		runtime_api_result.map_err(|e| {
			RpcError {
				code: ErrorCode::ServerError(9876), // No real reason for this value
				message: "Something wrong".into(),
				data: Some(format!("{:?}", e).into()),
			}
		})
	}

	// fn get_sum(&self, at: Option<<Block as BlockT>::Hash>) -> Result<u32> {
	// 	let api = self.client.runtime_api();
	// 	let at = BlockId::hash(at.unwrap_or_else(||
	// 		// If the block hash is not supplied assume the best block.
	// 		self.client.info().best_hash));

	// 	let runtime_api_result = api.get_sum(&at);
	// 	runtime_api_result.map_err(|e| RpcError {
	// 		code: ErrorCode::ServerError(9876), // No real reason for this value
	// 		message: "Something wrong".into(),
	// 		data: Some(format!("{:?}", e).into()),
	// 	})
	// }
}
