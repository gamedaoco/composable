/* eslint-disable no-trailing-spaces */
import { ApiPromise } from '@polkadot/api';
import { expect } from 'chai';


export class RpcCrowdloanRewardsTests {
  /**
   * 
   */
  public static runRpcCrowdloanRewardsTests() {
    describe('query.crowdloanRewards.account Tests', function() {
      it('STUB', async () => {
        await RpcCrowdloanRewardsTests.rpcCrowdloanRewardsTest();
      });
    });
  }

  /**
   * 
   */
  private static async rpcCrowdloanRewardsTest() {
    // ToDo (D. Roth): STUB
    const x = await api.rpc.crowdloanRewards.amountAvailableToClaimFor(null,
      api.createType(
        'PalletCrowdloanRewardsModelsRemoteAccount',
        {
          RelayChain: keyring.decodeAddress("5zCSgFGFyfADTNriuoZvRNVByYK4S2ebiioFPCzoCmsbB3WY")
        }));
    expect(x).to.equal(1);
  }
}

// Uncomment to debug
// RpcCrowdloanRewardsTests.runRpcCrowdloanRewardsTests();
