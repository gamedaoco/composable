use codec::Codec;
use crowdloan_rewards_runtime_api::CrowdloanRewardsRuntimeApi;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result as RpcResult};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;

#[rpc]
pub trait CrowdloanRewardsApi<BlockHash, AccountId, Balance> {
	#[rpc(name = "crowdloanRewards_isClaimAvailableFor")]
	fn is_claim_available_for(&self, at: Option<BlockHash>, account: AccountId) -> RpcResult<bool>;

	#[rpc(name = "crowdloanRewards_claimAmountFor")]
	fn claim_amount_for(
		&self,
		at: Option<BlockHash>,
		account: AccountId,
	) -> RpcResult<Option<Balance>>;
}

/// A struct that implements the `CrowdloanRewardsApi`.
pub struct CrowdloanRewards<C, Block> {
	// If you have more generics, no need to Assets<C, M, N, P, ...>
	// just use a tuple like Assets<C, (M, N, P, ...)>
	client: Arc<C>,
	_marker: std::marker::PhantomData<Block>,
}

impl<C, M> CrowdloanRewards<C, M> {
	/// Create new `SumStorage` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block, AccountId, Balance> CrowdloanRewardsApi<<Block as BlockT>::Hash, AccountId, Balance>
	for CrowdloanRewards<C, (Block, AccountId, Balance)>
where
	Block: BlockT,
	AccountId: Codec + Send + Sync + 'static,
	Balance: Codec + Send + Sync + 'static,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: CrowdloanRewardsRuntimeApi<Block, AccountId, Balance>,
{
	fn is_claim_available_for(
		&self,
		at: Option<<Block as BlockT>::Hash>,
		account_id: AccountId,
	) -> RpcResult<bool> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| {
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash
		}));

		let runtime_api_result = api.is_claim_available_for(&at, account_id);
		// TODO(benluelo): Review what error message & code to use
		runtime_api_result.map_err(|e| {
			RpcError {
				code: ErrorCode::ServerError(9876), // No real reason for this value
				message: "Something wrong".into(),
				data: Some(format!("{:?}", e).into()),
			}
		})
	}

	fn claim_amount_for(
		&self,
		at: Option<<Block as BlockT>::Hash>,
		account_id: AccountId,
	) -> RpcResult<Option<Balance>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| {
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash
		}));

		let runtime_api_result = api.claim_amount_for(&at, account_id);
		// TODO(benluelo): Review what error message & code to use
		runtime_api_result.map_err(|e| {
			RpcError {
				code: ErrorCode::ServerError(9876), // No real reason for this value
				message: "Something wrong".into(),
				data: Some(format!("{:?}", e).into()),
			}
		})
	}
}
