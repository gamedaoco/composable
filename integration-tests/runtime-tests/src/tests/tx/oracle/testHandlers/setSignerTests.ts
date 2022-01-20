import {IKeyringPair} from "@polkadot/types/types";
import {sendAndWaitForSuccess} from "@composable/utils/polkadotjs";


export async function runBeforeTxOracleSetSigner(sudoKey, signer) {
  return await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(
      api.tx.assets.mintInto(1, signer.publicKey, 555555555555)
    )
  );
}

/**
 * Tests tx.oracle.submitPrice with provided parameters that should succeed.
 * @param {IKeyringPair} sudoKey Connected API Promise w/ sudo rights.
 */
export async function txOracleSetSignerSuccessTest(sender, signer) {
  return await sendAndWaitForSuccess(
    api,
    sender,
    api.events.oracle.SignerSet.is,
    api.tx.oracle.setSigner(signer.publicKey),
    false
  );
}