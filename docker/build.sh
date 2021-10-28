#!/usr/bin/env bash

wget https://storage.googleapis.com/composable-binaries/releases/client/v0.1.0/target.zip 
unzip target.zip
./target/release/parachain-utils upgrade-runtime --path ./target/release/wbuild/picasso-runtime/picasso_runtime.compact.wasm