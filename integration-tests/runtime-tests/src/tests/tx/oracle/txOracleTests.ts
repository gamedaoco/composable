/* eslint-disable no-trailing-spaces */
import {expect} from "chai";
import {v4} from "uuid";
import {txOracleSubmitPriceSuccessTest} from "@composable/tests/tx/oracle/testHandlers/submitPriceTests";
import {txOracleAddAssetAndInfoSuccessTest} from "@composable/tests/tx/oracle/testHandlers/addAssetAndInfoTests";
import {
  runBeforeTxOracleSetSigner,
  txOracleSetSignerSuccessTest
} from "@composable/tests/tx/oracle/testHandlers/setSignerTests";
import {KeyringPair} from "@polkadot/keyring/types";
import {
  runBeforeTxOracleAddStake,
  txOracleAddStakeSuccessTest
} from "@composable/tests/tx/oracle/testHandlers/addStakeTests";

/**
 * Contains all TX tests for the pallet:
 * bondedFinance
 */
export class TxOracleTests {
  /**
   * Runs all tx tests for the bondedFinance pallet.
   *
   * ToDo (D. Roth): The tests assume you're running them on a fresh chain. Instead of assuming, use the test returns.
   */
  public static runTxOracleTests() {
    let assetsCountStart:number;
    let newAsset1:number;
    let signedWallet:KeyringPair;

    describe('tx.oracle Tests', function () {
      before(async function() {
        assetsCountStart = (await api.query.oracle.assetsCount()).toNumber();
        newAsset1=assetsCountStart+1;
      });
      /**
       * oracle.addAssetAndInfo Success Tests
       */
      describe('tx.addAssetAndInfo Success Test', function () {
        // Timeout set to 2 minutes
        this.timeout(2 * 60 * 1000);
        it('Can add new asset and info', async function () {
          const assetId = api.createType('u128', newAsset1);
          const threshold = api.createType('Percent', 50);
          const minAnswers = api.createType('u32', 2);
          const maxAnswers = api.createType('u32', 5);
          const blockInterval = api.createType('u32', 6);
          const reward = api.createType('u128', 150000000000);
          const slash = api.createType('u128', 100000000000);
          const sudoKey = walletAlice;
          const {data: [result],} = await txOracleAddAssetAndInfoSuccessTest(
            sudoKey,
            assetId,
            threshold,
            minAnswers,
            maxAnswers,
            blockInterval,
            reward,
            slash
          );
          if (result.isErr)
            console.debug(result.asErr.toString());
          expect(result.isOk).to.be.true;
        });
      });

      /**
       * oracle.setSigner Success Tests
       *
       * Current Problem:
       * Function works if the chain is clean and the account wasn't signed before, but it still reports the test as a
       * failure.
       * This is probably due to the function receiving multiple Promise resolves, and Mocha reports this as an error.
       * Info: https://mochajs.org/#detects-multiple-calls-to-done
       *
       * ToDo (D. Roth): Fix problem mentioned above.
       */
      describe('tx.setSigner Success Test', function () {
        // Timeout set to 2 minutes
        this.timeout(2 * 60 * 1000);
        let sender;
        before(async function() {
          //this.skip();
          const sudoKey = walletAlice;
          sender = walletAlice; // Is also a sudokey for the run function 2 lines below.
          signedWallet = walletCharlie;
          const {data: [result],} = await runBeforeTxOracleSetSigner(sudoKey, signedWallet);
          expect(result.isOk).to.be.true;
        });
        it('Can set signer', async function () {
          const {data: [resultAccount0, resultAccount1],} = await txOracleSetSignerSuccessTest(sender, signedWallet).catch(function(exc) {
            return {data:[exc]}; // We can't call this.skip() from here.
          });
          if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use" ||
              resultAccount0.message == "oracle.ControllerUsed: This controller is already in use")
            return this.skip();
          expect(resultAccount0).to.not.be.an('Error');
          // ToDo (D. Roth): Add more checks
        });
      });

      /**
       * oracle.addStake Success Tests
       */
      describe('tx.addStake Success Test', function () {
        let wallet;
        // Timeout set to 2 minutes
        this.timeout(2 * 60 * 1000);
        before(async function() {
          const sudoKey = walletAlice;
          wallet = walletAlice;
          const {data: [result],} = await runBeforeTxOracleAddStake(sudoKey, wallet);
          expect(result.isOk).to.be.true;
        });
        it('Can add stake', async function () {
          const stake = api.createType('u128', 250000000000);
          const {data: [result],} = await txOracleAddStakeSuccessTest(wallet, stake);
          expect(result).to.not.be.an('Error');
          // ToDo (D. Roth): Add more checks
        });
      });

      /**
       * oracle.submitPrice Success Tests
       */
      describe('tx.submitPrice Success Test', function () {
        // Timeout set to 2 minutes
        this.timeout(2 * 60 * 1000);
        it('Can submit new price', async function () {
          const price = api.createType('u128', 10000);
          const assetId = api.createType('u128', newAsset1);
          const {data: [result],} = await txOracleSubmitPriceSuccessTest(signedWallet, price, assetId);
          expect(result).to.not.be.an('Error');
          // ToDo (D. Roth): Add more checks
        });
      });
    });
  }
}

// Uncomment to debug
//TxBondedFinanceTests.runTxBondedFinanceTests();
