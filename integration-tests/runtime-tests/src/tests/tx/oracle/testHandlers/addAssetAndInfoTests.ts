import {IKeyringPair} from "@polkadot/types/types";
import {sendAndWaitForSuccess} from "@composable/utils/polkadotjs";


/**
 * Tests tx.oracle.addAssetAndInfo with provided parameters that should succeed.
 * @param {IKeyringPair} sudoKey Connected API Promise w/ sudo rights.
 */
export async function txOracleAddAssetAndInfoSuccessTest(
  sudoKey: IKeyringPair,
  assetId,
  threshold,
  minAnswers,
  maxAnswers,
  blockInterval,
  reward,
  slash
) {
  return await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(
      api.tx.oracle.addAssetAndInfo(
        assetId,
        threshold,
        minAnswers,
        maxAnswers,
        blockInterval,
        reward,
        slash
      ),
    )
  );
}