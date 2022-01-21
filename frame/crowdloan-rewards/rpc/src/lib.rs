use codec::Codec;
use crowdloan_rewards_runtime_api::CrowdloanRewardsRuntimeApi;
use frame_support::{pallet_prelude::MaybeSerializeDeserialize, Parameter};
use jsonrpc_core::{Error as RpcError, ErrorCode, Result as RpcResult};
use jsonrpc_derive::rpc;
use num_traits::{CheckedAdd, CheckedMul, CheckedSub, Zero};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::BlockId,
	traits::{AtLeast32BitUnsigned, Block as BlockT},
	AccountId32,
};
use std::sync::Arc;

#[rpc]
pub trait CrowdloanRewardsApi<BlockHash, RelayChainAccountId, Balance> {
	#[rpc(name = "crowdloanRewards_amountAvailableToClaimFor")]
	fn amount_available_to_claim_for(
		&self,
		at: Option<BlockHash>,
		account: pallet_crowdloan_rewards::models::RemoteAccount<RelayChainAccountId>,
	) -> RpcResult<Balance>;
}

/// A struct that implements the `CrowdloanRewardsApi`.
pub struct CrowdloanRewards<C, Block> {
	// If you have more generics, no need to Assets<C, M, N, P, ...>
	// just use a tuple like Assets<C, (M, N, P, ...)>
	client: Arc<C>,
	_marker: std::marker::PhantomData<Block>,
}

impl<C, M> CrowdloanRewards<C, M> {
	/// Create new `CrowdloanRewards` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block, RelayChainAccountId, Balance>
	CrowdloanRewardsApi<<Block as BlockT>::Hash, RelayChainAccountId, Balance>
	for CrowdloanRewards<C, (Block, RelayChainAccountId, Balance)>
where
	Block: BlockT,
	RelayChainAccountId:
		Send + Sync + Parameter + MaybeSerializeDeserialize + Into<AccountId32> + Ord + 'static,
	Balance: Send
		+ Sync
		+ Default
		+ Parameter
		+ Codec
		+ Copy
		+ Ord
		+ CheckedAdd
		+ CheckedSub
		+ CheckedMul
		+ AtLeast32BitUnsigned
		+ MaybeSerializeDeserialize
		+ Zero
		+ 'static,
	C: Send + Sync + ProvideRuntimeApi<Block> + HeaderBackend<Block> + 'static,
	C::Api: CrowdloanRewardsRuntimeApi<Block, RelayChainAccountId, Balance>,
{
	fn amount_available_to_claim_for(
		&self,
		at: Option<<Block as BlockT>::Hash>,
		remote_account: pallet_crowdloan_rewards::models::RemoteAccount<RelayChainAccountId>,
	) -> RpcResult<Balance> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| {
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash
		}));

		let runtime_api_result = api.amount_available_to_claim_for(&at, remote_account);
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
