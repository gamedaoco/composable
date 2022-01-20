import {IKeyringPair} from "@polkadot/types/types";
import {sendAndWaitForSuccess} from "@composable/utils/polkadotjs";
import {u128} from "@polkadot/types-codec";


export async function runBeforeTxOracleAddStake(sudoKey, targetWallet) {
  return await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(
      api.tx.assets.mintInto(1, targetWallet.publicKey, 555555555555)
    )
  );
}

/**
 * Tests tx.oracle.submitPrice with provided parameters that should succeed.
 * @param {IKeyringPair} sudoKey Connected API Promise w/ sudo rights.
 */
export async function txOracleAddStakeSuccessTest(sender, stake:u128) {
  return await sendAndWaitForSuccess(
    api,
    sender,
    api.events.oracle.StakeAdded.is,
    api.tx.oracle.addStake(stake),
    false
  );
}