/* eslint-disable no-var */
import '@composable/types/interfaces/augment-api';
import '@composable/types/interfaces/augment-types';
import * as definitions from '@composable/types/interfaces/definitions';

import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import Web3 from 'web3';
import { args } from "./args";

global.useTestnetWallets = true;
global.testSudoCommands = true;
// ToDo (D. Roth): Read public/private keys from external file to be usable in live environment.
//       and ability to specify keys using env variables or using run parameters.

exports.mochaHooks = {
  beforeAll: [async() => {
    // extract all types from definitions - fast and dirty approach, flatted on 'types'
    const types = Object.values(definitions).reduce((res, { types }): object => ({ ...res, ...types }), {});
    const rpc = Object.values(definitions).reduce((res, { rpc }): object => ({ ...res, ...rpc, }), {});


    global.endpoint = `ws://${args.h}:${args.p}`;
    const provider = new WsProvider(global.endpoint);
    console.debug(`Establishing connection to ${global.endpoint}...`);
    const apiOptions = {
      provider, types,
      rpc: {
        crowdloanRewards: {
          amountAvailableToClaimFor: {
            description: "The unclaimed amount",
            params: [
              {
                name: "at",
                type: "Option<Hash>",
              },
              {
                name: "account",
                type: "PalletCrowdloanRewardsModelsRemoteAccount"
              },
            ],
            type: "Balance"
          },
        },
      },
    };
    console.log(apiOptions);
    global.api = await ApiPromise.create(apiOptions);

    global.web3 = new Web3();

    // do something before every test,
    // then run the next hook in this array
    global.keyring = new Keyring({type: 'sr25519'});

    if (global.useTestnetWallets === true) {
      global.walletAlice = global.keyring.addFromUri('//Alice');
      global.walletBob = global.keyring.addFromUri('//Bob');
      global.walletCharlie = global.keyring.addFromUri('//Charlie');
      global.walletDave = global.keyring.addFromUri('//Dave');
      global.walletEve = global.keyring.addFromUri('//Eve');
      global.walletFerdie = global.keyring.addFromUri('//Ferdie');
    }
    return;
  }],
  afterAll: [async() => {
    global.api.disconnect();
    process.exit(0);
  }]
}
