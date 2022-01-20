import {IKeyringPair} from "@polkadot/types/types";
import {sendAndWaitForSuccess} from "@composable/utils/polkadotjs";


/**
 * Tests tx.oracle.submitPrice with provided parameters that should succeed.
 * @param {IKeyringPair} sudoKey Connected API Promise w/ sudo rights.
 */
export async function txOracleSubmitPriceSuccessTest(wallet: IKeyringPair, price, assetId) {
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.oracle.PriceSubmitted.is,
    api.tx.oracle.submitPrice(price, assetId),
    false
  );
}