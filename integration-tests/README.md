# Overview

Runs transfers from some Composable based parachain to Composable parachain. And other parachains integrations.

```
RUST_LOG=runtime=trace,substrate-relay=trace,bridge=trace cargo ltest cross_chain_transfer::transfer_from_dali -- --nocapture
```