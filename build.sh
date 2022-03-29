#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo +stable build --all --target wasm32-unknown-unknown --release
cp -u target/wasm32-unknown-unknown/release/nft-loans.wasm res/
